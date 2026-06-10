//! audit/bulk_hardening 集成测试入口
//! 测试审计系统在大规模数据下的正确性

mod bulk_statistics_tests {
    include!("audit/bulk_hardening/bulk_statistics_tests.rs");
}
mod export_bulk_tests {
    include!("audit/bulk_hardening/export_bulk_tests.rs");
}
mod redaction_bulk_tests {
    include!("audit/bulk_hardening/redaction_bulk_tests.rs");
}
