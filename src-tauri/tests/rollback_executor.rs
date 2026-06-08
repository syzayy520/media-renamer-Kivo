//! rollback/executor 集成测试入口
//! 由于 Rust 集成测试不自动识别子目录，通过此文件引入子模块

mod rollback_entry_tests {
    include!("rollback/executor/rollback_entry_tests.rs");
}
mod rollback_single_tests {
    include!("rollback/executor/rollback_single_tests.rs");
}
mod rollback_audit_tests {
    include!("rollback/executor/rollback_audit_tests.rs");
}
mod rollback_core_tests {
    include!("rollback/executor/rollback_core_tests.rs");
}
