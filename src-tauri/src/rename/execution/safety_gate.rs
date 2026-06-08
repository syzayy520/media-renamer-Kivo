// 安全门模块
// 职责：执行前安全检查判断
// 不做执行逻辑，不做过滤

use super::execution_mode::ExecutionMode;
use crate::rename::safety_checker;
use crate::rename::template::RenamePreviewItem;
use crate::shared::result_types::{AppError, AppResult};

/// 检查安全门条件
pub fn check_safety_gate(
    preview_items: &[RenamePreviewItem],
    mode: ExecutionMode,
) -> AppResult<()> {
    let report = safety_checker::check_all(preview_items);

    // Confirmed 模式必须通过安全检查
    if mode == ExecutionMode::Confirmed && !report.can_execute {
        return Err(AppError::SafetyCheckFailed {
            check_name: "all".to_string(),
            reason: report.blocking_reasons.join("; "),
        });
    }

    Ok(())
}
