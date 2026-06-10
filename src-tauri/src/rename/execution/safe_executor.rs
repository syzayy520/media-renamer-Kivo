// 安全执行器模块
// 职责：协调安全执行主流程（门闸 → 回滚计划 → 执行 → 结果）
// 不做单文件重命名，不做审计写入

use super::conflict_filter::{filter_non_blocking, get_blocking_items};
use super::execution_contract::{
    ExecutionItemResult, ExecutionItemStatus, ExecutionOutputSummary, SafeExecuteInput,
    SafeExecuteOutput,
};
use super::execution_mode::ExecutionMode;
use super::skip_filter::filter_actionable;
use super::single_rename::execute_single_rename;
use crate::audit::db::TaskStatus;
use crate::rename::safety_checker;
use crate::rollback::rollback_plan::RollbackPlan;

/// 执行安全重命名
///
/// 主流程：
/// 1. 检查用户确认
/// 2. 检查安全门闸（blockers）
/// 3. 生成回滚计划
/// 4. 过滤跳过项和冲突项
/// 5. 执行重命名
/// 6. 更新回滚计划状态
/// 7. 返回结果
pub fn safe_execute(input: SafeExecuteInput) -> SafeExecuteOutput {
    // 1. 检查用户确认（Confirmed 模式必须确认）
    if input.mode == ExecutionMode::Confirmed && !input.user_confirmed {
        return SafeExecuteOutput {
            allowed: false,
            rejection_reason: Some("Confirmed 模式需要用户确认".to_string()),
            item_results: vec![],
            summary: None,
            rollback_plan: None,
        };
    }

    // 2. 检查安全门闸
    let safety_report = safety_checker::check_all(&input.preview_items);
    if !safety_report.can_execute {
        return SafeExecuteOutput {
            allowed: false,
            rejection_reason: Some(format!(
                "安全检查未通过: {}",
                safety_report.blocking_reasons.join("; ")
            )),
            item_results: vec![],
            summary: None,
            rollback_plan: None,
        };
    }

    // 3. 生成回滚计划
    let mut rollback_plan = RollbackPlan::from_preview_items(&input.task_id, &input.preview_items);

    // 4. 过滤跳过项和冲突项
    let actionable_refs = filter_actionable(&input.preview_items);
    let blocking_refs = get_blocking_items(&actionable_refs);
    let non_blocking_refs = filter_non_blocking(&actionable_refs);

    let mut item_results = Vec::new();

    // 5. 记录阻塞项为失败
    for item in &blocking_refs {
        let conflicts: Vec<String> = item
            .conflicts
            .iter()
            .filter(|c| c.blocking)
            .map(|c| c.message.clone())
            .collect();
        item_results.push(ExecutionItemResult::blocked(
            &item.source_path,
            &item.target_path,
            &format!("阻塞冲突: {}", conflicts.join("; ")),
        ));
    }

    // 6. 记录跳过项
    let skipped_count = input.preview_items.len() - actionable_refs.len();

    // 7. 执行非阻塞项
    for item in &non_blocking_refs {
        let result = match input.mode {
            ExecutionMode::DryRun => {
                // DryRun: 不执行文件操作，返回模拟成功
                ExecutionItemResult::success(&item.source_path, &item.target_path)
            }
            ExecutionMode::Confirmed => {
                // Confirmed: 执行真实重命名
                let rename_result = execute_single_rename(item, &input.task_id);
                let item_result = ExecutionItemResult {
                    source_path: rename_result.source_path,
                    target_path: rename_result.target_path,
                    status: match rename_result.status {
                        TaskStatus::Completed | TaskStatus::Pending => ExecutionItemStatus::Success,
                        TaskStatus::Failed => ExecutionItemStatus::Failed,
                        _ => ExecutionItemStatus::Skipped,
                    },
                    error: rename_result.error_message,
                    rollback_entry: None,
                };

                // 成功时标记回滚计划
                if item_result.status == ExecutionItemStatus::Success {
                    rollback_plan.mark_executed(&item.source_path, &item.target_path);
                }

                item_result
            }
        };
        item_results.push(result);
    }

    // 8. 计算摘要
    let success = item_results
        .iter()
        .filter(|r| r.status == ExecutionItemStatus::Success)
        .count();
    let failed = item_results
        .iter()
        .filter(|r| r.status == ExecutionItemStatus::Failed)
        .count();
    let blocked = item_results
        .iter()
        .filter(|r| r.status == ExecutionItemStatus::Blocked)
        .count();

    let summary = ExecutionOutputSummary {
        total: input.preview_items.len(),
        success,
        failed,
        skipped: skipped_count,
        blocked,
        mode: input.mode,
        has_rollback_plan: rollback_plan.has_rollbackable_entries(),
    };

    // 9. 返回结果
    SafeExecuteOutput {
        allowed: true,
        rejection_reason: None,
        item_results,
        summary: Some(summary),
        rollback_plan: Some(rollback_plan),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::template::{MetadataSource, RenamePreviewItem};
    use crate::scan::MediaItem;

    fn make_safe_item(id: &str, source: &str, target: &str) -> RenamePreviewItem {
        RenamePreviewItem {
            id: id.to_string(),
            parsed_info: ParsedMediaInfo {
                media_item: MediaItem {
                    id: format!("media-{}", id),
                    file_path: source.to_string(),
                    file_name: "test.mkv".to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: MediaType::Movie,
                title: "Test Movie".to_string(),
                year: Some(2020),
                season: None,
                episode: None,
                episode_end: None,
                episode_title: None,
                resolution: None,
                source: None,
                video_codec: None,
                audio_codec: None,
                group: None,
                confidence: 90,
                rule_sources: vec![],
                special_type: None,
                extra_type: None,
                extra_number: None,
            },
            source_path: source.to_string(),
            original_name: "test.mkv".to_string(),
            proposed_name: "Test Movie (2020).mkv".to_string(),
            target_path: target.to_string(),
            media_type: MediaType::Movie,
            confidence: 90,
            needs_manual_review: false,
            should_skip: false,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    fn make_unsafe_item(id: &str, source: &str, target: &str) -> RenamePreviewItem {
        let mut item = make_safe_item(id, source, target);
        item.confidence = 30; // 低置信度会触发安全检查阻塞
        item
    }

    fn make_skip_item(id: &str, source: &str, target: &str) -> RenamePreviewItem {
        let mut item = make_safe_item(id, source, target);
        item.should_skip = true;
        item
    }

    #[test]
    fn test_dry_run_allowed() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![make_safe_item("1", "/s1.mkv", "/t1.mkv")],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = safe_execute(input);
        assert!(output.allowed);
        assert!(output.rollback_plan.is_some());
        assert_eq!(output.item_results.len(), 1);
        assert_eq!(output.item_results[0].status, ExecutionItemStatus::Success);
    }

    #[test]
    fn test_confirmed_requires_user_confirmation() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![make_safe_item("1", "/s1.mkv", "/t1.mkv")],
            mode: ExecutionMode::Confirmed,
            user_confirmed: false,
        };
        let output = safe_execute(input);
        assert!(!output.allowed);
        assert!(output
            .rejection_reason
            .unwrap()
            .contains("用户确认"));
    }

    #[test]
    fn test_safety_gate_blocks_unsafe_items() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![make_unsafe_item("1", "/s1.mkv", "/t1.mkv")],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = safe_execute(input);
        assert!(!output.allowed);
        assert!(output
            .rejection_reason
            .unwrap()
            .contains("安全检查未通过"));
    }

    #[test]
    fn test_skipped_items_counted() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![
                make_safe_item("1", "/s1.mkv", "/t1.mkv"),
                make_skip_item("2", "/s2.mkv", "/t2.mkv"),
            ],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = safe_execute(input);
        assert!(output.allowed);
        let summary = output.summary.unwrap();
        assert_eq!(summary.total, 2);
        assert_eq!(summary.skipped, 1);
        assert_eq!(summary.success, 1);
    }

    #[test]
    fn test_rollback_plan_generated() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![
                make_safe_item("1", "/s1.mkv", "/t1.mkv"),
                make_safe_item("2", "/s2.mkv", "/t2.mkv"),
            ],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = safe_execute(input);
        let plan = output.rollback_plan.unwrap();
        assert_eq!(plan.entries.len(), 2);
        assert_eq!(plan.entries[0].source_path, "/s1.mkv");
        assert_eq!(plan.entries[0].rollback_target, "/s1.mkv");
    }

    #[test]
    fn test_empty_preview_items() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = safe_execute(input);
        assert!(output.allowed);
        let summary = output.summary.unwrap();
        assert_eq!(summary.total, 0);
    }
}