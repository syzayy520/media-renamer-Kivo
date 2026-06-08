// shared 模块 - 共享工具
// 职责：路径工具、结果类型、平台适配
// 禁止：业务逻辑

pub mod path_utils;
pub mod platform;
pub mod result_types;

pub use path_utils::*;
pub use platform::*;
pub use result_types::*;
