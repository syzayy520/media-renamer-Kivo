// 大小写冲突检查模块
// 职责：检查大小写冲突（Windows 不区分大小写）
// 不做其他冲突检查

use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::result_types::ConflictType;
use std::collections::HashMap;

/// 检查大小写冲突（Windows 不区分大小写）
pub fn check_case_conflicts(preview_items: &[RenamePreviewItem]) -> Vec<RenameConflict> {
    let mut conflicts = Vec::new();
    let mut case_map: HashMap<String, Vec<usize>> = HashMap::new();

    // 收集所有目标路径（小写）
    for (index, item) in preview_items.iter().enumerate() {
        let lower_target = item.target_path.to_lowercase();
        case_map.entry(lower_target).or_default().push(index);
    }

    // 检查大小写冲突
    for (lower_target, indices) in case_map {
        if indices.len() > 1 {
            // 检查是否真的是大小写冲突（不同大小写但相同路径）
            let unique_targets: std::collections::HashSet<String> = indices
                .iter()
                .map(|&i| preview_items[i].target_path.clone())
                .collect();
            if unique_targets.len() > 1 {
                for &index in &indices {
                    let item = &preview_items[index];
                    conflicts.push(RenameConflict {
                        conflict_type: ConflictType::DuplicateTarget,
                        source_path: item.source_path.clone(),
                        target_path: item.target_path.clone(),
                        message: format!(
                            "Case conflict: multiple targets differ only in case: {}",
                            lower_target
                        ),
                        blocking: true,
                    });
                }
            }
        }
    }

    conflicts
}
