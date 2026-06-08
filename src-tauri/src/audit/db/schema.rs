// 数据库 Schema 模块
// 职责：初始化数据库表结构
// 不做 insert / update / query

use rusqlite::Connection;

/// 初始化数据库表
///
/// 创建 rename_tasks、rename_results、audit_log 三张表
/// 幂等操作，重复调用不会报错
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

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

/// 检查表是否存在
pub fn tables_exist(conn: &Connection) -> Result<bool, rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='rename_tasks'",
        [],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::connection::create_memory_connection;

    #[test]
    fn test_init_tables() {
        let conn = create_memory_connection().unwrap();
        let result = init_tables(&conn);
        assert!(result.is_ok());

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
    fn test_tables_exist() {
        let conn = create_memory_connection().unwrap();

        // 初始化前表不存在
        assert!(!tables_exist(&conn).unwrap());

        // 初始化后表存在
        init_tables(&conn).unwrap();
        assert!(tables_exist(&conn).unwrap());
    }

    #[test]
    fn test_repeated_init_is_idempotent() {
        let conn = create_memory_connection().unwrap();

        // 多次初始化不应报错
        assert!(init_tables(&conn).is_ok());
        assert!(init_tables(&conn).is_ok());
        assert!(init_tables(&conn).is_ok());
    }
}
