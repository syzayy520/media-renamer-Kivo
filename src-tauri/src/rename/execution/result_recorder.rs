// 结果记录模块
// 职责：记录失败项结果
// 不做安全检查，不做过滤

use crate::audit::db::{self, RenameResult, TaskStatus};
use crate::rename::template::RenamePreviewItem;

/// 记录有阻塞冲突的项为失败
pub fn record_blocking_conflicts(task_id: &str, items: &[&RenamePreviewItem]) -> Vec<RenameResult> {
    let mut results = Vec::new();

    for item in items {
        let result = db::create_result(
            task_id,
            &item.source_path,
            &item.target_path,
            TaskStatus::Failed,
        );
        // 手动设置 error_message
        let mut result = result;
        result.error_message = Some(format!(
            "Blocking conflicts: {}",
            item.conflicts
                .iter()
                .filter(|c| c.blocking)
                .map(|c| c.message.clone())
                .collect::<Vec<_>>()
                .join("; ")
        ));
        results.push(result);
    }

    results
}
