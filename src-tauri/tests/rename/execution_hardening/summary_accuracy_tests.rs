// 摘要准确性测试
// 职责：测试执行摘要统计的精确性

use app_lib::rename::execution::{safe_execute, SafeExecuteInput, ExecutionMode};
use app_lib::rename::template::RenameConflict;
use app_lib::shared::result_types::ConflictType;
use crate::test_support_preview_item_builder::PreviewItemBuilder;
use crate::test_support_bulk_generator::generate_safe_items;

#[test]
fn test_summary_all_success() {
    let items = generate_safe_items(100);
    let input = SafeExecuteInput {
        task_id: "test-summary".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 100);
    assert_eq!(summary.success, 100);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.blocked, 0);
    assert!(!summary.has_rollback_plan); // DryRun
}

#[test]
fn test_summary_mixed_blocked_and_success() {
    let mut items = generate_safe_items(50);
    // Add 10 blocked items
    for i in 50..60 {
        let mut item = PreviewItemBuilder::new(i).build();
        item.conflicts.push(RenameConflict {
            conflict_type: ConflictType::TargetExists,
            source_path: item.source_path.clone(),
            target_path: item.target_path.clone(),
            message: "Target exists".to_string(),
            blocking: true,
        });
        items.push(item);
    }
    // Add 5 skipped items
    for i in 60..65 {
        items.push(PreviewItemBuilder::new(i).should_skip(true).build());
    }
    let input = SafeExecuteInput {
        task_id: "test-summary-mixed".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 65);
    assert_eq!(summary.success, 50);
    assert_eq!(summary.blocked, 10);
    assert_eq!(summary.skipped, 5);
    assert_eq!(summary.failed, 0);
}

#[test]
fn test_summary_5000_items() {
    let items = generate_safe_items(5000);
    let input = SafeExecuteInput {
        task_id: "test-summary-5000".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 5000);
    assert_eq!(summary.success, 5000);
    assert_eq!(summary.failed, 0);
}

#[test]
fn test_summary_empty_items() {
    let input = SafeExecuteInput {
        task_id: "test-summary-empty".to_string(),
        preview_items: vec![],
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 0);
    assert_eq!(summary.success, 0);
}
