// 单文件重命名模块
// 职责：执行单个文件的真实重命名操作
// 不做批量处理，不做审计写入

use crate::audit::db::{self, RenameResult, TaskStatus};
use crate::rename::template::RenamePreviewItem;

/// 执行单个文件的真实重命名
pub fn execute_single_rename(item: &RenamePreviewItem, task_id: &str) -> RenameResult {
    let source = std::path::Path::new(&item.source_path);
    let target = std::path::Path::new(&item.target_path);

    // 检查源文件是否存在
    if !source.exists() {
        let mut result = db::create_result(
            task_id,
            &item.source_path,
            &item.target_path,
            TaskStatus::Failed,
        );
        result.error_message = Some(format!("Source file not found: {}", item.source_path));
        return result;
    }

    // 检查目标是否已存在
    if target.exists() {
        let mut result = db::create_result(
            task_id,
            &item.source_path,
            &item.target_path,
            TaskStatus::Failed,
        );
        result.error_message = Some(format!("Target already exists: {}", item.target_path));
        return result;
    }

    // 执行 rename
    match std::fs::rename(source, target) {
        Ok(()) => db::create_result(
            task_id,
            &item.source_path,
            &item.target_path,
            TaskStatus::Completed,
        ),
        Err(e) => {
            let mut result = db::create_result(
                task_id,
                &item.source_path,
                &item.target_path,
                TaskStatus::Failed,
            );
            result.error_message = Some(format!("Rename failed: {}", e));
            result
        }
    }
}
