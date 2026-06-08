// 回滚检查模块
// 职责：检查任务是否可回滚
// 不做回滚操作，不做审计记录

use crate::audit::db::{self, TaskStatus};
use crate::rollback::state_checker;
use crate::shared::result_types::{AppError, AppResult};
use rusqlite::Connection;

/// 检查任务是否可回滚
pub fn check_task_rollbackable(conn: &Connection, task_id: &str) -> AppResult<()> {
    // 检查任务存在
    let task = db::get_task(conn, task_id).map_err(|e| AppError::Database(e.to_string()))?;

    let task = match task {
        Some(t) => t,
        None => {
            return Err(AppError::TaskNotFound {
                task_id: task_id.to_string(),
            })
        }
    };

    // 已回滚的任务不能再次回滚
    if task.status == TaskStatus::RolledBack {
        return Err(AppError::TaskNotRollbackable {
            status: "RolledBack".to_string(),
        });
    }

    // 不在可回滚状态的任务
    if !state_checker::is_task_rollbackable(&task.status) {
        return Err(AppError::TaskNotRollbackable {
            status: task.status.to_string(),
        });
    }

    Ok(())
}
