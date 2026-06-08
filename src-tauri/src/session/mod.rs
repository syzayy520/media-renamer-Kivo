// session 模块 - 前端会话接口层
// 职责：将 pipeline 编排暴露为 Tauri 命令，供前端调用
// 不做业务逻辑，只做参数转换 + 错误映射 + Tauri 命令注册

pub mod plan_session;

pub use plan_session::*;
