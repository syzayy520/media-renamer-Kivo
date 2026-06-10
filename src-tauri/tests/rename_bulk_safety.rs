//! rename/bulk_safety 集成测试入口
//! 测试 safety_checker 在大规模数据下的正确性

mod test_support_preview_item_builder {
    include!("test_support/preview_item_builder.rs");
}
mod test_support_bulk_generator {
    include!("test_support/bulk_generator.rs");
}
mod test_support_unicode_fixtures {
    include!("test_support/unicode_fixtures.rs");
}
mod test_support_long_path_fixtures {
    include!("test_support/long_path_fixtures.rs");
}
mod test_support_conflict_fixtures {
    include!("test_support/conflict_fixtures.rs");
}

mod bulk_confidence_tests {
    include!("rename/bulk_safety/bulk_confidence_tests.rs");
}
mod bulk_conflict_tests {
    include!("rename/bulk_safety/bulk_conflict_tests.rs");
}
mod bulk_unicode_tests {
    include!("rename/bulk_safety/bulk_unicode_tests.rs");
}
mod bulk_long_path_tests {
    include!("rename/bulk_safety/bulk_long_path_tests.rs");
}
mod bulk_missing_metadata_tests {
    include!("rename/bulk_safety/bulk_missing_metadata_tests.rs");
}
mod bulk_mixed_media_tests {
    include!("rename/bulk_safety/bulk_mixed_media_tests.rs");
}
