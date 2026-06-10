// 审计步骤模块
// 职责：格式化执行审计消息
// 不做执行逻辑，不做安全检查，不做数据库写入

use super::execution_contract::ExecutionItemStatus;

/// 格式化执行审计消息
///
/// 为每个执行结果生成审计日志消息
/// 实际写入由调用方负责
pub fn format_execution_audit(
    task_id: &str,
    status: &ExecutionItemStatus,
    source: &str,
    target: &str,
) -> String {
    match status {
        ExecutionItemStatus::Success => {
            format!("[{}] 重命名成功: {} -> {}", task_id, source, target)
        }
        ExecutionItemStatus::Failed => {
            format!("[{}] 重命名失败: {} -> {}", task_id, source, target)
        }
        ExecutionItemStatus::Skipped => {
            format!("[{}] 重命名跳过: {} -> {}", task_id, source, target)
        }
        ExecutionItemStatus::Blocked => {
            format!("[{}] 重命名阻塞: {} -> {}", task_id, source, target)
        }
    }
}

/// 批量格式化执行审计
pub fn format_batch_execution_audit(
    task_id: &str,
    results: &[super::execution_contract::ExecutionItemResult],
) -> Vec<String> {
    results
        .iter()
        .map(|r| format_execution_audit(task_id, &r.status, &r.source_path, &r.target_path))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_success() {
        let msg = format_execution_audit("task-1", &ExecutionItemStatus::Success, "/s", "/t");
        assert!(msg.contains("成功"));
        assert!(msg.contains("task-1"));
    }

    #[test]
    fn test_format_failed() {
        let msg = format_execution_audit("task-1", &ExecutionItemStatus::Failed, "/s", "/t");
        assert!(msg.contains("失败"));
    }

    #[test]
    fn test_format_skipped() {
        let msg = format_execution_audit("task-1", &ExecutionItemStatus::Skipped, "/s", "/t");
        assert!(msg.contains("跳过"));
    }

    #[test]
    fn test_format_blocked() {
        let msg = format_execution_audit("task-1", &ExecutionItemStatus::Blocked, "/s", "/t");
        assert!(msg.contains("阻塞"));
    }

    #[test]
    fn test_format_batch() {
        let results = vec![
            super::super::execution_contract::ExecutionItemResult::success("/s1", "/t1"),
            super::super::execution_contract::ExecutionItemResult::failed("/s2", "/t2", "err"),
        ];
        let msgs = format_batch_execution_audit("task-1", &results);
        assert_eq!(msgs.len(), 2);
    }
}
