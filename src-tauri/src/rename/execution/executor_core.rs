// 执行器核心模块
// 职责：协调批量重命名执行
// 不做单文件重命名细节，不做统计聚合

use super::execution_mode::ExecutionMode;
use super::single_rename::execute_single_rename;
use crate::audit::db::{self, RenameResult, TaskStatus};
use crate::rename::conflict_detector;
use crate::rename::safety_checker;
use crate::rename::template::RenamePreviewItem;
use crate::shared::result_types::{AppError, AppResult};
use rusqlite::Connection;

/// 执行重命名任务
///
/// 1. 运行安全检查
/// 2. 根据模式决定是否执行真实 rename
/// 3. 返回每条结果（成功/失败）
pub fn execute(
    _conn: &Connection,
    task_id: &str,
    preview_items: &[RenamePreviewItem],
    mode: ExecutionMode,
) -> AppResult<Vec<RenameResult>> {
    // 1. 安全检查
    let report = safety_checker::check_all(preview_items);

    // Confirmed 模式必须通过安全检查
    if mode == ExecutionMode::Confirmed && !report.can_execute {
        return Err(AppError::SafetyCheckFailed {
            check_name: "all".to_string(),
            reason: report.blocking_reasons.join("; "),
        });
    }

    // 2. 过滤跳过项
    let actionable_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| !item.should_skip)
        .collect();

    // 3. 过滤有阻塞冲突的项
    let non_blocking_items: Vec<&&RenamePreviewItem> = actionable_items
        .iter()
        .filter(|item| !conflict_detector::has_blocking_conflicts(&item.conflicts))
        .collect();

    let mut results = Vec::new();

    // 4. 处理被跳过 + 有冲突的项，记录为 Failed
    for item in &actionable_items {
        if conflict_detector::has_blocking_conflicts(&item.conflicts) {
            let result = db::create_result(
                task_id,
                &item.source_path,
                &item.target_path,
                TaskStatus::Failed,
            );
            // 手动设置 error_message
            let mut result = result;
            result.error_message = Some(format!(
                "Blocking conflicts: {}",
                item.conflicts
                    .iter()
                    .filter(|c| c.blocking)
                    .map(|c| c.message.clone())
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
            results.push(result);
        }
    }

    // 5. 执行非阻塞项
    for item in non_blocking_items {
        let result = match mode {
            ExecutionMode::DryRun => {
                // DryRun: 不执行文件操作，返回模拟成功
                db::create_result(
                    task_id,
                    &item.source_path,
                    &item.target_path,
                    TaskStatus::Pending,
                )
            }
            ExecutionMode::Confirmed => {
                // Confirmed: 执行真实重命名
                execute_single_rename(item, task_id)
            }
        };
        results.push(result);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::{self, init_tables};
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::template::{MetadataSource, RenamePreviewItem};
    use crate::scan::MediaItem;
    use std::fs;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    fn make_preview_item(
        source: &str,
        target: &str,
        confidence: u8,
        needs_review: bool,
    ) -> RenamePreviewItem {
        RenamePreviewItem {
            id: Uuid::new_v4().to_string(),
            parsed_info: ParsedMediaInfo {
                media_item: MediaItem {
                    id: "test".to_string(),
                    file_path: source.to_string(),
                    file_name: std::path::Path::new(source)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: MediaType::Movie,
                title: "Test".to_string(),
                year: Some(2020),
                season: None,
                episode: None,
                episode_end: None,
                episode_title: None,
                resolution: Some("1080p".to_string()),
                source: None,
                video_codec: None,
                audio_codec: None,
                group: None,
                confidence,
                rule_sources: vec![],
                special_type: None,
                extra_type: None,
                extra_number: None,
            },
            source_path: source.to_string(),
            original_name: std::path::Path::new(source)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            proposed_name: std::path::Path::new(target)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            target_path: target.to_string(),
            media_type: MediaType::Movie,
            confidence,
            needs_manual_review: needs_review,
            should_skip: needs_review,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    #[test]
    fn test_dry_run_does_not_modify_files() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&source, b"test content").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();

        let item = make_preview_item(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            85,
            false,
        );

        let results = execute(&conn, &task.id, &[item], ExecutionMode::DryRun).unwrap();

        // 文件不应被修改
        assert!(source.exists());
        assert!(!target.exists());
        // 结果应为 Pending（dry-run 模拟成功）
        assert_eq!(results[0].status, TaskStatus::Pending);
    }

    #[test]
    fn test_confirmed_rename_success() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&source, b"test content").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();

        let item = make_preview_item(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            85,
            false,
        );

        let results = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed).unwrap();

        assert!(!source.exists());
        assert!(target.exists());
        assert_eq!(results[0].status, TaskStatus::Completed);
    }

    #[test]
    fn test_source_not_found_fails() {
        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();

        let item = make_preview_item(
            "C:\\nonexistent\\file.mkv",
            "C:\\nonexistent\\renamed.mkv",
            85,
            false,
        );

        let results = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed).unwrap();

        assert_eq!(results[0].status, TaskStatus::Failed);
        assert!(results[0]
            .error_message
            .as_ref()
            .unwrap()
            .contains("not found"));
    }

    #[test]
    fn test_target_already_exists_fails() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.mkv");
        let target = tmp.path().join("existing.mkv");
        fs::write(&source, b"source").unwrap();
        fs::write(&target, b"existing").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();

        let item = make_preview_item(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            85,
            false,
        );

        let results = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed).unwrap();

        assert_eq!(results[0].status, TaskStatus::Failed);
        assert!(results[0]
            .error_message
            .as_ref()
            .unwrap()
            .contains("already exists"));
    }

    #[test]
    fn test_conflict_item_blocked() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("file.mkv");
        let target = tmp.path().join("file.mkv"); // same path = conflict
        fs::write(&source, b"content").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();

        let mut item = make_preview_item(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            85,
            false,
        );
        item.conflicts
            .push(crate::rename::template::RenameConflict {
                conflict_type: crate::shared::result_types::ConflictType::TargetExists,
                source_path: source.to_string_lossy().to_string(),
                target_path: target.to_string_lossy().to_string(),
                message: "Target exists".to_string(),
                blocking: true,
            });

        // DryRun 模式不检查 can_execute，但冲突项会被标记为 Failed
        let results = execute(&conn, &task.id, &[item], ExecutionMode::DryRun).unwrap();

        assert_eq!(results[0].status, TaskStatus::Failed);
        assert!(results[0]
            .error_message
            .as_ref()
            .unwrap()
            .contains("Blocking conflicts"));
    }

    #[test]
    fn test_low_confidence_blocks_confirmed_mode() {
        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();

        let item = make_preview_item("C:\\test\\file.mkv", "C:\\test\\new.mkv", 50, false);

        // 置信度 < 阈值 -> can_execute = false -> Confirmed 模式应报错
        let result = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed);

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::SafetyCheckFailed { .. } => {} // expected
            other => panic!("Expected SafetyCheckFailed, got {:?}", other),
        }
    }

    #[test]
    fn test_audit_result_recorded() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("file.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&source, b"content").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();

        let item = make_preview_item(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            85,
            false,
        );

        let results = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed).unwrap();

        // 写入数据库
        for r in &results {
            db::insert_result(&conn, r).unwrap();
        }

        let stored = db::get_results_by_task(&conn, &task.id).unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].status, TaskStatus::Completed);
    }

    #[test]
    fn test_partial_failure_aggregation() {
        let tmp = TempDir::new().unwrap();
        let source_ok = tmp.path().join("ok.mkv");
        let target_ok = tmp.path().join("renamed_ok.mkv");
        fs::write(&source_ok, b"ok").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 2);
        db::insert_task(&conn, &task).unwrap();

        let item_ok = make_preview_item(
            source_ok.to_string_lossy().to_string().as_str(),
            target_ok.to_string_lossy().to_string().as_str(),
            85,
            false,
        );

        // 第二个文件源不存在
        let item_bad = make_preview_item(
            "C:\\nonexistent\\bad.mkv",
            "C:\\nonexistent\\bad_renamed.mkv",
            85,
            false,
        );

        let results = execute(
            &conn,
            &task.id,
            &[item_ok, item_bad],
            ExecutionMode::Confirmed,
        )
        .unwrap();

        assert_eq!(results.len(), 2);
        let summary = super::super::execution_summary::summarize(&results, 2);
        assert_eq!(summary.success, 1);
        assert_eq!(summary.failed, 1);
    }
}
