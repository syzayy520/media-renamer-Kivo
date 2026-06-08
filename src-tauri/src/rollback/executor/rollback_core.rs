// 回滚核心模块
// 职责：执行回滚操作
// 不做状态检查，不做统计聚合

use super::rollback_entry::{RollbackEntry, RollbackStatus};
use crate::audit::db::{self, RenameResult, TaskStatus};
use crate::audit::logger::AuditLogger;
use crate::rollback::state_checker;
use crate::shared::result_types::{AppError, AppResult};
use rusqlite::Connection;

/// 执行任务回滚
///
/// 1. 检查任务状态是否允许回滚
/// 2. 逐条检查并执行回滚
/// 3. 记录审计日志
pub fn rollback_task(
    conn: &Connection,
    task_id: &str,
    results: &[RenameResult],
) -> AppResult<Vec<RollbackEntry>> {
    // 检查任务状态
    let task = db::get_task(conn, task_id).map_err(|e| AppError::Database(e.to_string()))?;

    let task = match task {
        Some(t) => t,
        None => {
            return Err(AppError::TaskNotFound {
                task_id: task_id.to_string(),
            })
        }
    };

    // 已回滚的任务不能再次回滚
    if task.status == TaskStatus::RolledBack {
        return Err(AppError::TaskNotRollbackable {
            status: "RolledBack".to_string(),
        });
    }

    // 不在可回滚状态的任务
    if !state_checker::is_task_rollbackable(&task.status) {
        return Err(AppError::TaskNotRollbackable {
            status: task.status.to_string(),
        });
    }

    let logger = AuditLogger::new(conn);
    let mut entries = Vec::new();

    // 逐条处理
    for result in results {
        let entry = rollback_single(&logger, result);
        entries.push(entry);
    }

    // 更新任务状态
    let all_success = entries.iter().all(|e| e.status == RollbackStatus::Success);
    let new_status = if all_success {
        TaskStatus::RolledBack
    } else {
        // 部分失败时保持 Completed，让调用者决定
        TaskStatus::Completed
    };

    db::update_task_status(conn, task_id, new_status, None)
        .map_err(|e| AppError::Database(e.to_string()))?;

    // 记录回滚审计日志
    let success_count = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Success)
        .count();
    let failed_count = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Failed)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Blocked)
        .count();

    let _ = logger.log_event(
        Some(task_id),
        "rollback",
        &format!(
            "Rollback completed: {} success, {} failed, {} blocked",
            success_count, failed_count, blocked_count
        ),
    );

    Ok(entries)
}

