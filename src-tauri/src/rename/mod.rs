// rename 模块 - 重命名功能族
// 职责：模板渲染、冲突检测、安全检查、预览生成、执行改名
// 允许业务逻辑：是

// 子模块声明
pub mod case_conflict_checker;
pub mod conflict_detector;
pub mod duplicate_target_checker;
pub mod execution;
pub mod invalid_chars_checker;
pub mod path_exists_checker;
pub mod path_length_checker;
pub mod preview_generator;
pub mod safety_checker;
pub mod template;
