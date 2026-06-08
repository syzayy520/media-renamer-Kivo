// 日志仓库模块
// 职责：audit_log 表的 CRUD 操作
// 不做 task CRUD，不做 result CRUD，不做 schema 初始化

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub task_id: Option<String>,
    pub event_type: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

/// 创建审计日志条目（内存对象，不写入数据库）
pub fn create_log_entry(task_id: Option<&str>, event_type: &str, message: &str) -> AuditLogEntry {
    AuditLogEntry {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.map(|s| s.to_string()),
        event_type: event_type.to_string(),
        message: message.to_string(),
        created_at: Utc::now(),
    }
}

/// 插入审计日志
pub fn insert_log(conn: &Connection, entry: &AuditLogEntry) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO audit_log (id, task_id, event_type, message, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            entry.id,
            entry.task_id,
            entry.event_type,
            entry.message,
            entry.created_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// 查询任务的审计日志
pub fn get_logs_by_task(
    conn: &Connection,
    task_id: &str,
) -> Result<Vec<AuditLogEntry>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, event_type, message, created_at
         FROM audit_log WHERE task_id = ?1 ORDER BY created_at",
    )?;

    let logs = stmt
        .query_map(params![task_id], |row| {
            Ok(AuditLogEntry {
                id: row.get(0)?,
                task_id: row.get(1)?,
                event_type: row.get(2)?,
                message: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(logs)
}

/// 查询所有审计日志
pub fn get_all_logs(conn: &Connection) -> Result<Vec<AuditLogEntry>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, event_type, message, created_at
         FROM audit_log ORDER BY created_at",
    )?;

    let logs = stmt
        .query_map([], |row| {
            Ok(AuditLogEntry {
                id: row.get(0)?,
                task_id: row.get(1)?,
                event_type: row.get(2)?,
                message: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(logs)
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
    fn test_insert_log() {
        let conn = setup_db();
        let entry = create_log_entry(None, "system", "Audit system initialized");
        let result = insert_log(&conn, &entry);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_logs_by_task() {
        let conn = setup_db();
        let entry1 = create_log_entry(Some("task-1"), "preview", "Preview generated");
        let entry2 = create_log_entry(Some("task-1"), "execute", "Execution started");
        let entry3 = create_log_entry(Some("task-2"), "other", "Other task");

        insert_log(&conn, &entry1).unwrap();
        insert_log(&conn, &entry2).unwrap();
        insert_log(&conn, &entry3).unwrap();

        let logs = get_logs_by_task(&conn, "task-1").unwrap();
        assert_eq!(logs.len(), 2);
    }

    #[test]
    fn test_get_all_logs() {
        let conn = setup_db();
        let entry1 = create_log_entry(None, "event1", "Message 1");
        let entry2 = create_log_entry(None, "event2", "Message 2");

        insert_log(&conn, &entry1).unwrap();
        insert_log(&conn, &entry2).unwrap();

        let logs = get_all_logs(&conn).unwrap();
        assert_eq!(logs.len(), 2);
    }

    #[test]
    fn test_empty_logs() {
        let conn = setup_db();
        let logs = get_all_logs(&conn).unwrap();
        assert!(logs.is_empty());
    }

    #[test]
    fn test_get_logs_by_task_empty() {
        let conn = setup_db();
        let logs = get_logs_by_task(&conn, "nonexistent-task").unwrap();
        assert!(logs.is_empty());
    }
}
