// RollbackAudit 测试
// 职责：测试回滚审计记录

use app_lib::rollback::executor::rollback_task;

use app_lib::audit::db::{self, init_tables, RenameResult, TaskStatus};
use rusqlite::Connection;
use tempfile::TempDir;
use uuid::Uuid;
use chrono::Utc;
use std::fs;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    init_tables(&conn).unwrap();
    conn
}

fn make_result(task_id: &str, source: &str, target: &str, status: TaskStatus) -> RenameResult {
    RenameResult {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        source_path: source.to_string(),
        target_path: target.to_string(),
        status,
        created_at: Utc::now(),
        error_message: None,
    }
}

#[test]
fn test_rollback_writes_audit_log() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("original.mkv");
    let target = tmp.path().join("renamed.mkv");
    fs::write(&target, b"content").unwrap();

    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();
    db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

    let result = make_result(
        &task.id,
        &source.to_string_lossy(),
        &target.to_string_lossy(),
        TaskStatus::Completed,
    );
    db::insert_result(&conn, &result).unwrap();

    let _entries = rollback_task(&conn, &task.id, &[result]).unwrap();

    let logs = db::get_logs_by_task(&conn, &task.id).unwrap();
    assert!(!logs.is_empty());
    assert!(logs
        .iter()
        .any(|l| l.event_type == "rollback_success" || l.event_type == "rollback"));
}