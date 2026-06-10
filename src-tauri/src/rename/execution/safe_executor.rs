// 安全执行器模块（薄编排层）
// 职责：协调安全执行主流程
// 实际逻辑委托给子模块：确认门闸、安全门闸、回滚计划、执行器、审计、摘要

use super::conflict_filter::{filter_non_blocking, get_blocking_items};
use super::confirmed_rename_executor::execute_confirmed_rename;
use super::dry_run_executor::execute_dry_run;
use super::execution_audit_step::format_batch_execution_audit;
use super::execution_confirmation_gate::check_user_confirmation;
use super::execution_contract::{
    ExecutionItemResult, SafeExecuteInput, SafeExecuteOutput,
};
use super::execution_mode::ExecutionMode;
use super::execution_rollback_plan_step::generate_rollback_plan;
use super::execution_safety_gate_check::check_safety_gate;
use super::execution_summary_builder::build_execution_summary;
use super::skip_filter::filter_actionable;

/// 执行安全重命名
///
/// 主流程（委托给子模块）：
/// 1. 检查用户确认 → execution_confirmation_gate
/// 2. 检查安全门闸 → execution_safety_gate_check
/// 3. 生成回滚计划 → execution_rollback_plan_step
/// 4. 过滤跳过项和冲突项
/// 5. 执行重命名 → dry_run_executor / confirmed_rename_executor
/// 6. 记录审计 → execution_audit_step
/// 7. 计算摘要 → execution_summary_builder
pub fn safe_execute(input: SafeExecuteInput) -> SafeExecuteOutput {
    // 1. 检查用户确认
    if let Some(rejection) = check_user_confirmation(&input) {
        return rejection;
    }

    // 2. 检查安全门闸
    if let Some(rejection) = check_safety_gate(&input) {
        return rejection;
    }

    // 3. 生成回滚计划
    let mut rollback_plan = generate_rollback_plan(&input.task_id, &input.preview_items);

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
    let execution_results = match input.mode {
        ExecutionMode::DryRun => execute_dry_run(&non_blocking_refs),
        ExecutionMode::Confirmed => {
            execute_confirmed_rename(&non_blocking_refs, &input.task_id, &mut rollback_plan)
        }
    };
    item_results.extend(execution_results);

    // 8. 记录审计（格式化消息，实际写入由调用方负责）
    let _audit_messages = format_batch_execution_audit(&input.task_id, &item_results);

    // 9. 计算摘要
    let summary = build_execution_summary(
        &item_results,
        input.preview_items.len(),
        skipped_count,
        input.mode,
        &rollback_plan,
    );

    // 10. 返回结果
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
    use crate::rename::execution::ExecutionItemStatus;
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
        item.confidence = 30;
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
        assert!(output.rejection_reason.unwrap().contains("用户确认"));
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
        assert!(output.rejection_reason.unwrap().contains("安全检查未通过"));
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
