// 任务仓库模块
// 职责：rename_tasks 表的 CRUD 操作
// 不做 result CRUD，不做 log CRUD，不做 schema 初始化

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::task_status::{parse_task_status, TaskStatus};

/// 重命名任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameTask {
    pub id: String,
    pub status: TaskStatus,
    pub template: String,
    pub total_files: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// 创建新任务（内存对象，不写入数据库）
pub fn create_task(template: &str, total_files: u32) -> RenameTask {
    let now = Utc::now();
    RenameTask {
        id: Uuid::new_v4().to_string(),
        status: TaskStatus::Previewing,
        template: template.to_string(),
        total_files,
        created_at: now,
        updated_at: now,
        error_message: None,
    }
}

/// 插入重命名任务
pub fn insert_task(conn: &Connection, task: &RenameTask) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO rename_tasks (id, status, template, total_files, created_at, updated_at, error_message)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            task.id,
            task.status.to_string(),
            task.template,
            task.total_files,
            task.created_at.to_rfc3339(),
            task.updated_at.to_rfc3339(),
            task.error_message,
        ],
    )?;
    Ok(())
}

/// 查询重命名任务
pub fn get_task(conn: &Connection, task_id: &str) -> Result<Option<RenameTask>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, status, template, total_files, created_at, updated_at, error_message
         FROM rename_tasks WHERE id = ?1",
    )?;

    let mut rows = stmt.query_map(params![task_id], |row| {
        let status_str: String = row.get(1)?;
        Ok(RenameTask {
            id: row.get(0)?,
            status: parse_task_status(&status_str),
            template: row.get(2)?,
            total_files: row.get(3)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            error_message: row.get(6)?,
        })
    })?;

    match rows.next() {
        Some(Ok(task)) => Ok(Some(task)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

/// 更新任务状态
pub fn update_task_status(
    conn: &Connection,
    task_id: &str,
    status: TaskStatus,
    error_message: Option<&str>,
) -> Result<(), rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE rename_tasks SET status = ?1, updated_at = ?2, error_message = ?3 WHERE id = ?4",
        params![status.to_string(), now, error_message, task_id],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::connection::create_memory_connection;
    use crate::audit::db::schema::init_tables;

    fn setup_db() -> Connection {
        let conn = create_memory_connection().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_insert_task() {
        let conn = setup_db();
        let task = create_task("{Title}", 10);
        let result = insert_task(&conn, &task);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_task() {
        let conn = setup_db();
        let task = create_task("{Title}", 10);
        insert_task(&conn, &task).unwrap();

        let retrieved = get_task(&conn, &task.id).unwrap().unwrap();
        assert_eq!(retrieved.id, task.id);
        assert_eq!(retrieved.template, "{Title}");
        assert_eq!(retrieved.total_files, 10);
        assert_eq!(retrieved.status, TaskStatus::Previewing);
    }

    #[test]
    fn test_update_task_status() {
        let conn = setup_db();
        let task = create_task("{Title}", 5);
        insert_task(&conn, &task).unwrap();

        update_task_status(&conn, &task.id, TaskStatus::Failed, Some("Parse error")).unwrap();

        let updated = get_task(&conn, &task.id).unwrap().unwrap();
        assert_eq!(updated.status, TaskStatus::Failed);
        assert_eq!(updated.error_message, Some("Parse error".to_string()));
    }

    #[test]
    fn test_failed_status_record() {
        let conn = setup_db();
        let mut task = create_task("{Title}", 3);
        task.status = TaskStatus::Failed;
        task.error_message = Some("Invalid template".to_string());
        insert_task(&conn, &task).unwrap();

        let retrieved = get_task(&conn, &task.id).unwrap().unwrap();
        assert_eq!(retrieved.status, TaskStatus::Failed);
        assert!(retrieved.error_message.is_some());
    }

    #[test]
    fn test_get_nonexistent_task() {
        let conn = setup_db();
        let result = get_task(&conn, "nonexistent-id").unwrap();
        assert!(result.is_none());
    }
}
