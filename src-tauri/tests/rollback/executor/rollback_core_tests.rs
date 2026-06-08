// RollbackCore 测试
// 职责：测试回滚核心编排

use app_lib::rollback::executor::rollback_task;
use app_lib::rollback::executor::rollback_entry::RollbackStatus;
use app_lib::audit::db::{self, init_tables, RenameResult, TaskStatus};
use app_lib::shared::result_types::AppError;
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
fn test_multi_file_rollback_success() {
    let tmp = TempDir::new().unwrap();
    let s1 = tmp.path().join("orig1.mkv");
    let t1 = tmp.path().join("renamed1.mkv");
    let s2 = tmp.path().join("orig2.mkv");
    let t2 = tmp.path().join("renamed2.mkv");
    fs::write(&t1, b"content1").unwrap();
    fs::write(&t2, b"content2").unwrap();

    let conn = setup_db();
    let task = db::create_task("{Title}", 2);
    db::insert_task(&conn, &task).unwrap();
    db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

    let r1 = make_result(
        &task.id,
        &s1.to_string_lossy(),
        &t1.to_string_lossy(),
        TaskStatus::Completed,
    );
    let r2 = make_result(
        &task.id,
        &s2.to_string_lossy(),
        &t2.to_string_lossy(),
        TaskStatus::Completed,
    );
    db::insert_result(&conn, &r1).unwrap();
    db::insert_result(&conn, &r2).unwrap();

    let entries = rollback_task(&conn, &task.id, &[r1, r2]).unwrap();

    assert_eq!(entries.len(), 2);
    assert!(entries.iter().all(|e| e.status == RollbackStatus::Success));
    assert!(s1.exists());
    assert!(s2.exists());
}

#[test]
fn test_partial_failure_recorded() {
    let tmp = TempDir::new().unwrap();
    let s1 = tmp.path().join("orig1.mkv");
    let t1 = tmp.path().join("renamed1.mkv");
    let s2 = tmp.path().join("orig2.mkv");
    let t2 = tmp.path().join("nonexistent2.mkv"); // 不存在
    fs::write(&t1, b"content1").unwrap();

    let conn = setup_db();
    let task = db::create_task("{Title}", 2);
    db::insert_task(&conn, &task).unwrap();
    db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

    let r1 = make_result(
        &task.id,
        &s1.to_string_lossy(),
        &t1.to_string_lossy(),
        TaskStatus::Completed,
    );
    let r2 = make_result(
        &task.id,
        &s2.to_string_lossy(),
        &t2.to_string_lossy(),
        TaskStatus::Completed,
    );
    db::insert_result(&conn, &r1).unwrap();
    db::insert_result(&conn, &r2).unwrap();

    let entries = rollback_task(&conn, &task.id, &[r1, r2]).unwrap();

    assert_eq!(entries.len(), 2);
    let summary = app_lib::rollback::executor::rollback_summary::summarize_rollback(&entries);
    assert_eq!(summary.success, 1);
    assert_eq!(summary.blocked, 1);
}

#[test]
fn test_does_not_overwrite_existing_file() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("original.mkv");
    let target = tmp.path().join("renamed.mkv");
    fs::write(&source, b"already exists").unwrap();
    fs::write(&target, b"renamed content").unwrap();

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

    // 应被阻塞，不覆盖
    assert_eq!(entries[0].status, RollbackStatus::Blocked);
    // 原文件内容不变
    assert_eq!(fs::read_to_string(&source).unwrap(), "already exists");
}

#[test]
fn test_idempotent_already_rolled_back() {
    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();
    // 任务已回滚
    db::update_task_status(&conn, &task.id, TaskStatus::RolledBack, None).unwrap();

    let result = make_result(&task.id, "a.mkv", "b.mkv", TaskStatus::Completed);

    let err = rollback_task(&conn, &task.id, &[result]).unwrap_err();
    match err {
        AppError::TaskNotRollbackable { status } => {
            assert_eq!(status, "RolledBack");
        }
        other => panic!("Expected TaskNotRollbackable, got {:?}", other),
    }
}