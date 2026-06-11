//! 命名规则模块 — Token-based 自由顺序命名规则引擎
//!
//! 子模块:
//! - token: Token 类型定义与字段池
//! - preset: 内置预设
//! - site_release_preset: 站点发布名预设
//! - token_render: Token 渲染引擎
//! - template_validation: 命名结果清理验证
//! - title_strategy: 标题策略（中文/英文/双语/PT前缀）
//! - pt_preserve: PT/BT 原始命名信息保留
//! - separator_cleanup: 分隔符清理与空token处理

pub mod preset;
pub mod pt_preserve;
pub mod separator_cleanup;
pub mod site_release_preset;
pub mod template_validation;
pub mod title_strategy;
pub mod token;
pub mod token_render;

pub use preset::*;
pub use pt_preserve::*;
pub use separator_cleanup::*;
pub use site_release_preset::*;
pub use template_validation::*;
pub use title_strategy::*;
pub use token::*;
pub use token_render::*;
