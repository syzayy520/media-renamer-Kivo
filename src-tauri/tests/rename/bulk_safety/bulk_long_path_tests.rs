// 批量长路径测试
// 职责：测试长路径在安全检查中的正确性

use app_lib::rename::safety_checker::check_all;
use crate::test_support_long_path_fixtures::{generate_long_path_items, generate_long_path_items_bulk};
use crate::test_support_bulk_generator::generate_safe_items;

#[test]
fn test_long_path_items_blocked() {
    let items = generate_long_path_items();
    let report = check_all(&items);
    assert!(!report.can_execute, "Long path items should be blocked");
    assert!(report.blocking_reasons.iter().any(|r| r.contains("Paths too long")));
}

#[test]
fn test_100_long_path_items_blocked() {
    let items = generate_long_path_items_bulk(100);
    let report = check_all(&items);
    assert!(!report.can_execute, "100 long path items should be blocked");
}

#[test]
fn test_1000_long_path_items_blocked() {
    let items = generate_long_path_items_bulk(1000);
    let report = check_all(&items);
    assert!(!report.can_execute, "1000 long path items should be blocked");
}

#[test]
fn test_boundary_260_chars_ok() {
    let mut items = generate_safe_items(1);
    // Exactly 260 chars should pass: "C:\" = 3, ".mkv" = 4, so need 253 'a' chars
    let path_260 = format!("C:\\{}.mkv", "a".repeat(253));
    assert_eq!(path_260.len(), 260);
    items[0].target_path = path_260;
    let report = check_all(&items);
    let path_check = report.checks.iter().find(|c| c.name == "path_length").unwrap();
    assert!(path_check.passed, "260 char path should pass");
}

#[test]
fn test_boundary_261_chars_blocked() {
    let mut items = generate_safe_items(1);
    // 261 chars should fail: "C:\" = 3, ".mkv" = 4, so need 254 'a' chars
    let path_261 = format!("C:\\{}.mkv", "a".repeat(254));
    assert_eq!(path_261.len(), 261);
    items[0].target_path = path_261;
    let report = check_all(&items);
    assert!(!report.can_execute, "261 char path should be blocked");
}

#[test]
fn test_mixed_long_and_short_paths() {
    let mut items = generate_safe_items(50);
    // Add some long path items
    let long_items = generate_long_path_items_bulk(10);
    items.extend(long_items);
    let report = check_all(&items);
    assert!(!report.can_execute, "Mixed with long paths should be blocked");
}
