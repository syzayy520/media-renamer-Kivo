// 回滚审计模块
// 职责：记录回滚审计日志
// 不做回滚操作，不做状态检查

use super::rollback_entry::{RollbackEntry, RollbackStatus};
use crate::audit::db::{self, TaskStatus};
use crate::audit::logger::AuditLogger;
use crate::shared::result_types::{AppError, AppResult};
use rusqlite::Connection;

/// 更新任务状态
pub fn update_task_status(
    conn: &Connection,
    task_id: &str,
    entries: &[RollbackEntry],
) -> AppResult<()> {
    let all_success = entries.iter().all(|e| e.status == RollbackStatus::Success);
    let new_status = if all_success {
        TaskStatus::RolledBack
    } else {
        // 部分失败时保持 Completed，让调用者决定
        TaskStatus::Completed
    };

    db::update_task_status(conn, task_id, new_status, None)
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(())
}

/// 记录回滚审计日志
pub fn log_rollback_audit(
    conn: &Connection,
    task_id: &str,
    entries: &[RollbackEntry],
) -> AppResult<()> {
    let logger = AuditLogger::new(conn);

    let success_count = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Success)
        .count();
    let failed_count = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Failed)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Blocked)
        .count();

    let _ = logger.log_event(
        Some(task_id),
        "rollback",
        &format!(
            "Rollback completed: {} success, {} failed, {} blocked",
            success_count, failed_count, blocked_count
        ),
    );

    Ok(())
}
