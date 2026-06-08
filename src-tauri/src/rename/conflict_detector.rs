// 冲突检测模块
// 职责：协调冲突检测主流程编排
// 不做具体检查逻辑

use super::case_conflict_checker::check_case_conflicts;
use super::duplicate_target_checker::check_duplicate_targets;
use super::invalid_chars_checker::check_invalid_chars;
use super::path_exists_checker::check_path_exists;
use super::path_length_checker::check_path_length;
use super::template::RenameConflict;
use super::template::RenamePreviewItem;

/// 检测冲突
///
/// 主流程编排：
/// 1. 检查路径存在
/// 2. 检查路径长度
/// 3. 检查非法字符
/// 4. 检查重复目标
/// 5. 检查大小写冲突
pub fn detect_conflicts(preview_items: &[RenamePreviewItem]) -> Vec<RenameConflict> {
    let mut conflicts = Vec::new();

    for item in preview_items {
        // 1. 检查路径存在
        if let Some(conflict) = check_path_exists(item) {
            conflicts.push(conflict);
        }

        // 2. 检查路径长度
        if let Some(conflict) = check_path_length(item) {
            conflicts.push(conflict);
        }

        // 3. 检查非法字符
        if let Some(conflict) = check_invalid_chars(item) {
            conflicts.push(conflict);
        }
    }

    // 4. 检查重复目标
    let duplicate_conflicts = check_duplicate_targets(preview_items);
    conflicts.extend(duplicate_conflicts);

    // 5. 检查大小写冲突
    let case_conflicts = check_case_conflicts(preview_items);
    conflicts.extend(case_conflicts);

    conflicts
}

/// 检查是否有阻塞冲突
pub fn has_blocking_conflicts(conflicts: &[RenameConflict]) -> bool {
    conflicts.iter().any(|c| c.blocking)
}
