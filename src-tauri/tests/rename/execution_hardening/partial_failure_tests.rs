// 部分失败测试
// 职责：测试部分项失败时的摘要正确性和错误消息

use app_lib::rename::execution::{safe_execute, SafeExecuteInput, ExecutionMode};
use app_lib::rename::execution::ExecutionItemStatus;
use app_lib::rename::template::RenameConflict;
use app_lib::shared::result_types::ConflictType;
use crate::test_support_preview_item_builder::PreviewItemBuilder;

fn make_mixed_items() -> Vec<app_lib::rename::template::RenamePreviewItem> {
    let mut items = Vec::new();
    // 3 safe items
    for i in 0..3 {
        items.push(PreviewItemBuilder::new(i).build());
    }
    // 2 items with conflicts (blocking)
    for i in 3..5 {
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
    // 1 skipped item
    items.push(PreviewItemBuilder::new(5).should_skip(true).build());
    items
}

#[test]
fn test_partial_failure_summary_counts() {
    let items = make_mixed_items();
    let input = SafeExecuteInput {
        task_id: "test-partial".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 6);
    assert_eq!(summary.success, 3); // 3 safe items
    assert_eq!(summary.skipped, 1); // 1 skipped item
    assert_eq!(summary.blocked, 2); // 2 blocking conflict items
    assert_eq!(summary.failed, 0); // no execution failures in DryRun
}

#[test]
fn test_blocked_items_have_error_messages() {
    let items = make_mixed_items();
    let input = SafeExecuteInput {
        task_id: "test-partial-msg".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    let blocked: Vec<_> = output.item_results.iter()
        .filter(|r| r.status == ExecutionItemStatus::Blocked)
        .collect();
    assert_eq!(blocked.len(), 2);
    for item in blocked {
        assert!(item.error.is_some(), "Blocked items should have error messages");
        assert!(item.error.as_ref().unwrap().contains("阻塞冲突"));
    }
}

#[test]
fn test_100_mixed_items_summary() {
    let mut items = Vec::new();
    // 80 safe
    for i in 0..80 {
        items.push(PreviewItemBuilder::new(i).confidence(90).build());
    }
    // 20 low confidence
    for i in 80..100 {
        items.push(PreviewItemBuilder::new(i).confidence(30).build());
    }
    let input = SafeExecuteInput {
        task_id: "test-mixed-100".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    // Safety gate should block due to low confidence items
    assert!(!output.allowed);
    assert!(output.rejection_reason.is_some());
}