/// 回滚单条结果
fn rollback_single(logger: &AuditLogger, result: &RenameResult) -> RollbackEntry {
    // 只有 Completed 状态可回滚
    if result.status != TaskStatus::Completed {
        return RollbackEntry {
            rename_result_id: result.id.clone(),
            status: RollbackStatus::Blocked,
            error_message: Some(format!("Status is {}, not Completed", result.status)),
        };
    }

    let target = std::path::Path::new(&result.target_path);
    let source = std::path::Path::new(&result.source_path);

    // 检查 afterPath 是否存在
    if !target.exists() {
        let _ = logger.log_failure(
            Some(&result.task_id),
            &format!(
                "Rollback blocked: afterPath missing: {}",
                result.target_path
            ),
        );
        return RollbackEntry {
            rename_result_id: result.id.clone(),
            status: RollbackStatus::Blocked,
            error_message: Some(format!("afterPath does not exist: {}", result.target_path)),
        };
    }

    // 检查 beforePath 是否被占用
    if source.exists() {
        let _ = logger.log_failure(
            Some(&result.task_id),
            &format!(
                "Rollback blocked: beforePath occupied: {}",
                result.source_path
            ),
        );
        return RollbackEntry {
            rename_result_id: result.id.clone(),
            status: RollbackStatus::Blocked,
            error_message: Some(format!(
                "beforePath already occupied: {}",
                result.source_path
            )),
        };
    }

    // 执行回滚 rename (target -> source)
    match std::fs::rename(target, source) {
        Ok(()) => {
            let _ = logger.log_event(
                Some(&result.task_id),
                "rollback_success",
                &format!(
                    "Rolled back: {} -> {}",
                    result.target_path, result.source_path
                ),
            );
            RollbackEntry {
                rename_result_id: result.id.clone(),
                status: RollbackStatus::Success,
                error_message: None,
            }
        }
        Err(e) => {
            let _ = logger.log_failure(
                Some(&result.task_id),
                &format!("Rollback failed: {}: {}", result.target_path, e),
            );
            RollbackEntry {
                rename_result_id: result.id.clone(),
                status: RollbackStatus::Failed,
                error_message: Some(format!("Rename failed: {}", e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::{self, init_tables};
    use crate::rollback::executor::rollback_entry::summarize_rollback;
    use chrono::Utc;
    use std::fs;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    fn make_result(task_id: &str, source: &str, target: &str, status: TaskStatus) -> RenameResult {
        RenameResult {
            id: Uuid::new_v4().to_string(),
            task_id: task_id.to_string(),
            source_path: source.to_string(),
            target_path: target.to_string(),
            status,
            created_at: Utc::now(),
            error_message: None,
        }
    }

    #[test]
    fn test_single_file_rollback_success() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&target, b"content").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

        let result = make_result(
            &task.id,
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &result).unwrap();

        let entries = rollback_task(&conn, &task.id, &[result]).unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].status, RollbackStatus::Success);
        assert!(source.exists());
        assert!(!target.exists());
    }

    #[test]
    fn test_multi_file_rollback_success() {
        let tmp = TempDir::new().unwrap();
        let s1 = tmp.path().join("orig1.mkv");
        let t1 = tmp.path().join("renamed1.mkv");
        let s2 = tmp.path().join("orig2.mkv");
        let t2 = tmp.path().join("renamed2.mkv");
        fs::write(&t1, b"content1").unwrap();
        fs::write(&t2, b"content2").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 2);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

        let r1 = make_result(
            &task.id,
            &s1.to_string_lossy(),
            &t1.to_string_lossy(),
            TaskStatus::Completed,
        );
        let r2 = make_result(
            &task.id,
            &s2.to_string_lossy(),
            &t2.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &r1).unwrap();
        db::insert_result(&conn, &r2).unwrap();

        let entries = rollback_task(&conn, &task.id, &[r1, r2]).unwrap();

        assert_eq!(entries.len(), 2);
        assert!(entries.iter().all(|e| e.status == RollbackStatus::Success));
        assert!(s1.exists());
        assert!(s2.exists());
    }

    #[test]
    fn test_partial_failure_recorded() {
        let tmp = TempDir::new().unwrap();
        let s1 = tmp.path().join("orig1.mkv");
        let t1 = tmp.path().join("renamed1.mkv");
        let s2 = tmp.path().join("orig2.mkv");
        let t2 = tmp.path().join("nonexistent2.mkv"); // 不存在
        fs::write(&t1, b"content1").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 2);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

        let r1 = make_result(
            &task.id,
            &s1.to_string_lossy(),
            &t1.to_string_lossy(),
            TaskStatus::Completed,
        );
        let r2 = make_result(
            &task.id,
            &s2.to_string_lossy(),
            &t2.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &r1).unwrap();
        db::insert_result(&conn, &r2).unwrap();

        let entries = rollback_task(&conn, &task.id, &[r1, r2]).unwrap();

        assert_eq!(entries.len(), 2);
        let summary = summarize_rollback(&entries);
        assert_eq!(summary.success, 1);
        assert_eq!(summary.blocked, 1);
    }

    #[test]
    fn test_beforepath_occupied_blocks() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&source, b"already back").unwrap();
        fs::write(&target, b"renamed").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

        let result = make_result(
            &task.id,
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &result).unwrap();

        let entries = rollback_task(&conn, &task.id, &[result]).unwrap();

        assert_eq!(entries[0].status, RollbackStatus::Blocked);
        assert!(entries[0]
            .error_message
            .as_ref()
            .unwrap()
            .contains("beforePath already occupied"));
    }

    #[test]
    fn test_afterpath_missing_blocks() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("nonexistent.mkv");

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

        let result = make_result(
            &task.id,
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &result).unwrap();

        let entries = rollback_task(&conn, &task.id, &[result]).unwrap();

        assert_eq!(entries[0].status, RollbackStatus::Blocked);
        assert!(entries[0]
            .error_message
            .as_ref()
            .unwrap()
            .contains("afterPath does not exist"));
    }

    #[test]
    fn test_rollback_writes_audit_log() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&target, b"content").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

        let result = make_result(
            &task.id,
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &result).unwrap();

        let _entries = rollback_task(&conn, &task.id, &[result]).unwrap();

        let logs = db::get_logs_by_task(&conn, &task.id).unwrap();
        assert!(!logs.is_empty());
        assert!(logs
            .iter()
            .any(|l| l.event_type == "rollback_success" || l.event_type == "rollback"));
    }

    #[test]
    fn test_does_not_overwrite_existing_file() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&source, b"already exists").unwrap();
        fs::write(&target, b"renamed content").unwrap();

        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

        let result = make_result(
            &task.id,
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &result).unwrap();

        let entries = rollback_task(&conn, &task.id, &[result]).unwrap();

        // 应被阻塞，不覆盖
        assert_eq!(entries[0].status, RollbackStatus::Blocked);
        // 原文件内容不变
        assert_eq!(fs::read_to_string(&source).unwrap(), "already exists");
    }

    #[test]
    fn test_idempotent_already_rolled_back() {
        let conn = setup_db();
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();
        // 任务已回滚
        db::update_task_status(&conn, &task.id, TaskStatus::RolledBack, None).unwrap();

        let result = make_result(&task.id, "a.mkv", "b.mkv", TaskStatus::Completed);

        let err = rollback_task(&conn, &task.id, &[result]).unwrap_err();
        match err {
            AppError::TaskNotRollbackable { status } => {
                assert_eq!(status, "RolledBack");
            }
            other => panic!("Expected TaskNotRollbackable, got {:?}", other),
        }
    }
}
