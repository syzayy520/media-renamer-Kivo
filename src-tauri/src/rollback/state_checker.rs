// 回滚状态检查模块
// 职责：判断任务/结果是否可回滚
// 不执行文件操作，只检查状态和路径存在性

use crate::audit::db::{RenameResult, TaskStatus};
use std::path::Path;

/// 回滚可行性报告
#[derive(Debug, Clone)]
pub struct RollbackReport {
    /// 是否可回滚
    pub can_rollback: bool,
    /// 可回滚的结果列表
    pub rollbackable_results: Vec<RenameResult>,
    /// 不可回滚的结果及原因
    pub blocked_results: Vec<BlockedRollbackEntry>,
    /// 阻塞原因汇总
    pub blocking_reasons: Vec<String>,
}

/// 被阻塞的回滚条目
#[derive(Debug, Clone)]
pub struct BlockedRollbackEntry {
    pub result: RenameResult,
    pub reason: String,
}

/// 检查任务的回滚可行性
///
/// 规则：
/// - 只有 Completed 状态的 rename_result 可回滚
/// - afterPath 必须存在（已被重命名的文件）
/// - beforePath 不能被占用（回滚目标不能已有文件）
pub fn check_rollback(results: &[RenameResult]) -> RollbackReport {
    let mut rollbackable = Vec::new();
    let mut blocked = Vec::new();
    let mut blocking_reasons = Vec::new();

    for result in results {
        // 只有 Completed 状态可回滚
        if result.status != TaskStatus::Completed {
            blocked.push(BlockedRollbackEntry {
                result: result.clone(),
                reason: format!("Status is {}, not Completed", result.status),
            });
            continue;
        }

        // 检查 afterPath 是否存在
        if !Path::new(&result.target_path).exists() {
            blocked.push(BlockedRollbackEntry {
                result: result.clone(),
                reason: format!("afterPath does not exist: {}", result.target_path),
            });
            blocking_reasons.push(format!("afterPath missing: {}", result.target_path));
            continue;
        }

        // 检查 beforePath 是否被占用
        if Path::new(&result.source_path).exists() {
            blocked.push(BlockedRollbackEntry {
                result: result.clone(),
                reason: format!("beforePath already occupied: {}", result.source_path),
            });
            blocking_reasons.push(format!("beforePath occupied: {}", result.source_path));
            continue;
        }

        // 通过所有检查，可回滚
        rollbackable.push(result.clone());
    }

    RollbackReport {
        can_rollback: !rollbackable.is_empty() && blocked.is_empty(),
        rollbackable_results: rollbackable,
        blocked_results: blocked,
        blocking_reasons,
    }
}

/// 检查单条结果是否可回滚
pub fn can_rollback_single(result: &RenameResult) -> bool {
    if result.status != TaskStatus::Completed {
        return false;
    }
    if !Path::new(&result.target_path).exists() {
        return false;
    }
    if Path::new(&result.source_path).exists() {
        return false;
    }
    true
}

/// 检查任务状态是否允许回滚
pub fn is_task_rollbackable(task_status: &TaskStatus) -> bool {
    matches!(task_status, TaskStatus::Completed | TaskStatus::Failed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::fs;
    use tempfile::TempDir;

    fn make_result(source: &str, target: &str, status: TaskStatus) -> RenameResult {
        RenameResult {
            id: uuid::Uuid::new_v4().to_string(),
            task_id: "test-task".to_string(),
            source_path: source.to_string(),
            target_path: target.to_string(),
            status,
            created_at: Utc::now(),
            error_message: None,
        }
    }

    #[test]
    fn test_rollbackable_state() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");

        // target 存在，source 不存在 -> 可回滚
        fs::write(&target, b"content").unwrap();

        let result = make_result(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            TaskStatus::Completed,
        );

        let report = check_rollback(&[result]);
        assert!(report.can_rollback);
        assert_eq!(report.rollbackable_results.len(), 1);
        assert!(report.blocked_results.is_empty());
    }

    #[test]
    fn test_afterpath_not_exist_blocks() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("nonexistent.mkv");

        // target 不存在 -> 不可回滚
        let result = make_result(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            TaskStatus::Completed,
        );

        let report = check_rollback(&[result]);
        assert!(!report.can_rollback);
        assert_eq!(report.blocked_results.len(), 1);
        assert!(report.blocked_results[0]
            .reason
            .contains("afterPath does not exist"));
    }

    #[test]
    fn test_beforepath_occupied_blocks() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");

        // source 和 target 都存在 -> beforePath 被占用
        fs::write(&source, b"original").unwrap();
        fs::write(&target, b"renamed").unwrap();

        let result = make_result(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            TaskStatus::Completed,
        );

        let report = check_rollback(&[result]);
        assert!(!report.can_rollback);
        assert!(report.blocked_results[0]
            .reason
            .contains("beforePath already occupied"));
    }

    #[test]
    fn test_failed_rename_not_rollbackable() {
        let result = make_result("a.mkv", "b.mkv", TaskStatus::Failed);

        let report = check_rollback(&[result]);
        assert!(!report.can_rollback);
        assert_eq!(report.blocked_results.len(), 1);
        assert!(report.blocked_results[0].reason.contains("not Completed"));
    }

    #[test]
    fn test_partial_rollbackable() {
        let tmp = TempDir::new().unwrap();
        let source1 = tmp.path().join("orig1.mkv");
        let target1 = tmp.path().join("renamed1.mkv");
        let source2 = tmp.path().join("orig2.mkv");
        let target2 = tmp.path().join("renamed2.mkv");

        // 第一个可回滚
        fs::write(&target1, b"content1").unwrap();
        // 第二个 target 不存在 -> 不可回滚

        let r1 = make_result(
            source1.to_string_lossy().to_string().as_str(),
            target1.to_string_lossy().to_string().as_str(),
            TaskStatus::Completed,
        );
        let r2 = make_result(
            source2.to_string_lossy().to_string().as_str(),
            target2.to_string_lossy().to_string().as_str(),
            TaskStatus::Completed,
        );

        let report = check_rollback(&[r1, r2]);
        assert!(!report.can_rollback); // 有阻塞项
        assert_eq!(report.rollbackable_results.len(), 1);
        assert_eq!(report.blocked_results.len(), 1);
    }

    #[test]
    fn test_no_results_not_rollbackable() {
        let report = check_rollback(&[]);
        assert!(!report.can_rollback);
        assert!(report.rollbackable_results.is_empty());
    }

    #[test]
    fn test_can_rollback_single() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("original.mkv");
        let target = tmp.path().join("renamed.mkv");
        fs::write(&target, b"content").unwrap();

        let result = make_result(
            source.to_string_lossy().to_string().as_str(),
            target.to_string_lossy().to_string().as_str(),
            TaskStatus::Completed,
        );

        assert!(can_rollback_single(&result));
    }
}
