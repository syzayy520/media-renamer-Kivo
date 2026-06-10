// rename/execution 子模块 - 重命名执行功能族
// 职责：受控重命名执行
// mod.rs 只做 registry / alias / re-export

pub mod conflict_filter;
pub mod execution_contract;
pub mod execution_mode;
pub mod execution_summary;
pub mod executor_core;
pub mod result_recorder;
pub mod safe_executor;
pub mod safety_gate;
pub mod single_rename;
pub mod skip_filter;

// 重新导出常用类型，保持向后兼容
pub use execution_contract::{
    ExecutionItemResult, ExecutionItemStatus, ExecutionOutputSummary, SafeExecuteInput,
    SafeExecuteOutput,
};
pub use execution_mode::ExecutionMode;
pub use execution_summary::{summarize, ExecutionSummary};
pub use executor_core::execute;
pub use safe_executor::safe_execute;
pub use single_rename::execute_single_rename;
