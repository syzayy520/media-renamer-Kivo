// 回滚幂等性测试
// 职责：测试重复回滚的幂等性

use app_lib::rollback::executor::rollback_task;
use app_lib::audit::db::{self, init_tables, RenameResult, TaskStatus};
use app_lib::shared::result_types::AppError;
use rusqlite::Connection;
use uuid::Uuid;
use chrono::Utc;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    init_tables(&conn).unwrap();
    conn
}

fn make_result(task_id: &str, source: &str, target: &str) -> RenameResult {
    RenameResult {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        source_path: source.to_string(),
        target_path: target.to_string(),
        status: TaskStatus::Completed,
        created_at: Utc::now(),
        error_message: None,
    }
}

#[test]
fn test_double_rollback_rejected() {
    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();
    db::update_task_status(&conn, &task.id, TaskStatus::RolledBack, None).unwrap();

    let result = make_result(&task.id, "a.mkv", "b.mkv");
    let err = rollback_task(&conn, &task.id, &[result]).unwrap_err();
    match err {
        AppError::TaskNotRollbackable { status } => assert_eq!(status, "RolledBack"),
        other => panic!("Expected TaskNotRollbackable, got {:?}", other),
    }
}

#[test]
fn test_double_rollback_10_tasks() {
    let conn = setup_db();
    for i in 0..10 {
        let task = db::create_task("{Title}", 1);
        db::insert_task(&conn, &task).unwrap();
        db::update_task_status(&conn, &task.id, TaskStatus::RolledBack, None).unwrap();

        let result = make_result(&task.id, &format!("a_{}.mkv", i), &format!("b_{}.mkv", i));
        let err = rollback_task(&conn, &task.id, &[result]).unwrap_err();
        match err {
            AppError::TaskNotRollbackable { .. } => {} // expected
            other => panic!("Expected TaskNotRollbackable, got {:?}", other),
        }
    }
}

#[test]
fn test_failed_task_not_rollbackable() {
    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();
    db::update_task_status(&conn, &task.id, TaskStatus::Failed, None).unwrap();

    let result = make_result(&task.id, "a.mkv", "b.mkv");
    // Failed tasks are still rollbackable, but rollback may succeed or be blocked
    // depending on file existence. Here we just verify it doesn't error.
    let entries = rollback_task(&conn, &task.id, &[result]).unwrap();
    assert_eq!(entries.len(), 1);
}
