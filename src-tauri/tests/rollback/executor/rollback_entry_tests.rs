// RollbackEntry 测试
// 职责：测试 RollbackEntry 定义

use app_lib::rollback::executor::rollback_entry::{RollbackEntry, RollbackStatus, summarize_rollback};

#[test]
fn test_rollback_entry_creation() {
    let entry = RollbackEntry {
        rename_result_id: "test-id".to_string(),
        status: RollbackStatus::Success,
        error_message: None,
    };

    assert_eq!(entry.rename_result_id, "test-id");
    assert_eq!(entry.status, RollbackStatus::Success);
    assert!(entry.error_message.is_none());
}

#[test]
fn test_rollback_status_equality() {
    assert_eq!(RollbackStatus::Success, RollbackStatus::Success);
    assert_eq!(RollbackStatus::Failed, RollbackStatus::Failed);
    assert_eq!(RollbackStatus::Blocked, RollbackStatus::Blocked);
    assert_ne!(RollbackStatus::Success, RollbackStatus::Failed);
}

#[test]
fn test_summarize_rollback() {
    let entries = vec![
        RollbackEntry {
            rename_result_id: "1".to_string(),
            status: RollbackStatus::Success,
            error_message: None,
        },
        RollbackEntry {
            rename_result_id: "2".to_string(),
            status: RollbackStatus::Failed,
            error_message: Some("error".to_string()),
        },
        RollbackEntry {
            rename_result_id: "3".to_string(),
            status: RollbackStatus::Blocked,
            error_message: Some("blocked".to_string()),
        },
    ];

    let summary = summarize_rollback(&entries);
    assert_eq!(summary.success, 1);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.blocked, 1);
    assert_eq!(summary.total, 3);
}