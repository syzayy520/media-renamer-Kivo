// 冲突类型夹具
// 职责：提供各种冲突类型的测试数据

use app_lib::rename::template::{RenameConflict, RenamePreviewItem};
use app_lib::shared::result_types::ConflictType;
use crate::test_support_preview_item_builder::PreviewItemBuilder;

/// 创建目标已存在冲突
#[allow(dead_code)]
pub fn target_exists_conflict(source: &str, target: &str) -> RenameConflict {
    RenameConflict {
        conflict_type: ConflictType::TargetExists,
        source_path: source.to_string(),
        target_path: target.to_string(),
        message: format!("Target already exists: {}", target),
        blocking: true,
    }
}

/// 创建重复目标冲突
#[allow(dead_code)]
pub fn duplicate_target_conflict(source: &str, target: &str) -> RenameConflict {
    RenameConflict {
        conflict_type: ConflictType::DuplicateTarget,
        source_path: source.to_string(),
        target_path: target.to_string(),
        message: format!("Duplicate target: {}", target),
        blocking: true,
    }
}

/// 创建路径过长冲突
#[allow(dead_code)]
pub fn path_too_long_conflict(source: &str, target: &str) -> RenameConflict {
    RenameConflict {
        conflict_type: ConflictType::PathTooLong,
        source_path: source.to_string(),
        target_path: target.to_string(),
        message: format!("Path too long: {} chars", target.len()),
        blocking: true,
    }
}

/// 创建非法字符冲突
#[allow(dead_code)]
pub fn invalid_chars_conflict(source: &str, target: &str) -> RenameConflict {
    RenameConflict {
        conflict_type: ConflictType::InvalidChars,
        source_path: source.to_string(),
        target_path: target.to_string(),
        message: format!("Invalid characters in: {}", target),
        blocking: false,
    }
}

/// 生成带冲突的预览项（确保 detect_conflicts 能发现重复目标）
#[allow(dead_code)]
pub fn generate_items_with_conflicts(count: usize) -> Vec<RenamePreviewItem> {
    (0..count)
        .map(|i| {
            let mut item = PreviewItemBuilder::new(i).build();
            if i % 3 == 0 {
                // Give these items a shared target so detect_conflicts finds duplicates
                item.target_path = "C:\\media\\conflict_shared_target.mkv".to_string();
            }
            if i % 5 == 0 {
                item.conflicts.push(invalid_chars_conflict(
                    &item.source_path,
                    &item.target_path,
                ));
            }
            item
        })
        .collect()
}
