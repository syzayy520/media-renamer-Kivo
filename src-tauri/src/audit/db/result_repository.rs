// 结果仓库模块
// 职责：rename_results 表的 CRUD 操作
// 不做 task CRUD，不做 log CRUD，不做 schema 初始化

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::task_status::{parse_task_status, TaskStatus};

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

/// 创建新结果（内存对象，不写入数据库）
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
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                error_message: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::connection::create_memory_connection;
    use crate::audit::db::schema::init_tables;
    use crate::audit::db::task_repository::{create_task, insert_task};

    fn setup_db() -> Connection {
        let conn = create_memory_connection().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_insert_result() {
        let conn = setup_db();
        let task = create_task("{Title}", 1);
        insert_task(&conn, &task).unwrap();

        let result = create_result(
            &task.id,
            "/src/file.mkv",
            "/dst/file.mkv",
            TaskStatus::Completed,
        );
        let insert_result = insert_result(&conn, &result);
        assert!(insert_result.is_ok());
    }

    #[test]
    fn test_get_results_by_task() {
        let conn = setup_db();
        let task = create_task("{Title}", 1);
        insert_task(&conn, &task).unwrap();

        let result = create_result(
            &task.id,
            "/src/file.mkv",
            "/dst/file.mkv",
            TaskStatus::Completed,
        );
        insert_result(&conn, &result).unwrap();

        let results = get_results_by_task(&conn, &task.id).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].source_path, "/src/file.mkv");
        assert_eq!(results[0].status, TaskStatus::Completed);
    }

    #[test]
    fn test_get_results_empty() {
        let conn = setup_db();
        let results = get_results_by_task(&conn, "nonexistent-task").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_multiple_results() {
        let conn = setup_db();
        let task = create_task("{Title}", 3);
        insert_task(&conn, &task).unwrap();

        for i in 0..3 {
            let result = create_result(
                &task.id,
                &format!("/src/file{}.mkv", i),
                &format!("/dst/file{}.mkv", i),
                TaskStatus::Completed,
            );
            insert_result(&conn, &result).unwrap();
        }

        let results = get_results_by_task(&conn, &task.id).unwrap();
        assert_eq!(results.len(), 3);
    }
}
