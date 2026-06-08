//! rename/execution 集成测试入口
//! 由于 Rust 集成测试不自动识别子目录，通过此文件引入子模块

mod execution_mode_tests {
    include!("rename/execution/execution_mode_tests.rs");
}
mod safety_gate_tests {
    include!("rename/execution/safety_gate_tests.rs");
}
mod conflict_filter_tests {
    include!("rename/execution/conflict_filter_tests.rs");
}
mod skip_filter_tests {
    include!("rename/execution/skip_filter_tests.rs");
}
mod result_recorder_tests {
    include!("rename/execution/result_recorder_tests.rs");
}
mod executor_core_tests {
    include!("rename/execution/executor_core_tests.rs");
}
