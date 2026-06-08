// SQLite 连接模块
// 职责：创建和管理 SQLite 连接
// 不做建表，不做 CRUD

use rusqlite::Connection;

/// 创建内存数据库连接（用于测试）
pub fn create_memory_connection() -> Result<Connection, rusqlite::Error> {
    Connection::open_in_memory()
}

/// 创建或打开文件数据库连接
pub fn open_connection(path: &str) -> Result<Connection, rusqlite::Error> {
    Connection::open(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_connection() {
        let conn = create_memory_connection();
        assert!(conn.is_ok());
    }

    #[test]
    fn test_connection_open() {
        let tmp = tempfile::TempDir::new().unwrap();
        let db_path = tmp.path().join("test.db");
        let conn = open_connection(db_path.to_str().unwrap());
        assert!(conn.is_ok());
    }
}
