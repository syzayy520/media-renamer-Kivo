// Confirmed 重命名执行器模块
// 职责：执行真实文件重命名
// 不做 DryRun，不做安全检查

use super::execution_contract::{ExecutionItemResult, ExecutionItemStatus};
use super::single_rename::execute_single_rename;
use crate::audit::db::TaskStatus;
use crate::rename::template::RenamePreviewItem;
use crate::rollback::rollback_plan::RollbackPlan;

/// 执行 Confirmed 重命名
///
/// 对每个非阻塞项执行真实重命名
/// 成功时更新回滚计划
pub fn execute_confirmed_rename(
    items: &[&RenamePreviewItem],
    task_id: &str,
    rollback_plan: &mut RollbackPlan,
) -> Vec<ExecutionItemResult> {
    let mut results = Vec::new();

    for item in items {
        let rename_result = execute_single_rename(item, task_id);
        let item_result = ExecutionItemResult {
            source_path: rename_result.source_path,
            target_path: rename_result.target_path,
            status: match rename_result.status {
                TaskStatus::Completed | TaskStatus::Pending => ExecutionItemStatus::Success,
                TaskStatus::Failed => ExecutionItemStatus::Failed,
                _ => ExecutionItemStatus::Skipped,
            },
            error: rename_result.error_message,
            rollback_entry: None,
        };

        // 成功时标记回滚计划
        if item_result.status == ExecutionItemStatus::Success {
            rollback_plan.mark_executed(&item.source_path, &item.target_path);
        }

        results.push(item_result);
    }

    results
}

#[cfg(test)]
mod tests {
    // 注：此模块的测试需要文件系统操作，集成测试中覆盖
    // 单元测试只验证函数签名和基本逻辑
    use super::*;

    #[test]
    fn test_function_signature_exists() {
        // 验证函数可以被引用
        let _ = execute_confirmed_rename;
    }
}
