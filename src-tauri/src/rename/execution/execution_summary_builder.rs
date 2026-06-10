// 执行摘要构建器模块
// 职责：构建执行输出摘要
// 不做执行逻辑，不做安全检查

use super::execution_contract::{ExecutionItemResult, ExecutionItemStatus, ExecutionOutputSummary};
use super::execution_mode::ExecutionMode;
use crate::rollback::rollback_plan::RollbackPlan;

/// 构建执行摘要
///
/// 从执行结果列表构建摘要统计
pub fn build_execution_summary(
    item_results: &[ExecutionItemResult],
    total_items: usize,
    skipped_count: usize,
    mode: ExecutionMode,
    rollback_plan: &RollbackPlan,
) -> ExecutionOutputSummary {
    let success = item_results
        .iter()
        .filter(|r| r.status == ExecutionItemStatus::Success)
        .count();
    let failed = item_results
        .iter()
        .filter(|r| r.status == ExecutionItemStatus::Failed)
        .count();
    let blocked = item_results
        .iter()
        .filter(|r| r.status == ExecutionItemStatus::Blocked)
        .count();

    ExecutionOutputSummary {
        total: total_items,
        success,
        failed,
        skipped: skipped_count,
        blocked,
        mode,
        has_rollback_plan: rollback_plan.has_rollbackable_entries(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rollback::rollback_plan::RollbackPlanEntry;
    use chrono::Utc;

    fn make_result(status: ExecutionItemStatus) -> ExecutionItemResult {
        ExecutionItemResult {
            source_path: "/s".to_string(),
            target_path: "/t".to_string(),
            status,
            error: None,
            rollback_entry: None,
        }
    }

    fn make_plan(entries: usize) -> RollbackPlan {
        RollbackPlan {
            task_id: "test".to_string(),
            entries: (0..entries)
                .map(|i| RollbackPlanEntry {
                    source_path: format!("/s{}", i),
                    target_path: format!("/t{}", i),
                    rollback_source: format!("/t{}", i),
                    rollback_target: format!("/s{}", i),
                    executed: true,
                })
                .collect(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_build_summary_all_success() {
        let results = vec![
            make_result(ExecutionItemStatus::Success),
            make_result(ExecutionItemStatus::Success),
        ];
        let plan = make_plan(2);
        let summary = build_execution_summary(&results, 2, 0, ExecutionMode::DryRun, &plan);
        assert_eq!(summary.total, 2);
        assert_eq!(summary.success, 2);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.skipped, 0);
        assert_eq!(summary.blocked, 0);
    }

    #[test]
    fn test_build_summary_mixed() {
        let results = vec![
            make_result(ExecutionItemStatus::Success),
            make_result(ExecutionItemStatus::Failed),
            make_result(ExecutionItemStatus::Blocked),
        ];
        let plan = make_plan(1);
        let summary = build_execution_summary(&results, 4, 1, ExecutionMode::Confirmed, &plan);
        assert_eq!(summary.total, 4);
        assert_eq!(summary.success, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.skipped, 1);
        assert_eq!(summary.blocked, 1);
        assert!(summary.has_rollback_plan);
    }

    #[test]
    fn test_build_summary_empty() {
        let plan = RollbackPlan {
            task_id: "test".to_string(),
            entries: vec![],
            created_at: Utc::now(),
        };
        let summary = build_execution_summary(&[], 0, 0, ExecutionMode::DryRun, &plan);
        assert_eq!(summary.total, 0);
        assert!(!summary.has_rollback_plan);
    }
}
