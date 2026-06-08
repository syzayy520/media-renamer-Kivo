// audit_contract_tests 模块 - 审计命令契约测试
// 职责：验证 audit commands 的参数、返回值、错误分类、安全边界
// 使用内存数据库，禁止真实路径

use app_lib::audit::db;
use app_lib::session::plan_session::DbState;

/// 测试：获取任务 - 存在的任务
#[test]
fn test_get_task_found() {
    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    // 创建任务
    let task = db::task_repository::create_task("{Title}", 5);
    db::task_repository::insert_task(&conn, &task).unwrap();
    drop(conn);

    // 查询任务
    let conn = state.conn.lock().unwrap();
    let found = db::get_task(&conn, &task.id).unwrap();
    assert!(found.is_some());

    let task = found.unwrap();
    assert_eq!(task.template, "{Title}");
    assert_eq!(task.total_files, 5);
}

/// 测试：获取任务 - 不存在的任务
#[test]
fn test_get_task_not_found() {
    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let found = db::get_task(&conn, "nonexistent-task-id").unwrap();
    assert!(found.is_none());
}

/// 测试：获取所有任务 - 空数据库
#[test]
fn test_get_all_tasks_empty() {
    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let mut stmt = conn
        .prepare(
            "SELECT id, status, template, total_files, created_at, updated_at, error_message
             FROM rename_tasks ORDER BY created_at DESC",
        )
        .unwrap();

    let tasks: Vec<db::RenameTask> = stmt
        .query_map([], |row| {
            let status_str: String = row.get(1)?;
            Ok(db::RenameTask {
                id: row.get(0)?,
                status: db::task_status::parse_task_status(&status_str),
                template: row.get(2)?,
                total_files: row.get(3)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                error_message: row.get(6)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    assert!(tasks.is_empty());
}

/// 测试：获取所有任务 - 有多个任务
#[test]
fn test_get_all_tasks_multiple() {
    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    // 创建多个任务
    let task1 = db::task_repository::create_task("{Title}", 5);
    let task2 = db::task_repository::create_task("{Series} S{Season}E{Episode}", 10);
    db::task_repository::insert_task(&conn, &task1).unwrap();
    db::task_repository::insert_task(&conn, &task2).unwrap();

    // 查询所有任务
    let mut stmt = conn
        .prepare(
            "SELECT id, status, template, total_files, created_at, updated_at, error_message
             FROM rename_tasks ORDER BY created_at DESC",
        )
        .unwrap();

    let tasks: Vec<db::RenameTask> = stmt
        .query_map([], |row| {
            let status_str: String = row.get(1)?;
            Ok(db::RenameTask {
                id: row.get(0)?,
                status: db::task_status::parse_task_status(&status_str),
                template: row.get(2)?,
                total_files: row.get(3)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                error_message: row.get(6)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    assert_eq!(tasks.len(), 2);
}

/// 测试：获取审计日志 - 空结果
#[test]
fn test_get_audit_logs_empty() {
    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let logs = db::get_logs_by_task(&conn, "nonexistent-task").unwrap();
    assert!(logs.is_empty());
}

/// 测试：获取审计日志 - 有日志
#[test]
fn test_get_audit_logs_with_entries() {
    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    // 创建任务
    let task = db::task_repository::create_task("{Title}", 5);
    db::task_repository::insert_task(&conn, &task).unwrap();

    // 写入审计日志
    let log = db::AuditLogEntry {
        id: uuid::Uuid::new_v4().to_string(),
        task_id: Some(task.id.clone()),
        event_type: "scan_complete".to_string(),
        message: "Scan completed successfully".to_string(),
        created_at: chrono::Utc::now(),
    };
    db::log_repository::insert_log(&conn, &log).unwrap();

    // 查询日志
    let logs = db::get_logs_by_task(&conn, &task.id).unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].event_type, "scan_complete");
}

/// 测试：审计日志 - 敏感字段脱敏
#[test]
fn test_audit_log_sensitive_data_redacted() {
    // 审计日志中的 API Key 应该被脱敏
    // 这个测试验证 redaction 模块的行为
    let sensitive = "api_key=abcdefghijklmnop"; // 16+ 字符的 API Key
    let redacted = app_lib::audit::redaction::redact_text(sensitive);

    // 不应该包含原始 API Key
    assert!(!redacted.contains("abcdefghijklmnop"));
    assert!(redacted.contains("REDACTED"));
}

/// 测试：任务状态解析
#[test]
fn test_task_status_parse() {
    use app_lib::audit::db::task_status::{parse_task_status, TaskStatus};

    assert_eq!(parse_task_status("Previewing"), TaskStatus::Previewing);
    assert_eq!(parse_task_status("Pending"), TaskStatus::Pending);
    assert_eq!(parse_task_status("Executing"), TaskStatus::Executing);
    assert_eq!(parse_task_status("Completed"), TaskStatus::Completed);
    assert_eq!(parse_task_status("Failed"), TaskStatus::Failed);
    assert_eq!(parse_task_status("RolledBack"), TaskStatus::RolledBack);
}

/// 测试：任务状态显示
#[test]
fn test_task_status_display() {
    use app_lib::audit::db::task_status::TaskStatus;

    assert_eq!(format!("{}", TaskStatus::Previewing), "Previewing");
    assert_eq!(format!("{}", TaskStatus::Completed), "Completed");
}

/// 测试：无效状态处理
#[test]
fn test_invalid_status_handling() {
    use app_lib::audit::db::task_status::{parse_task_status, TaskStatus};

    // 无效状态应该返回 Failed（默认值）
    let status = parse_task_status("InvalidStatus");
    assert_eq!(status, TaskStatus::Failed);
}
