// 批量冲突测试
// 职责：测试大规模冲突检测的正确性

use app_lib::rename::conflict_detector::detect_conflicts;
use app_lib::rename::safety_checker::check_all;
use crate::test_support_bulk_generator::{generate_conflict_items, generate_unique_items, generate_safe_items};
use crate::test_support_conflict_fixtures::generate_items_with_conflicts;

#[test]
fn test_100_unique_items_no_conflicts() {
    let items = generate_unique_items(100);
    let conflicts = detect_conflicts(&items);
    assert!(conflicts.is_empty(), "100 unique items should have no conflicts");
}

#[test]
fn test_1000_unique_items_no_conflicts() {
    let items = generate_unique_items(1000);
    let conflicts = detect_conflicts(&items);
    assert!(conflicts.is_empty(), "1000 unique items should have no conflicts");
}

#[test]
fn test_100_same_target_detected() {
    let items = generate_conflict_items(100);
    let conflicts = detect_conflicts(&items);
    // All 100 items target same file → many duplicate target conflicts
    assert!(!conflicts.is_empty(), "100 same-target items should have conflicts");
    assert!(conflicts.iter().all(|c| c.blocking));
}

#[test]
fn test_5000_unique_items_no_conflicts() {
    let items = generate_unique_items(5000);
    let conflicts = detect_conflicts(&items);
    assert!(conflicts.is_empty(), "5000 unique items should have no conflicts");
}

#[test]
fn test_mixed_conflict_types_detected() {
    let items = generate_items_with_conflicts(100);
    let report = check_all(&items);
    // Items with blocking conflicts should prevent execution
    // (unless the conflict is non-blocking InvalidChars only)
    let has_blocking = items.iter().any(|i| i.conflicts.iter().any(|c| c.blocking));
    if has_blocking {
        assert!(report.blocking_reasons.iter().any(|r| r.contains("conflicts")));
    }
}

#[test]
fn test_pairwise_same_target() {
    // Every pair targets the same file
    let mut items = generate_safe_items(50);
    for item in items.iter_mut() {
        item.target_path = "C:\\media\\shared_target.mkv".to_string();
    }
    let conflicts = detect_conflicts(&items);
    assert!(!conflicts.is_empty(), "50 items with same target should have conflicts");
}
