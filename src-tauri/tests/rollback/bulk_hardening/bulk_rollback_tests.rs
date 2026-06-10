// 批量回滚测试
// 职责：测试大规模回滚的正确性

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
fn test_50_file_rollback_success() {
    let tmp = TempDir::new().unwrap();
    let conn = setup_db();
    let task = db::create_task("{Title}", 50);
    db::insert_task(&conn, &task).unwrap();
    db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

    let mut results = Vec::new();
    for i in 0..50 {
        let source = tmp.path().join(format!("orig_{}.mkv", i));
        let target = tmp.path().join(format!("renamed_{}.mkv", i));
        fs::write(&target, format!("content_{}", i)).unwrap();

        let result = make_result(
            &task.id,
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &result).unwrap();
        results.push(result);
    }

    let entries = rollback_task(&conn, &task.id, &results).unwrap();
    assert_eq!(entries.len(), 50);
    assert!(entries.iter().all(|e| e.status == RollbackStatus::Success));
}

#[test]
fn test_100_file_rollback_partial_failure() {
    let tmp = TempDir::new().unwrap();
    let conn = setup_db();
    let task = db::create_task("{Title}", 100);
    db::insert_task(&conn, &task).unwrap();
    db::update_task_status(&conn, &task.id, TaskStatus::Completed, None).unwrap();

    let mut results = Vec::new();
    for i in 0..100 {
        let source = tmp.path().join(format!("orig_{}.mkv", i));
        let target = tmp.path().join(format!("renamed_{}.mkv", i));
        // Only create some files — others will fail rollback (source doesn't exist)
        if i % 3 == 0 {
            fs::write(&target, format!("content_{}", i)).unwrap();
        }

        let result = make_result(
            &task.id,
            &source.to_string_lossy(),
            &target.to_string_lossy(),
            TaskStatus::Completed,
        );
        db::insert_result(&conn, &result).unwrap();
        results.push(result);
    }

    let entries = rollback_task(&conn, &task.id, &results).unwrap();
    assert_eq!(entries.len(), 100);
    
    let success_count = entries.iter().filter(|e| e.status == RollbackStatus::Success).count();
    let blocked_count = entries.iter().filter(|e| e.status == RollbackStatus::Blocked).count();
    // ~33 files exist (i%3==0), ~67 don't exist (blocked)
    assert!(success_count > 0);
    assert!(blocked_count > 0);
    assert_eq!(success_count + blocked_count, 100);
}
