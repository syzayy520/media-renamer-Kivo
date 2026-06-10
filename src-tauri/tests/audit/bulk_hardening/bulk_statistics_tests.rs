// 审计批量统计测试
// 职责：测试审计日志在大规模数据下的统计正确性

use app_lib::audit::db::{self, init_tables, AuditLogEntry};
use rusqlite::Connection;

fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    init_tables(&conn).unwrap();
    conn
}

fn make_log_entry(task_id: &str, event_type: &str, message: &str) -> AuditLogEntry {
    db::create_log_entry(Some(task_id), event_type, message)
}

#[test]
fn test_100_logs_count() {
    let conn = setup();
    for i in 0..100 {
        let entry = make_log_entry(&format!("task-{}", i % 10), "rename", &format!("Renamed file {}", i));
        db::insert_log(&conn, &entry).unwrap();
    }
    let all = db::get_all_logs(&conn).unwrap();
    assert_eq!(all.len(), 100);
}

#[test]
fn test_1000_logs_count() {
    let conn = setup();
    for i in 0..1000 {
        let entry = make_log_entry(&format!("task-{}", i % 5), "event", &format!("Event {}", i));
        db::insert_log(&conn, &entry).unwrap();
    }
    let all = db::get_all_logs(&conn).unwrap();
    assert_eq!(all.len(), 1000);
}

#[test]
fn test_logs_by_task_filtering() {
    let conn = setup();
    for i in 0..100 {
        let task_id = if i < 60 { "task-a" } else { "task-b" };
        let entry = make_log_entry(task_id, "event", &format!("Event {}", i));
        db::insert_log(&conn, &entry).unwrap();
    }
    let task_a = db::get_logs_by_task(&conn, "task-a").unwrap();
    let task_b = db::get_logs_by_task(&conn, "task-b").unwrap();
    assert_eq!(task_a.len(), 60);
    assert_eq!(task_b.len(), 40);
}

#[test]
fn test_5000_logs_count() {
    let conn = setup();
    for i in 0..5000 {
        let entry = make_log_entry("bulk-task", "bulk_event", &format!("Bulk event {}", i));
        db::insert_log(&conn, &entry).unwrap();
    }
    let all = db::get_all_logs(&conn).unwrap();
    assert_eq!(all.len(), 5000);
    let task_logs = db::get_logs_by_task(&conn, "bulk-task").unwrap();
    assert_eq!(task_logs.len(), 5000);
}
