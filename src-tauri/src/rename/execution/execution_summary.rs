// 执行摘要模块
// 职责：聚合执行结果统计
// 不做文件操作，不做审计写入

use crate::audit::db::{RenameResult, TaskStatus};

/// 聚合结果统计
#[derive(Debug, Clone)]
pub struct ExecutionSummary {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub skipped: usize,
}

/// 从结果列表生成统计摘要
pub fn summarize(results: &[RenameResult], total_preview_items: usize) -> ExecutionSummary {
    let success = results
        .iter()
        .filter(|r| r.status == TaskStatus::Completed || r.status == TaskStatus::Pending)
        .count();
    let failed = results
        .iter()
        .filter(|r| r.status == TaskStatus::Failed)
        .count();
    let skipped = total_preview_items.saturating_sub(results.len());

    ExecutionSummary {
        total: total_preview_items,
        success,
        failed,
        skipped,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn make_result(status: TaskStatus) -> RenameResult {
        RenameResult {
            id: Uuid::new_v4().to_string(),
            task_id: "test".to_string(),
            source_path: "a.mkv".to_string(),
            target_path: "b.mkv".to_string(),
            status,
            created_at: Utc::now(),
            error_message: None,
        }
    }

    #[test]
    fn test_summarize_all_success() {
        let results = vec![
            make_result(TaskStatus::Completed),
            make_result(TaskStatus::Pending),
        ];
        let summary = summarize(&results, 2);
        assert_eq!(summary.total, 2);
        assert_eq!(summary.success, 2);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.skipped, 0);
    }

    #[test]
    fn test_summarize_mixed() {
        let results = vec![
            make_result(TaskStatus::Completed),
            make_result(TaskStatus::Failed),
        ];
        let summary = summarize(&results, 3);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.success, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.skipped, 1);
    }

    #[test]
    fn test_summarize_empty() {
        let summary = summarize(&[], 5);
        assert_eq!(summary.total, 5);
        assert_eq!(summary.success, 0);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.skipped, 5);
    }
}
