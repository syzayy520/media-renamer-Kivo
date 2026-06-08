// pipeline 模块 - 端到端编排层
// 职责：串联 Scan → Parse → Preview → Safety Check → Audit 完整流程
// 不做文件 I/O、不做模板渲染、不做冲突检测 — 委托给各子模块

pub mod orchestrator;
pub mod result;

pub use orchestrator::*;
pub use result::*;
