// 重复目标检查模块
// 职责：检查多个源指向同一目标
// 不做其他冲突检查

use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::path_utils;
use crate::shared::result_types::ConflictType;
use std::collections::HashMap;

/// 检查重复目标（多个源指向同一目标）
pub fn check_duplicate_targets(preview_items: &[RenamePreviewItem]) -> Vec<RenameConflict> {
    let mut conflicts = Vec::new();
    let mut target_map: HashMap<String, Vec<usize>> = HashMap::new();

    // 收集所有目标路径
    for (index, item) in preview_items.iter().enumerate() {
        let normalized_target = path_utils::normalize_path(&item.target_path).to_lowercase();
        target_map.entry(normalized_target).or_default().push(index);
    }

    // 检查重复目标
    for (target, indices) in target_map {
        if indices.len() > 1 {
            for &index in &indices {
                let item = &preview_items[index];
                conflicts.push(RenameConflict {
                    conflict_type: ConflictType::DuplicateTarget,
                    source_path: item.source_path.clone(),
                    target_path: item.target_path.clone(),
                    message: format!(
                        "Duplicate target: {} sources point to {}",
                        indices.len(),
                        target
                    ),
                    blocking: true,
                });
            }
        }
    }

    conflicts
}
