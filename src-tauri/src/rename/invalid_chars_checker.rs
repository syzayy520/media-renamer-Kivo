// 非法字符检查模块
// 职责：检查路径中是否有非法字符
// 不做其他冲突检查

use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::path_utils;
use crate::shared::result_types::ConflictType;

/// 检查路径中是否有非法字符
pub fn check_invalid_chars(item: &RenamePreviewItem) -> Option<RenameConflict> {
    if path_utils::has_invalid_chars(&item.target_path) {
        Some(RenameConflict {
            conflict_type: ConflictType::InvalidChars,
            source_path: item.source_path.clone(),
            target_path: item.target_path.clone(),
            message: format!("Invalid characters in path: {}", item.target_path),
            blocking: true,
        })
    } else {
        None
    }
}
