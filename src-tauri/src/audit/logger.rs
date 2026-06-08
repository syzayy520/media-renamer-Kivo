// 审计日志写入模块
// 职责：统一写入审计事件，调用 redaction 脱敏
// 不直接拼 SQL，通过 db 模块操作

use rusqlite::Connection;

use super::db::{self, AuditLogEntry};
use super::redaction::redact_text;

/// 审计日志器
pub struct AuditLogger<'a> {
    conn: &'a Connection,
}

impl<'a> AuditLogger<'a> {
    /// 创建新的审计日志器
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// 写入普通事件
    pub fn log_event(
        &self,
        task_id: Option<&str>,
        event_type: &str,
        message: &str,
    ) -> Result<AuditLogEntry, rusqlite::Error> {
        let redacted_message = redact_text(message);
        let entry = db::create_log_entry(task_id, event_type, &redacted_message);
        db::insert_log(self.conn, &entry)?;
        Ok(entry)
    }

    /// 写入失败事件
    pub fn log_failure(
        &self,
        task_id: Option<&str>,
        error: &str,
    ) -> Result<AuditLogEntry, rusqlite::Error> {
        let redacted_error = redact_text(error);
        let entry = db::create_log_entry(task_id, "failure", &redacted_error);
        db::insert_log(self.conn, &entry)?;
        Ok(entry)
    }

    /// 写入 preview 事件
    pub fn log_preview(
        &self,
        task_id: &str,
        file_count: u32,
    ) -> Result<AuditLogEntry, rusqlite::Error> {
        let message = format!("Preview generated for {} files", file_count);
        let entry = db::create_log_entry(Some(task_id), "preview", &message);
        db::insert_log(self.conn, &entry)?;
        Ok(entry)
    }

    /// 写入任务创建事件
    pub fn log_task_created(
        &self,
        task_id: &str,
        template: &str,
    ) -> Result<AuditLogEntry, rusqlite::Error> {
        let redacted_template = redact_text(template);
        let message = format!("Task created with template: {}", redacted_template);
        let entry = db::create_log_entry(Some(task_id), "task_created", &message);
        db::insert_log(self.conn, &entry)?;
        Ok(entry)
    }

    /// 写入执行计划事件
    pub fn log_execution_plan(
        &self,
        task_id: &str,
        file_count: u32,
    ) -> Result<AuditLogEntry, rusqlite::Error> {
        let message = format!("Execution plan: {} files to rename", file_count);
        let entry = db::create_log_entry(Some(task_id), "execution_plan", &message);
        db::insert_log(self.conn, &entry)?;
        Ok(entry)
    }

    /// 获取连接引用
    pub fn conn(&self) -> &Connection {
        self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::init_tables;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_log_event() {
        let conn = setup();
        let logger = AuditLogger::new(&conn);

        let entry = logger.log_event(None, "system", "System started").unwrap();
        assert_eq!(entry.event_type, "system");
        assert_eq!(entry.message, "System started");
    }

    #[test]
    fn test_log_failure() {
        let conn = setup();
        let logger = AuditLogger::new(&conn);

        let entry = logger.log_failure(None, "Connection timeout").unwrap();
        assert_eq!(entry.event_type, "failure");
        assert_eq!(entry.message, "Connection timeout");
    }

    #[test]
    fn test_log_preview() {
        let conn = setup();
        let logger = AuditLogger::new(&conn);

        let entry = logger.log_preview("task-123", 42).unwrap();
        assert_eq!(entry.event_type, "preview");
        assert!(entry.message.contains("42"));
        assert_eq!(entry.task_id, Some("task-123".to_string()));
    }

    #[test]
    fn test_log_does_not_leak_key() {
        let conn = setup();
        let logger = AuditLogger::new(&conn);

        let message = "Config loaded: tmdb_api_key: abcdef1234567890abcdef1234567890";
        let entry = logger.log_event(None, "config", message).unwrap();

        assert!(!entry.message.contains("abcdef1234567890abcdef1234567890"));
        assert!(entry.message.contains("***REDACTED***"));
    }

    #[test]
    fn test_log_error_handling() {
        let conn = setup();
        let logger = AuditLogger::new(&conn);

        // 正常写入应该成功
        let result = logger.log_event(None, "test", "test message");
        assert!(result.is_ok());
    }

    #[test]
    fn test_log_task_created() {
        let conn = setup();
        let logger = AuditLogger::new(&conn);

        let entry = logger
            .log_task_created("task-456", "{Title} ({Year})")
            .unwrap();
        assert_eq!(entry.event_type, "task_created");
        assert!(entry.message.contains("Task created"));
        assert!(entry.message.contains("{Title}"));
    }
}
