// 批量混合媒体类型测试
// 职责：测试混合媒体类型在安全检查中的正确性

use app_lib::rename::safety_checker::check_all;
use app_lib::rename::conflict_detector::detect_conflicts;

#[path = "../../test_support/bulk_generator.rs"]
mod bulk_generator;

use bulk_generator::generate_mixed_media_items;

#[test]
fn test_100_mixed_media_items_pass() {
    let items = generate_mixed_media_items(100);
    let report = check_all(&items);
    assert!(report.can_execute, "100 mixed media items should pass");
}

#[test]
fn test_1000_mixed_media_items_pass() {
    let items = generate_mixed_media_items(1000);
    let report = check_all(&items);
    assert!(report.can_execute, "1000 mixed media items should pass");
}

#[test]
fn test_5000_mixed_media_items_no_false_conflicts() {
    let items = generate_mixed_media_items(5000);
    let conflicts = detect_conflicts(&items);
    assert!(conflicts.is_empty(), "5000 mixed media items with unique paths should have no conflicts");
}

#[test]
fn test_mixed_media_safety_report_structure() {
    let items = generate_mixed_media_items(200);
    let report = check_all(&items);
    // Should have all 6 standard checks
    assert_eq!(report.checks.len(), 6);
    let check_names: Vec<&str> = report.checks.iter().map(|c| c.name.as_str()).collect();
    assert!(check_names.contains(&"confidence"));
    assert!(check_names.contains(&"manual_review"));
    assert!(check_names.contains(&"conflicts"));
    assert!(check_names.contains(&"invalid_chars"));
    assert!(check_names.contains(&"path_length"));
    assert!(check_names.contains(&"skipped"));
}

#[test]
fn test_mixed_media_with_mixed_confidence() {
    let mut items = generate_mixed_media_items(100);
    // Set some items to low confidence
    for (i, item) in items.iter_mut().enumerate() {
        if i % 4 == 0 {
            item.confidence = 30;
        }
    }
    let report = check_all(&items);
    assert!(!report.can_execute, "Mixed media with low confidence should be blocked");
}
