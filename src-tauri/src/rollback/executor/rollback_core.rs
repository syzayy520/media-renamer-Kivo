// 回滚核心模块
// 职责：协调回滚任务主流程编排
// 不做状态检查，不做审计记录，不做结果汇总

use super::rollback_audit::{log_rollback_audit, update_task_status};
use super::rollback_checker::check_task_rollbackable;
use super::rollback_entry::RollbackEntry;
use super::rollback_single::rollback_single;
use crate::audit::db::RenameResult;
use crate::audit::logger::AuditLogger;
use crate::shared::result_types::AppResult;
use rusqlite::Connection;

/// 执行任务回滚
///
/// 主流程编排：
/// 1. 检查任务状态是否允许回滚
/// 2. 逐条执行回滚
/// 3. 更新任务状态
/// 4. 记录审计日志
pub fn rollback_task(
    conn: &Connection,
    task_id: &str,
    results: &[RenameResult],
) -> AppResult<Vec<RollbackEntry>> {
    // 1. 检查任务状态是否允许回滚
    check_task_rollbackable(conn, task_id)?;

    let logger = AuditLogger::new(conn);
    let mut entries = Vec::new();

    // 2. 逐条执行回滚
    for result in results {
        let entry = rollback_single(&logger, result);
        entries.push(entry);
    }

    // 3. 更新任务状态
    update_task_status(conn, task_id, &entries)?;

    // 4. 记录审计日志
    log_rollback_audit(conn, task_id, &entries)?;

    Ok(entries)
}
