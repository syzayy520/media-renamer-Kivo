// rollback/executor 子模块 - 回滚执行功能族
// 职责：执行回滚操作
// mod.rs 只做 registry / alias / re-export

pub mod rollback_core;
pub mod rollback_entry;

// 重新导出常用类型，保持向后兼容
pub use rollback_core::rollback_task;
pub use rollback_entry::{summarize_rollback, RollbackEntry, RollbackStatus, RollbackSummary};
