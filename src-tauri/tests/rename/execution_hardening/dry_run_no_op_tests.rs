// DryRun 无副作用测试
// 职责：证明 DryRun 模式不会修改任何文件

use app_lib::rename::execution::{safe_execute, SafeExecuteInput, ExecutionMode};
use app_lib::rename::execution::ExecutionItemStatus;
use crate::test_support_preview_item_builder::PreviewItemBuilder;
use crate::test_support_bulk_generator::generate_safe_items;

#[test]
fn test_dry_run_no_file_modified() {
    let items = vec![
        PreviewItemBuilder::new(0)
            .source_path("C:\\nonexistent\\s1.mkv")
            .target_path("C:\\nonexistent\\t1.mkv")
            .build(),
    ];
    let input = SafeExecuteInput {
        task_id: "test-dry".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    // DryRun should succeed even with nonexistent files
    assert_eq!(output.item_results[0].status, ExecutionItemStatus::Success);
}

#[test]
fn test_dry_run_100_items_all_success() {
    let items = generate_safe_items(100);
    let input = SafeExecuteInput {
        task_id: "test-dry-100".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    assert!(output.item_results.iter().all(|r| r.status == ExecutionItemStatus::Success));
}

#[test]
fn test_dry_run_1000_items_all_success() {
    let items = generate_safe_items(1000);
    let input = SafeExecuteInput {
        task_id: "test-dry-1000".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    assert_eq!(output.item_results.len(), 1000);
    assert!(output.item_results.iter().all(|r| r.status == ExecutionItemStatus::Success));
}

#[test]
fn test_dry_run_5000_items_summary_correct() {
    let items = generate_safe_items(5000);
    let input = SafeExecuteInput {
        task_id: "test-dry-5000".to_string(),
        preview_items: items,
        mode: ExecutionMode::DryRun,
        user_confirmed: false,
    };
    let output = safe_execute(input);
    assert!(output.allowed);
    let summary = output.summary.unwrap();
    assert_eq!(summary.total, 5000);
    assert_eq!(summary.success, 5000);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.blocked, 0);
}
