// 执行契约模块
// 职责：定义安全执行的输入/输出类型
// 不做执行逻辑，不做安全检查

use serde::{Deserialize, Serialize};

use super::execution_mode::ExecutionMode;
use crate::rename::template::RenamePreviewItem;
use crate::rollback::executor::rollback_entry::RollbackEntry;
use crate::rollback::rollback_plan::RollbackPlan;

/// 安全执行输入
#[derive(Debug, Clone, Deserialize)]
pub struct SafeExecuteInput {
    /// 任务 ID
    pub task_id: String,
    /// 预览项列表
    pub preview_items: Vec<RenamePreviewItem>,
    /// 执行模式
    pub mode: ExecutionMode,
    /// 用户已确认（仅 Confirmed 模式需要）
    pub user_confirmed: bool,
}

/// 安全执行输出
#[derive(Debug, Clone, Serialize)]
pub struct SafeExecuteOutput {
    /// 执行是否被允许
    pub allowed: bool,
    /// 拒绝原因（不允许时有值）
    pub rejection_reason: Option<String>,
    /// 执行结果项（允许时有值）
    pub item_results: Vec<ExecutionItemResult>,
    /// 执行摘要（允许时有值）
    pub summary: Option<ExecutionOutputSummary>,
    /// 回滚计划（Confirmed 模式执行成功后有值）
    pub rollback_plan: Option<RollbackPlan>,
}

/// 单个文件执行结果
#[derive(Debug, Clone, Serialize)]
pub struct ExecutionItemResult {
    /// 源文件路径
    pub source_path: String,
    /// 目标文件路径
    pub target_path: String,
    /// 执行状态
    pub status: ExecutionItemStatus,
    /// 错误信息（失败时有值）
    pub error: Option<String>,
    /// 回滚条目（成功时有值）
    pub rollback_entry: Option<RollbackEntry>,
}

/// 执行项状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionItemStatus {
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 跳过（用户跳过或冲突跳过）
    Skipped,
    /// 阻塞（安全检查未通过）
    Blocked,
}

/// 执行输出摘要
#[derive(Debug, Clone, Serialize)]
pub struct ExecutionOutputSummary {
    /// 总数
    pub total: usize,
    /// 成功数
    pub success: usize,
    /// 失败数
    pub failed: usize,
    /// 跳过数
    pub skipped: usize,
    /// 阻塞数
    pub blocked: usize,
    /// 执行模式
    pub mode: ExecutionMode,
    /// 是否生成了回滚计划
    pub has_rollback_plan: bool,
}

impl ExecutionItemResult {
    /// 创建成功结果
    pub fn success(source: &str, target: &str) -> Self {
        Self {
            source_path: source.to_string(),
            target_path: target.to_string(),
            status: ExecutionItemStatus::Success,
            error: None,
            rollback_entry: None,
        }
    }

    /// 创建失败结果
    pub fn failed(source: &str, target: &str, error: &str) -> Self {
        Self {
            source_path: source.to_string(),
            target_path: target.to_string(),
            status: ExecutionItemStatus::Failed,
            error: Some(error.to_string()),
            rollback_entry: None,
        }
    }

    /// 创建跳过结果
    pub fn skipped(source: &str, target: &str) -> Self {
        Self {
            source_path: source.to_string(),
            target_path: target.to_string(),
            status: ExecutionItemStatus::Skipped,
            error: None,
            rollback_entry: None,
        }
    }

    /// 创建阻塞结果
    pub fn blocked(source: &str, target: &str, reason: &str) -> Self {
        Self {
            source_path: source.to_string(),
            target_path: target.to_string(),
            status: ExecutionItemStatus::Blocked,
            error: Some(reason.to_string()),
            rollback_entry: None,
        }
    }

    /// 设置回滚条目
    pub fn with_rollback_entry(mut self, entry: RollbackEntry) -> Self {
        self.rollback_entry = Some(entry);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_item_status_equality() {
        assert_eq!(ExecutionItemStatus::Success, ExecutionItemStatus::Success);
        assert_eq!(ExecutionItemStatus::Failed, ExecutionItemStatus::Failed);
        assert_ne!(ExecutionItemStatus::Success, ExecutionItemStatus::Failed);
    }

    #[test]
    fn test_execution_item_result_success() {
        let result = ExecutionItemResult::success("/source", "/target");
        assert_eq!(result.status, ExecutionItemStatus::Success);
        assert!(result.error.is_none());
        assert!(result.rollback_entry.is_none());
    }

    #[test]
    fn test_execution_item_result_failed() {
        let result = ExecutionItemResult::failed("/source", "/target", "error");
        assert_eq!(result.status, ExecutionItemStatus::Failed);
        assert_eq!(result.error.unwrap(), "error");
    }

    #[test]
    fn test_execution_item_result_skipped() {
        let result = ExecutionItemResult::skipped("/source", "/target");
        assert_eq!(result.status, ExecutionItemStatus::Skipped);
    }

    #[test]
    fn test_execution_item_result_blocked() {
        let result = ExecutionItemResult::blocked("/source", "/target", "conflict");
        assert_eq!(result.status, ExecutionItemStatus::Blocked);
        assert_eq!(result.error.unwrap(), "conflict");
    }

    #[test]
    fn test_safe_execute_input_deserialize() {
        let json = r#"{
            "task_id": "test-task",
            "preview_items": [],
            "mode": "DryRun",
            "user_confirmed": false
        }"#;
        let input: SafeExecuteInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.task_id, "test-task");
        assert_eq!(input.mode, ExecutionMode::DryRun);
        assert!(!input.user_confirmed);
    }

    #[test]
    fn test_safe_execute_input_confirmed_mode() {
        let json = r#"{
            "task_id": "test-task",
            "preview_items": [],
            "mode": "Confirmed",
            "user_confirmed": true
        }"#;
        let input: SafeExecuteInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.mode, ExecutionMode::Confirmed);
        assert!(input.user_confirmed);
    }
}
