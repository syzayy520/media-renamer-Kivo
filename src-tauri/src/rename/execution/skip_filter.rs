// 跳过过滤模块
// 职责：过滤应该跳过的项
// 不做冲突检查，不做结果记录

use crate::rename::template::RenamePreviewItem;

/// 过滤掉应该跳过的项，返回可执行的项
pub fn filter_actionable(items: &[RenamePreviewItem]) -> Vec<&RenamePreviewItem> {
    items.iter().filter(|item| !item.should_skip).collect()
}
