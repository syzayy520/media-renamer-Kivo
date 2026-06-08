// 执行器核心模块
// 职责：协调批量重命名执行主流程
// 不做安全检查，不做过滤，不做结果记录

use super::conflict_filter::{filter_non_blocking, get_blocking_items};
use super::execution_mode::ExecutionMode;
use super::result_recorder::record_blocking_conflicts;
use super::safety_gate::check_safety_gate;
use super::single_rename::execute_single_rename;
use super::skip_filter::filter_actionable;
use crate::audit::db::{self, RenameResult, TaskStatus};
use crate::rename::template::RenamePreviewItem;
use crate::shared::result_types::AppResult;
use rusqlite::Connection;

/// 执行重命名任务
///
/// 主流程编排：
/// 1. 安全门检查
/// 2. 过滤跳过项
/// 3. 过滤冲突项
/// 4. 记录失败项
/// 5. 执行非阻塞项
pub fn execute(
    _conn: &Connection,
    task_id: &str,
    preview_items: &[RenamePreviewItem],
    mode: ExecutionMode,
) -> AppResult<Vec<RenameResult>> {
    // 1. 安全门检查
    check_safety_gate(preview_items, mode)?;

    // 2. 过滤跳过项
    let actionable_items = filter_actionable(preview_items);

    // 3. 过滤冲突项
    let blocking_items = get_blocking_items(&actionable_items);
    let non_blocking_items = filter_non_blocking(&actionable_items);

    let mut results = Vec::new();

    // 4. 记录失败项
    let failed_results = record_blocking_conflicts(task_id, &blocking_items);
    results.extend(failed_results);

    // 5. 执行非阻塞项
    for item in non_blocking_items {
        let result = match mode {
            ExecutionMode::DryRun => {
                // DryRun: 不执行文件操作，返回模拟成功
                db::create_result(
                    task_id,
                    &item.source_path,
                    &item.target_path,
                    TaskStatus::Pending,
                )
            }
            ExecutionMode::Confirmed => {
                // Confirmed: 执行真实重命名
                execute_single_rename(item, task_id)
            }
        };
        results.push(result);
    }

    Ok(results)
}
