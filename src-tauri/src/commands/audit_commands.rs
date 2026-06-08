// audit_commands 模块 - 审计相关 Tauri 命令
// 职责：将 audit 子模块暴露为 Tauri 命令
// 使用 DbState（与 session 共享）

use crate::audit::db;
use crate::audit::db::{AuditLogEntry, RenameTask};
use crate::session::plan_session::DbState;

/// 获取指定任务
#[tauri::command]
pub fn get_task(
    task_id: String,
    db_state: tauri::State<'_, DbState>,
) -> Result<RenameTask, String> {
    let conn = db_state
        .conn
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    db::get_task(&conn, &task_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task not found: {}", task_id))
}

/// 获取所有任务（按创建时间倒序）
#[tauri::command]
pub fn get_all_tasks(db_state: tauri::State<'_, DbState>) -> Result<Vec<RenameTask>, String> {
    let conn = db_state
        .conn
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    // 使用 settings_repository 的模式，查询所有任务
    let mut stmt = conn
        .prepare(
            "SELECT id, status, template, total_files, created_at, updated_at, error_message
             FROM rename_tasks ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            let status_str: String = row.get(1)?;
            Ok(RenameTask {
                id: row.get(0)?,
                status: db::task_status::parse_task_status(&status_str),
                template: row.get(2)?,
                total_files: row.get(3)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                error_message: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(tasks)
}

/// 获取指定任务的审计日志
#[tauri::command]
pub fn get_audit_logs(
    task_id: String,
    db_state: tauri::State<'_, DbState>,
) -> Result<Vec<AuditLogEntry>, String> {
    let conn = db_state
        .conn
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    db::get_logs_by_task(&conn, &task_id).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::task_repository;

    fn setup_db() -> DbState {
        DbState::new_memory().unwrap()
    }

    #[test]
    fn test_get_task_found() {
        let state = setup_db();
        let conn = state.conn.lock().unwrap();

        let task = task_repository::create_task("{Title}", 5);
        task_repository::insert_task(&conn, &task).unwrap();
        drop(conn);

        let conn = state.conn.lock().unwrap();
        let found = db::get_task(&conn, &task.id).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().template, "{Title}");
    }

    #[test]
    fn test_get_task_not_found() {
        let state = setup_db();
        let conn = state.conn.lock().unwrap();

        let found = db::get_task(&conn, "nonexistent").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_get_audit_logs_empty() {
        let state = setup_db();
        let conn = state.conn.lock().unwrap();

        let logs = db::get_logs_by_task(&conn, "task-123").unwrap();
        assert!(logs.is_empty());
    }
}
