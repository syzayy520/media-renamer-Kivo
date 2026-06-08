// 冲突过滤模块
// 职责：过滤有阻塞冲突的项
// 不做安全检查，不做结果记录

use crate::rename::conflict_detector;
use crate::rename::template::RenamePreviewItem;

/// 过滤掉有阻塞冲突的项，返回无冲突的项
pub fn filter_non_blocking<'a>(items: &[&'a RenamePreviewItem]) -> Vec<&'a RenamePreviewItem> {
    items
        .iter()
        .filter(|item| !conflict_detector::has_blocking_conflicts(&item.conflicts))
        .copied()
        .collect()
}

/// 获取有阻塞冲突的项
pub fn get_blocking_items<'a>(items: &[&'a RenamePreviewItem]) -> Vec<&'a RenamePreviewItem> {
    items
        .iter()
        .filter(|item| conflict_detector::has_blocking_conflicts(&item.conflicts))
        .copied()
        .collect()
}
