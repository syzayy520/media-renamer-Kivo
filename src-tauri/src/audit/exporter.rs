// 审计日志导出模块
// 职责：导出 JSONL 格式审计日志，导出前必须 redaction
// 不导出密钥

use rusqlite::Connection;
use serde_json;

use super::db::{self, AuditLogEntry};
use super::redaction::redact_text;

/// JSONL 导出器
pub struct JsonlExporter<'a> {
    conn: &'a Connection,
}

impl<'a> JsonlExporter<'a> {
    /// 创建新的导出器
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// 导出所有审计日志为 JSONL 格式
    ///
    /// 每行一个 JSON 对象，敏感字段已脱敏
    pub fn export_all(&self) -> Result<String, Box<dyn std::error::Error>> {
        let logs = db::get_all_logs(self.conn)?;
        self.export_entries(&logs)
    }

    /// 导出指定任务的审计日志
    pub fn export_by_task(&self, task_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let logs = db::get_logs_by_task(self.conn, task_id)?;
        self.export_entries(&logs)
    }

    /// 将日志条目导出为 JSONL 格式
    fn export_entries(
        &self,
        entries: &[AuditLogEntry],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut lines = Vec::with_capacity(entries.len());

        for entry in entries {
            // 对 message 进行脱敏
            let mut redacted_entry = entry.clone();
            redacted_entry.message = redact_text(&entry.message);

            let json = serde_json::to_string(&redacted_entry)?;
            lines.push(json);
        }

        Ok(lines.join("\n"))
    }
}

/// 将单条日志导出为 JSON 字符串（已脱敏）
pub fn entry_to_json(entry: &AuditLogEntry) -> Result<String, serde_json::Error> {
    let mut redacted = entry.clone();
    redacted.message = redact_text(&entry.message);
    serde_json::to_string(&redacted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::db::{create_log_entry, init_tables, insert_log};

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_export_jsonl_format() {
        let conn = setup();
        let entry = create_log_entry(None, "test", "Test message");
        insert_log(&conn, &entry).unwrap();

        let exporter = JsonlExporter::new(&conn);
        let jsonl = exporter.export_all().unwrap();

        // 应该是有效的 JSONL（每行一个 JSON）
        for line in jsonl.lines() {
            let _: serde_json::Value = serde_json::from_str(line).unwrap();
        }
    }

    #[test]
    fn test_export_multiple_logs() {
        let conn = setup();
        let entry1 = create_log_entry(None, "event1", "Message 1");
        let entry2 = create_log_entry(None, "event2", "Message 2");
        insert_log(&conn, &entry1).unwrap();
        insert_log(&conn, &entry2).unwrap();

        let exporter = JsonlExporter::new(&conn);
        let jsonl = exporter.export_all().unwrap();

        let lines: Vec<&str> = jsonl.lines().collect();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_export_redacts_sensitive_data() {
        let conn = setup();
        let entry = create_log_entry(None, "config", "api_key: abcdef123456789012345678");
        insert_log(&conn, &entry).unwrap();

        let exporter = JsonlExporter::new(&conn);
        let jsonl = exporter.export_all().unwrap();

        assert!(!jsonl.contains("abcdef123456789012345678"));
        assert!(jsonl.contains("***REDACTED***"));
    }

    #[test]
    fn test_export_empty_logs() {
        let conn = setup();
        let exporter = JsonlExporter::new(&conn);
        let jsonl = exporter.export_all().unwrap();

        assert!(jsonl.is_empty());
    }

    #[test]
    fn test_export_by_task() {
        let conn = setup();
        let entry1 = create_log_entry(Some("task-1"), "event", "For task 1");
        let entry2 = create_log_entry(Some("task-2"), "event", "For task 2");
        insert_log(&conn, &entry1).unwrap();
        insert_log(&conn, &entry2).unwrap();

        let exporter = JsonlExporter::new(&conn);
        let jsonl = exporter.export_by_task("task-1").unwrap();

        assert!(jsonl.contains("For task 1"));
        assert!(!jsonl.contains("For task 2"));
    }
}
