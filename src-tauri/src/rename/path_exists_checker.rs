// 路径存在检查模块
// 职责：检查目标路径是否已存在
// 不做其他冲突检查

use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::result_types::ConflictType;

/// 检查目标路径是否已存在
pub fn check_path_exists(item: &RenamePreviewItem) -> Option<RenameConflict> {
    // 检查目标路径是否已存在（模拟：检查是否与源路径相同）
    if item.target_path == item.source_path {
        Some(RenameConflict {
            conflict_type: ConflictType::TargetExists,
            source_path: item.source_path.clone(),
            target_path: item.target_path.clone(),
            message: format!("Target path already exists: {}", item.target_path),
            blocking: true,
        })
    } else {
        None
    }
}
