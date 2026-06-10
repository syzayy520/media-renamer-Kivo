// 批量执行测试
// 职责：测试大规模批量执行的正确性和性能

use app_lib::rename::execution::{safe_execute, SafeExecuteInput, ExecutionMode};
use app_lib::rename::execution::ExecutionItemStatus;
use crate::test_support_bulk_generator::generate_safe_items;

#[test]
fn test_100_items_dry_run() {
    let items = generate_safe_items(100);
    let input = SafeExecuteInput {
        task_id: "bulk-100".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    assert_eq!(output.item_results.len(), 100);
    assert!(output.item_results.iter().all(|r| r.status == ExecutionItemStatus::Success));
}

#[test]
fn test_1000_items_dry_run() {
    let items = generate_safe_items(1000);
    let input = SafeExecuteInput {
        task_id: "bulk-1000".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    assert_eq!(output.item_results.len(), 1000);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 1000);
    assert_eq!(summary.success, 1000);
}

#[test]
fn test_5000_items_dry_run() {
    let items = generate_safe_items(5000);
    let input = SafeExecuteInput {
        task_id: "bulk-5000".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    assert_eq!(output.item_results.len(), 5000);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 5000);
    assert_eq!(summary.success, 5000);
}

#[test]
fn test_confirmed_requires_user_confirmation() {
    let items = generate_safe_items(10);
    let input = SafeExecuteInput {
        task_id: "bulk-confirm".to_string(),
        preview_items: items,
        mode: ExecutionMode::Confirmed,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(!output.allowed);
    assert!(output.rejection_reason.is_some());
}

#[test]
fn test_confirmed_with_user_confirmation() {
    let items = generate_safe_items(10);
    let input = SafeExecuteInput {
        task_id: "bulk-confirmed".to_string(),
        preview_items: items,
        mode: ExecutionMode::Confirmed,
        user_confirmed: true,
    };
    let output = safe_execute(input);
    // Should be allowed (files don't exist so they'll fail, but the gate passes)
    assert!(output.allowed);
}
