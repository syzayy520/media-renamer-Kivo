// 回滚摘要测试
// 职责：测试回滚摘要统计的准确性

use app_lib::rollback::executor::rollback_entry::{RollbackEntry, RollbackStatus};
use app_lib::rollback::executor::rollback_summary::summarize_rollback;

fn make_rollback_entry(status: RollbackStatus) -> RollbackEntry {
    RollbackEntry {
        rename_result_id: uuid::Uuid::new_v4().to_string(),
        status,
        error_message: None,
    }
}

#[test]
fn test_summary_all_success() {
    let entries: Vec<_> = (0..100)
        .map(|_| make_rollback_entry(RollbackStatus::Success))
        .collect();
    let summary = summarize_rollback(&entries);
    assert_eq!(summary.total, 100);
    assert_eq!(summary.success, 100);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.blocked, 0);
}

#[test]
fn test_summary_mixed_statuses() {
    let mut entries = Vec::new();
    for _ in 0..60 {
        entries.push(make_rollback_entry(RollbackStatus::Success));
    }
    for _ in 0..30 {
        entries.push(make_rollback_entry(RollbackStatus::Blocked));
    }
    for _ in 0..10 {
        entries.push(make_rollback_entry(RollbackStatus::Failed));
    }
    let summary = summarize_rollback(&entries);
    assert_eq!(summary.total, 100);
    assert_eq!(summary.success, 60);
    assert_eq!(summary.blocked, 30);
    assert_eq!(summary.failed, 10);
}

#[test]
fn test_summary_empty() {
    let summary = summarize_rollback(&[]);
    assert_eq!(summary.total, 0);
    assert_eq!(summary.success, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.blocked, 0);
}

#[test]
fn test_summary_1000_entries() {
    let entries: Vec<_> = (0..1000)
        .map(|i| {
            let status = match i % 4 {
                0 => RollbackStatus::Failed,
                1 => RollbackStatus::Blocked,
                _ => RollbackStatus::Success,
            };
            make_rollback_entry(status)
        })
        .collect();
    let summary = summarize_rollback(&entries);
    assert_eq!(summary.total, 1000);
    assert_eq!(summary.success, 500); // indices 2,3 mod 4
    assert_eq!(summary.blocked, 250); // indices 1 mod 4
    assert_eq!(summary.failed, 250); // indices 0 mod 4
}
