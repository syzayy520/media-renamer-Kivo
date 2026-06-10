//! rollback/bulk_hardening 集成测试入口
//! 测试回滚系统在大规模数据下的正确性

mod bulk_rollback_tests {
    include!("rollback/bulk_hardening/bulk_rollback_tests.rs");
}
mod rollback_idempotency_tests {
    include!("rollback/bulk_hardening/rollback_idempotency_tests.rs");
}
mod rollback_summary_tests {
    include!("rollback/bulk_hardening/rollback_summary_tests.rs");
}
