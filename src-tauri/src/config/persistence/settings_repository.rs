// settings_repository 模块 - 设置仓库
// 职责：settings 表的 CRUD 操作（SQLite 持久化）
// 表结构：key TEXT PRIMARY KEY, value TEXT NOT NULL, updated_at TEXT NOT NULL

use chrono::Utc;
use rusqlite::Connection;

/// 获取设置值
pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query_map([key], |row| row.get::<_, String>(0))?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

/// 保存设置值（upsert）
pub fn save_setting(conn: &Connection, key: &str, value: &str) -> Result<(), rusqlite::Error> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
        rusqlite::params![key, value, now],
    )?;
    Ok(())
}

/// 删除设置值
pub fn delete_setting(conn: &Connection, key: &str) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM settings WHERE key = ?1", [key])?;
    Ok(())
}

/// 获取所有设置
pub fn get_all_settings(conn: &Connection) -> Result<Vec<(String, String)>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT key, value FROM settings ORDER BY key")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut settings = Vec::new();
    for row in rows {
        settings.push(row?);
    }
    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::connection::create_memory_connection;
    use crate::audit::db::schema::init_tables;

    fn setup_conn() -> Connection {
        let conn = create_memory_connection().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_get_nonexistent_setting() {
        let conn = setup_conn();
        let result = get_setting(&conn, "nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_get_setting() {
        let conn = setup_conn();
        save_setting(&conn, "tmdb_api_key", "sk_test_123").unwrap();
        let value = get_setting(&conn, "tmdb_api_key").unwrap();
        assert_eq!(value.unwrap(), "sk_test_123");
    }

    #[test]
    fn test_upsert_setting() {
        let conn = setup_conn();
        save_setting(&conn, "key1", "value1").unwrap();
        save_setting(&conn, "key1", "value2").unwrap();
        let value = get_setting(&conn, "key1").unwrap();
        assert_eq!(value.unwrap(), "value2");
    }

    #[test]
    fn test_delete_setting() {
        let conn = setup_conn();
        save_setting(&conn, "key1", "value1").unwrap();
        delete_setting(&conn, "key1").unwrap();
        let result = get_setting(&conn, "key1").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_get_all_settings() {
        let conn = setup_conn();
        save_setting(&conn, "b_key", "b_value").unwrap();
        save_setting(&conn, "a_key", "a_value").unwrap();
        let all = get_all_settings(&conn).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].0, "a_key"); // sorted by key
        assert_eq!(all[1].0, "b_key");
    }
}
