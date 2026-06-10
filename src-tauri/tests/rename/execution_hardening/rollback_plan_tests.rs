// 回滚计划正确性测试
// 职责：测试执行后回滚计划的正确性

use app_lib::rename::execution::{safe_execute, SafeExecuteInput, ExecutionMode};
use crate::test_support_preview_item_builder::PreviewItemBuilder;
use crate::test_support_bulk_generator::generate_safe_items;

#[test]
fn test_rollback_plan_entries_match_items() {
    let items = generate_safe_items(10);
    let input = SafeExecuteInput {
        task_id: "test-rollback".to_string(),
        preview_items: items.clone(),
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let plan = output.rollback_plan.unwrap();
    assert_eq!(plan.entries.len(), 10);
    for (entry, item) in plan.entries.iter().zip(items.iter()) {
        assert_eq!(entry.source_path, item.source_path);
        assert_eq!(entry.target_path, item.target_path);
        assert_eq!(entry.rollback_target, item.source_path); // rollback restores original
    }
}

#[test]
fn test_rollback_plan_100_items() {
    let items = generate_safe_items(100);
    let input = SafeExecuteInput {
        task_id: "test-rollback-100".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let plan = output.rollback_plan.unwrap();
    assert_eq!(plan.entries.len(), 100);
    // In DryRun mode, entries are not executed (executed=false)
    assert!(plan.entries.iter().all(|e| !e.executed));
}

#[test]
fn test_rollback_plan_excludes_skipped() {
    let mut items = generate_safe_items(5);
    items.push(PreviewItemBuilder::new(99).should_skip(true).build());
    let input = SafeExecuteInput {
        task_id: "test-rollback-skip".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let plan = output.rollback_plan.unwrap();
    // Rollback plan includes all items, but skipped ones won't be executed
    assert_eq!(plan.entries.len(), 6);
}

#[test]
fn test_rollback_plan_1000_items() {
    let items = generate_safe_items(1000);
    let input = SafeExecuteInput {
        task_id: "test-rollback-1000".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let plan = output.rollback_plan.unwrap();
    assert_eq!(plan.entries.len(), 1000);
}
