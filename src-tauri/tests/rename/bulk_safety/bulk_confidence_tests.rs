// 批量置信度测试
// 职责：测试大规模置信度混合下的安全检查正确性

use app_lib::rename::safety_checker::check_all;
use crate::test_support_bulk_generator::{generate_mixed_confidence_items, generate_safe_items};
use crate::test_support_preview_item_builder::PreviewItemBuilder;

#[test]
fn test_100_safe_items_all_pass() {
    let items = generate_safe_items(100);
    let report = check_all(&items);
    assert!(report.can_execute, "100 safe items should pass");
    assert!(report.blocking_reasons.is_empty());
}

#[test]
fn test_1000_safe_items_all_pass() {
    let items = generate_safe_items(1000);
    let report = check_all(&items);
    assert!(report.can_execute, "1000 safe items should pass");
    assert!(report.blocking_reasons.is_empty());
}

#[test]
fn test_5000_safe_items_all_pass() {
    let items = generate_safe_items(5000);
    let report = check_all(&items);
    assert!(report.can_execute, "5000 safe items should pass");
    assert!(report.blocking_reasons.is_empty());
}

#[test]
fn test_100_mixed_confidence_some_blocked() {
    let items = generate_mixed_confidence_items(100);
    let report = check_all(&items);
    // 50% have low confidence (40) — should block
    assert!(!report.can_execute);
    assert!(report.blocking_reasons.iter().any(|r| r.contains("Low confidence")));
}

#[test]
fn test_single_low_confidence_blocks_all() {
    let mut items = generate_safe_items(100);
    items.push(PreviewItemBuilder::new(999).confidence(30).build());
    let report = check_all(&items);
    assert!(!report.can_execute, "One low confidence item should block batch");
}

#[test]
fn test_manual_review_blocks_confirmed() {
    let mut items = generate_safe_items(50);
    items.push(PreviewItemBuilder::new(999).needs_manual_review(true).build());
    let report = check_all(&items);
    assert!(!report.can_execute, "Manual review item should block batch");
    assert!(report.blocking_reasons.iter().any(|r| r.contains("Manual review")));
}

#[test]
fn test_5000_mixed_confidence_statistics() {
    let items = generate_mixed_confidence_items(5000);
    let report = check_all(&items);
    // Exactly 2500 low confidence items (odd indices)
    let low_conf_check = report.checks.iter().find(|c| c.name == "confidence").unwrap();
    assert!(!low_conf_check.passed);
    assert!(report.blocking_reasons.iter().any(|r| r.contains("2500")));
}
