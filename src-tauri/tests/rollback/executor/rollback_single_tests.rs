// RollbackSingle 测试
// 职责：测试单条回滚操作

use app_lib::rollback::executor::rollback_task;
use app_lib::rollback::executor::rollback_entry::RollbackStatus;
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
fn test_single_file_rollback_success() {
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
        source.to_string_lossy().to_string().as_str(),
        target.to_string_lossy().to_string().as_str(),
        TaskStatus::Completed,
    );
    db::insert_result(&conn, &result).unwrap();

    let entries = rollback_task(&conn, &task.id, &[result]).unwrap();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].status, RollbackStatus::Success);
    assert!(source.exists());
    assert!(!target.exists());
}

#[test]
fn test_beforepath_occupied_blocks() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("original.mkv");
    let target = tmp.path().join("renamed.mkv");
    fs::write(&source, b"already back").unwrap();
    fs::write(&target, b"renamed").unwrap();

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

    let entries = rollback_task(&conn, &task.id, &[result]).unwrap();

    assert_eq!(entries[0].status, RollbackStatus::Blocked);
    assert!(entries[0]
        .error_message
        .as_ref()
        .unwrap()
        .contains("beforePath already occupied"));
}

#[test]
fn test_afterpath_missing_blocks() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("original.mkv");
    let target = tmp.path().join("nonexistent.mkv");

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

    let entries = rollback_task(&conn, &task.id, &[result]).unwrap();

    assert_eq!(entries[0].status, RollbackStatus::Blocked);
    assert!(entries[0]
        .error_message
        .as_ref()
        .unwrap()
        .contains("afterPath does not exist"));
}