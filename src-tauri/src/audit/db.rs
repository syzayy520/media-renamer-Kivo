// 审计数据库模块
// 职责：SQLite 表初始化、rename_tasks / rename_results / audit_log 表 CRUD
// 不做业务决策，不做 redaction 逻辑

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 重命名任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    /// 预览生成中
    Previewing,
    /// 等待执行
    Pending,
    /// 执行中
    Executing,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已回滚
    RolledBack,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Previewing => write!(f, "Previewing"),
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Executing => write!(f, "Executing"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
            TaskStatus::RolledBack => write!(f, "RolledBack"),
        }
    }
}

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

/// 重命名结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameResult {
    pub id: String,
    pub task_id: String,
    pub source_path: String,
    pub target_path: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,
    pub task_id: Option<String>,
    pub event_type: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

/// 初始化数据库表
pub fn init_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS rename_tasks (
            id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            template TEXT NOT NULL,
            total_files INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            error_message TEXT
        );

        CREATE TABLE IF NOT EXISTS rename_results (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL,
            source_path TEXT NOT NULL,
            target_path TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            error_message TEXT,
            FOREIGN KEY (task_id) REFERENCES rename_tasks(id)
        );

        CREATE TABLE IF NOT EXISTS audit_log (
            id TEXT PRIMARY KEY,
            task_id TEXT,
            event_type TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
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

/// 插入重命名结果
pub fn insert_result(conn: &Connection, result: &RenameResult) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO rename_results (id, task_id, source_path, target_path, status, created_at, error_message)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            result.id,
            result.task_id,
            result.source_path,
            result.target_path,
            result.status.to_string(),
            result.created_at.to_rfc3339(),
            result.error_message,
        ],
    )?;
    Ok(())
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
            created_at: parse_datetime(&row.get::<_, String>(4)?),
            updated_at: parse_datetime(&row.get::<_, String>(5)?),
            error_message: row.get(6)?,
        })
    })?;

    match rows.next() {
        Some(Ok(task)) => Ok(Some(task)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

/// 查询任务的所有结果
pub fn get_results_by_task(
    conn: &Connection,
    task_id: &str,
) -> Result<Vec<RenameResult>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, source_path, target_path, status, created_at, error_message
         FROM rename_results WHERE task_id = ?1",
    )?;

    let results = stmt
        .query_map(params![task_id], |row| {
            let status_str: String = row.get(4)?;
            Ok(RenameResult {
                id: row.get(0)?,
                task_id: row.get(1)?,
                source_path: row.get(2)?,
                target_path: row.get(3)?,
                status: parse_task_status(&status_str),
                created_at: parse_datetime(&row.get::<_, String>(5)?),
                error_message: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(results)
}

/// 查询审计日志
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
                created_at: parse_datetime(&row.get::<_, String>(4)?),
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
                created_at: parse_datetime(&row.get::<_, String>(4)?),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(logs)
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

/// 创建新任务
pub fn create_task(_conn: &Connection, template: &str, total_files: u32) -> RenameTask {
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

/// 创建新结果
pub fn create_result(
    task_id: &str,
    source_path: &str,
    target_path: &str,
    status: TaskStatus,
) -> RenameResult {
    RenameResult {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        source_path: source_path.to_string(),
        target_path: target_path.to_string(),
        status,
        created_at: Utc::now(),
        error_message: None,
    }
}

/// 创建审计日志条目
pub fn create_log_entry(
    task_id: Option<&str>,
    event_type: &str,
    message: &str,
) -> AuditLogEntry {
    AuditLogEntry {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.map(|s| s.to_string()),
        event_type: event_type.to_string(),
        message: message.to_string(),
        created_at: Utc::now(),
    }
}

/// 解析任务状态字符串
fn parse_task_status(s: &str) -> TaskStatus {
    match s {
        "Previewing" => TaskStatus::Previewing,
        "Pending" => TaskStatus::Pending,
        "Executing" => TaskStatus::Executing,
        "Completed" => TaskStatus::Completed,
        "Failed" => TaskStatus::Failed,
        "RolledBack" => TaskStatus::RolledBack,
        _ => TaskStatus::Failed, // 默认为 Failed
    }
}

/// 解析 ISO 8601 日期时间
fn parse_datetime(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_init_tables() {
        let conn = setup_db();
        // 验证表已创建
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='rename_tasks'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_insert_and_get_task() {
        let conn = setup_db();
        let task = create_task(&conn, "{Title}", 10);
        insert_task(&conn, &task).unwrap();

        let retrieved = get_task(&conn, &task.id).unwrap().unwrap();
        assert_eq!(retrieved.id, task.id);
        assert_eq!(retrieved.template, "{Title}");
        assert_eq!(retrieved.total_files, 10);
        assert_eq!(retrieved.status, TaskStatus::Previewing);
    }

    #[test]
    fn test_insert_and_get_result() {
        let conn = setup_db();
        let task = create_task(&conn, "{Title}", 1);
        insert_task(&conn, &task).unwrap();

        let result = create_result(&task.id, "/src/file.mkv", "/dst/file.mkv", TaskStatus::Completed);
        insert_result(&conn, &result).unwrap();

        let results = get_results_by_task(&conn, &task.id).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].source_path, "/src/file.mkv");
        assert_eq!(results[0].status, TaskStatus::Completed);
    }

    #[test]
    fn test_insert_and_get_log() {
        let conn = setup_db();
        let entry = create_log_entry(None, "system", "Audit system initialized");
        insert_log(&conn, &entry).unwrap();

        let logs = get_all_logs(&conn).unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].event_type, "system");
    }

    #[test]
    fn test_update_task_status() {
        let conn = setup_db();
        let task = create_task(&conn, "{Title}", 5);
        insert_task(&conn, &task).unwrap();

        update_task_status(&conn, &task.id, TaskStatus::Failed, Some("Parse error")).unwrap();

        let updated = get_task(&conn, &task.id).unwrap().unwrap();
        assert_eq!(updated.status, TaskStatus::Failed);
        assert_eq!(updated.error_message, Some("Parse error".to_string()));
    }

    #[test]
    fn test_failed_status_record() {
        let conn = setup_db();
        let mut task = create_task(&conn, "{Title}", 3);
        task.status = TaskStatus::Failed;
        task.error_message = Some("Invalid template".to_string());
        insert_task(&conn, &task).unwrap();

        let retrieved = get_task(&conn, &task.id).unwrap().unwrap();
        assert_eq!(retrieved.status, TaskStatus::Failed);
        assert!(retrieved.error_message.is_some());
    }

    #[test]
    fn test_get_logs_by_task() {
        let conn = setup_db();
        let task = create_task(&conn, "{Title}", 1);
        insert_task(&conn, &task).unwrap();

        let entry1 = create_log_entry(Some(&task.id), "preview", "Preview generated");
        let entry2 = create_log_entry(Some(&task.id), "execute", "Execution started");
        insert_log(&conn, &entry1).unwrap();
        insert_log(&conn, &entry2).unwrap();

        let logs = get_logs_by_task(&conn, &task.id).unwrap();
        assert_eq!(logs.len(), 2);
    }

    #[test]
    fn test_task_status_display() {
        assert_eq!(TaskStatus::Previewing.to_string(), "Previewing");
        assert_eq!(TaskStatus::Completed.to_string(), "Completed");
        assert_eq!(TaskStatus::RolledBack.to_string(), "RolledBack");
    }
}
