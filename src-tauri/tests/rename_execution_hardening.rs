//! rename/execution_hardening 集成测试入口
//! 测试执行安全性在大规模和边界情况下的正确性

mod test_support_preview_item_builder {
    include!("test_support/preview_item_builder.rs");
}
mod test_support_bulk_generator {
    include!("test_support/bulk_generator.rs");
}

mod dry_run_no_op_tests {
    include!("rename/execution_hardening/dry_run_no_op_tests.rs");
}
mod partial_failure_tests {
    include!("rename/execution_hardening/partial_failure_tests.rs");
}
mod rollback_plan_tests {
    include!("rename/execution_hardening/rollback_plan_tests.rs");
}
mod summary_accuracy_tests {
    include!("rename/execution_hardening/summary_accuracy_tests.rs");
}
mod bulk_execution_tests {
    include!("rename/execution_hardening/bulk_execution_tests.rs");
}
