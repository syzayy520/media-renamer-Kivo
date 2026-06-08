// 路径长度检查模块
// 职责：检查路径是否过长
// 不做其他冲突检查

use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::path_utils;
use crate::shared::result_types::ConflictType;

/// 检查路径是否过长
pub fn check_path_length(item: &RenamePreviewItem) -> Option<RenameConflict> {
    if path_utils::is_path_too_long(&item.target_path) {
        Some(RenameConflict {
            conflict_type: ConflictType::PathTooLong,
            source_path: item.source_path.clone(),
            target_path: item.target_path.clone(),
            message: format!("Path too long: {}", item.target_path.len()),
            blocking: true,
        })
    } else {
        None
    }
}
