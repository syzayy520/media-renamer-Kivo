// 审计导出批量测试
// 职责：测试 JSONL 导出在大规模数据下的正确性

use app_lib::audit::db::{self, init_tables};
use app_lib::audit::exporter::JsonlExporter;
use rusqlite::Connection;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    init_tables(&conn).unwrap();
    conn
}

#[test]
fn test_export_100_logs() {
    let conn = setup();
    for i in 0..100 {
        let entry = db::create_log_entry(Some("task-1"), "event", &format!("Message {}", i));
        db::insert_log(&conn, &entry).unwrap();
    }
    let exporter = JsonlExporter::new(&conn);
    let jsonl = exporter.export_all().unwrap();
    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(lines.len(), 100);
    // Each line should be valid JSON
    for line in &lines {
        let _: serde_json::Value = serde_json::from_str(line).unwrap();
    }
}

#[test]
fn test_export_1000_logs() {
    let conn = setup();
    for i in 0..1000 {
        let entry = db::create_log_entry(Some("bulk"), "event", &format!("Event {}", i));
        db::insert_log(&conn, &entry).unwrap();
    }
    let exporter = JsonlExporter::new(&conn);
    let jsonl = exporter.export_all().unwrap();
    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(lines.len(), 1000);
}

#[test]
fn test_export_by_task_subset() {
    let conn = setup();
    for i in 0..100 {
        let task = if i < 30 { "task-a" } else { "task-b" };
        let entry = db::create_log_entry(Some(task), "event", &format!("Event {}", i));
        db::insert_log(&conn, &entry).unwrap();
    }
    let exporter = JsonlExporter::new(&conn);
    let jsonl = exporter.export_by_task("task-a").unwrap();
    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(lines.len(), 30);
}

#[test]
fn test_export_empty() {
    let conn = setup();
    let exporter = JsonlExporter::new(&conn);
    let jsonl = exporter.export_all().unwrap();
    assert!(jsonl.is_empty());
}
