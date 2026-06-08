//! rename/conflict_detection 集成测试入口
//! 由于 Rust 集成测试不自动识别子目录，通过此文件引入子模块

mod path_exists_checker_tests {
    include!("rename/conflict_detection/path_exists_checker_tests.rs");
}
mod duplicate_target_checker_tests {
    include!("rename/conflict_detection/duplicate_target_checker_tests.rs");
}
mod case_conflict_checker_tests {
    include!("rename/conflict_detection/case_conflict_checker_tests.rs");
}
mod path_length_checker_tests {
    include!("rename/conflict_detection/path_length_checker_tests.rs");
}
mod invalid_chars_checker_tests {
    include!("rename/conflict_detection/invalid_chars_checker_tests.rs");
}
mod conflict_detector_tests {
    include!("rename/conflict_detection/conflict_detector_tests.rs");
}
