// rollback 模块 - 回滚功能族
// 职责：状态检查、回滚执行
// 允许业务逻辑：是

// 子模块声明
pub mod executor;
pub mod rollback_plan;
pub mod state_checker;
