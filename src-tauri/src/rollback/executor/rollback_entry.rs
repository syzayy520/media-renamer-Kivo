// 回滚条目模块
// 职责：定义 RollbackEntry、RollbackStatus、RollbackSummary 和统计函数
// 不做文件操作，不做审计写入

/// 回滚结果
#[derive(Debug, Clone)]
pub struct RollbackEntry {
    /// 原始 rename_result 的 ID
    pub rename_result_id: String,
    /// 回滚状态
    pub status: RollbackStatus,
    /// 错误信息（如果失败）
    pub error_message: Option<String>,
}

/// 回滚状态
#[derive(Debug, Clone, PartialEq)]
pub enum RollbackStatus {
    /// 回滚成功
    Success,
    /// 回滚失败
    Failed,
    /// 被阻塞（beforePath 被占用等）
    Blocked,
}

/// 聚合回滚统计
#[derive(Debug, Clone)]
pub struct RollbackSummary {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub blocked: usize,
}

/// 从回滚条目生成统计
pub fn summarize_rollback(entries: &[RollbackEntry]) -> RollbackSummary {
    RollbackSummary {
        total: entries.len(),
        success: entries
            .iter()
            .filter(|e| e.status == RollbackStatus::Success)
            .count(),
        failed: entries
            .iter()
            .filter(|e| e.status == RollbackStatus::Failed)
            .count(),
        blocked: entries
            .iter()
            .filter(|e| e.status == RollbackStatus::Blocked)
            .count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(status: RollbackStatus) -> RollbackEntry {
        RollbackEntry {
            rename_result_id: "test".to_string(),
            status,
            error_message: None,
        }
    }

    #[test]
    fn test_summarize_all_success() {
        let entries = vec![
            make_entry(RollbackStatus::Success),
            make_entry(RollbackStatus::Success),
        ];
        let summary = summarize_rollback(&entries);
        assert_eq!(summary.total, 2);
        assert_eq!(summary.success, 2);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.blocked, 0);
    }

    #[test]
    fn test_summarize_mixed() {
        let entries = vec![
            make_entry(RollbackStatus::Success),
            make_entry(RollbackStatus::Failed),
            make_entry(RollbackStatus::Blocked),
        ];
        let summary = summarize_rollback(&entries);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.success, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.blocked, 1);
    }

    #[test]
    fn test_summarize_empty() {
        let summary = summarize_rollback(&[]);
        assert_eq!(summary.total, 0);
        assert_eq!(summary.success, 0);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.blocked, 0);
    }

    #[test]
    fn test_rollback_status_equality() {
        assert_eq!(RollbackStatus::Success, RollbackStatus::Success);
        assert_eq!(RollbackStatus::Failed, RollbackStatus::Failed);
        assert_eq!(RollbackStatus::Blocked, RollbackStatus::Blocked);
        assert_ne!(RollbackStatus::Success, RollbackStatus::Failed);
    }
}
