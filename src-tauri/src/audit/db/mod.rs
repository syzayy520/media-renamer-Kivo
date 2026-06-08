// audit/db 模块 - 数据库功能族
// 职责：SQLite 连接、Schema 初始化、任务/结果/日志仓库
// mod.rs 只做 registry / alias / re-export

pub mod connection;
pub mod log_repository;
pub mod result_repository;
pub mod schema;
pub mod task_repository;
pub mod task_status;

// 重新导出常用类型，保持向后兼容
pub use log_repository::{
    create_log_entry, get_all_logs, get_logs_by_task, insert_log, AuditLogEntry,
};
pub use result_repository::{create_result, get_results_by_task, insert_result, RenameResult};
pub use schema::init_tables;
pub use task_repository::{create_task, get_task, insert_task, update_task_status, RenameTask};
pub use task_status::TaskStatus;
