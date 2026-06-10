// 批量 Unicode 文件名测试
// 职责：测试 Unicode 文件名在安全检查中的正确性

use app_lib::rename::safety_checker::check_all;
use app_lib::rename::conflict_detector::detect_conflicts;
use crate::test_support_unicode_fixtures::{generate_unicode_items, generate_unicode_items_bulk};

#[test]
fn test_unicode_items_pass_safety() {
    let items = generate_unicode_items();
    let report = check_all(&items);
    // Unicode filenames should not be blocked by safety checker
    // (invalid chars check is about OS-prohibited chars, not Unicode)
    assert!(report.can_execute, "Unicode items should pass safety check");
}

#[test]
fn test_100_unicode_items_no_false_conflicts() {
    let items = generate_unicode_items_bulk(100);
    let conflicts = detect_conflicts(&items);
    // Each item has unique index in path, so no conflicts expected
    assert!(conflicts.is_empty(), "100 unicode items with unique paths should have no conflicts");
}

#[test]
fn test_1000_unicode_items_no_false_conflicts() {
    let items = generate_unicode_items_bulk(1000);
    let conflicts = detect_conflicts(&items);
    assert!(conflicts.is_empty(), "1000 unicode items with unique paths should have no conflicts");
}

#[test]
fn test_5000_unicode_items_pass_safety() {
    let items = generate_unicode_items_bulk(5000);
    let report = check_all(&items);
    assert!(report.can_execute, "5000 unicode items should pass safety check");
}

#[test]
fn test_cjk_filenames_not_flagged_as_invalid() {
    let items = generate_unicode_items();
    // CJK filenames should not trigger invalid_chars check
    let report = check_all(&items);
    let invalid_check = report.checks.iter().find(|c| c.name == "invalid_chars").unwrap();
    assert!(invalid_check.passed, "CJK filenames should not be flagged as invalid");
}
