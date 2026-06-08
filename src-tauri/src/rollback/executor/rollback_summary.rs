// 回滚汇总模块
// 职责：汇总回滚结果
// 不做回滚操作，不做审计记录

use super::rollback_entry::{RollbackEntry, RollbackStatus};

/// 回滚汇总结果
#[derive(Debug, Clone)]
pub struct RollbackSummary {
    pub success: usize,
    pub failed: usize,
    pub blocked: usize,
    pub total: usize,
}

/// 汇总回滚结果
pub fn summarize_rollback(entries: &[RollbackEntry]) -> RollbackSummary {
    let success = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Success)
        .count();
    let failed = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Failed)
        .count();
    let blocked = entries
        .iter()
        .filter(|e| e.status == RollbackStatus::Blocked)
        .count();

    RollbackSummary {
        success,
        failed,
        blocked,
        total: entries.len(),
    }
}
