// 批量缺失元数据测试
// 职责：测试缺失元数据项在安全检查中的正确性

use app_lib::rename::safety_checker::check_all;

#[path = "../../test_support/preview_item_builder.rs"]
mod preview_item_builder;

use preview_item_builder::PreviewItemBuilder;

#[test]
fn test_missing_year_still_passes_safety() {
    let items = vec![PreviewItemBuilder::new(0).year(None).build()];
    let report = check_all(&items);
    // Missing year doesn't affect safety — it's a metadata concern, not safety
    assert!(report.can_execute, "Missing year should not block safety");
}

#[test]
fn test_100_items_missing_year() {
    let items: Vec<_> = (0..100)
        .map(|i| PreviewItemBuilder::new(i).year(None).build())
        .collect();
    let report = check_all(&items);
    assert!(report.can_execute, "100 items missing year should not block safety");
}

#[test]
fn test_1000_items_missing_season_episode() {
    let items: Vec<_> = (0..1000)
        .map(|i| {
            PreviewItemBuilder::new(i)
                .season(None)
                .episode(None)
                .build()
        })
        .collect();
    let report = check_all(&items);
    assert!(report.can_execute, "Missing season/episode should not block safety");
}

#[test]
fn test_5000_items_missing_all_optional_metadata() {
    let items: Vec<_> = (0..5000)
        .map(|i| {
            PreviewItemBuilder::new(i)
                .year(None)
                .season(None)
                .episode(None)
                .build()
        })
        .collect();
    let report = check_all(&items);
    assert!(report.can_execute, "Missing optional metadata should not block safety");
}

#[test]
fn test_mixed_complete_and_incomplete_metadata() {
    let mut items: Vec<_> = (0..50)
        .map(|i| PreviewItemBuilder::new(i).build()) // complete
        .collect();
    let incomplete: Vec<_> = (50..100)
        .map(|i| {
            PreviewItemBuilder::new(i)
                .year(None)
                .season(None)
                .episode(None)
                .build()
        })
        .collect();
    items.extend(incomplete);
    let report = check_all(&items);
    assert!(report.can_execute, "Mixed metadata completeness should not block safety");
}
