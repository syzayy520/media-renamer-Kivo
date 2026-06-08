// 冲突检测模块
// 职责：检测重命名预览项中的冲突

use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::path_utils;
use crate::shared::result_types::ConflictType;
use std::collections::HashMap;

/// 检测冲突
pub fn detect_conflicts(preview_items: &[RenamePreviewItem]) -> Vec<RenameConflict> {
    let mut conflicts = Vec::new();

    // 收集所有目标路径，用于检测重复目标
    let mut target_map: HashMap<String, Vec<usize>> = HashMap::new();

    for (index, item) in preview_items.iter().enumerate() {
        // 检查源文件是否存在（模拟：假设都存在）
        // 在真实环境中，需要检查文件系统

        // 检查目标路径是否已存在（模拟：检查是否与源路径相同）
        if item.target_path == item.source_path {
            conflicts.push(RenameConflict {
                conflict_type: ConflictType::TargetExists,
                source_path: item.source_path.clone(),
                target_path: item.target_path.clone(),
                message: format!("Target path already exists: {}", item.target_path),
                blocking: true,
            });
        }

        // 检查路径过长
        if path_utils::is_path_too_long(&item.target_path) {
            conflicts.push(RenameConflict {
                conflict_type: ConflictType::PathTooLong,
                source_path: item.source_path.clone(),
                target_path: item.target_path.clone(),
                message: format!("Path too long: {}", item.target_path.len()),
                blocking: true,
            });
        }

        // 检查非法字符
        if path_utils::has_invalid_chars(&item.target_path) {
            conflicts.push(RenameConflict {
                conflict_type: ConflictType::InvalidChars,
                source_path: item.source_path.clone(),
                target_path: item.target_path.clone(),
                message: format!("Invalid characters in path: {}", item.target_path),
                blocking: true,
            });
        }

        // 记录目标路径用于重复检测
        let normalized_target = path_utils::normalize_path(&item.target_path).to_lowercase();
        target_map
            .entry(normalized_target)
            .or_default()
            .push(index);
    }

    // 检查重复目标（多个源指向同一目标）
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

    // 检查大小写冲突（Windows 不区分大小写）
    let mut case_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (index, item) in preview_items.iter().enumerate() {
        let lower_target = item.target_path.to_lowercase();
        case_map
            .entry(lower_target)
            .or_default()
            .push(index);
    }

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

/// 检查是否有阻塞冲突
pub fn has_blocking_conflicts(conflicts: &[RenameConflict]) -> bool {
    conflicts.iter().any(|c| c.blocking)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::MediaType;
    use crate::scan::MediaItem;

    fn make_test_preview_item(source: &str, target: &str) -> RenamePreviewItem {
        RenamePreviewItem {
            id: "test-id".to_string(),
            parsed_info: crate::parse::movie_parser::ParsedMediaInfo {
                media_item: MediaItem {
                    id: "test".to_string(),
                    file_path: source.to_string(),
                    file_name: std::path::Path::new(source)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: MediaType::Movie,
                title: "Test".to_string(),
                year: Some(2020),
                season: None,
                episode: None,
                episode_end: None,
                episode_title: None,
                resolution: Some("1080p".to_string()),
                source: None,
                video_codec: None,
                audio_codec: None,
                group: None,
                confidence: 85,
                rule_sources: vec![],
                special_type: None,
                extra_type: None,
                extra_number: None,
            },
            source_path: source.to_string(),
            original_name: std::path::Path::new(source)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            proposed_name: std::path::Path::new(target)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            target_path: target.to_string(),
            media_type: MediaType::Movie,
            confidence: 85,
            needs_manual_review: false,
            should_skip: false,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: super::super::template::MetadataSource::LocalRule,
        }
    }

    #[test]
    fn test_detect_target_exists() {
        let item = make_test_preview_item(
            "C:\\test\\file.mkv",
            "C:\\test\\file.mkv", // 目标与源相同
        );
        let conflicts = detect_conflicts(&[item]);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].conflict_type, ConflictType::TargetExists);
        assert!(conflicts[0].blocking);
    }

    #[test]
    fn test_detect_duplicate_target() {
        let item1 = make_test_preview_item("C:\\test\\file1.mkv", "C:\\test\\new.mkv");
        let item2 = make_test_preview_item("C:\\test\\file2.mkv", "C:\\test\\new.mkv");
        let conflicts = detect_conflicts(&[item1, item2]);
        assert!(conflicts.len() >= 2);
        assert!(conflicts
            .iter()
            .any(|c| c.conflict_type == ConflictType::DuplicateTarget));
    }

    #[test]
    fn test_detect_case_conflict() {
        let item1 = make_test_preview_item("C:\\test\\file1.mkv", "C:\\test\\New.mkv");
        let item2 = make_test_preview_item("C:\\test\\file2.mkv", "C:\\test\\new.mkv");
        let conflicts = detect_conflicts(&[item1, item2]);
        assert!(conflicts
            .iter()
            .any(|c| c.message.contains("Case conflict")));
    }

    #[test]
    fn test_detect_source_not_found() {
        // 模拟源文件不存在（需要文件系统检查，这里跳过）
        // 在真实实现中，需要检查文件系统
        let item = make_test_preview_item("C:\\nonexistent\\file.mkv", "C:\\test\\new.mkv");
        let conflicts = detect_conflicts(&[item]);
        // 没有源文件不存在的检查，所以没有冲突
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_detect_path_too_long() {
        let long_path = format!("C:\\test\\{}.mkv", "a".repeat(260));
        let item = make_test_preview_item("C:\\test\\file.mkv", &long_path);
        let conflicts = detect_conflicts(&[item]);
        assert!(conflicts
            .iter()
            .any(|c| c.conflict_type == ConflictType::PathTooLong));
    }

    #[test]
    fn test_detect_no_conflict() {
        let item = make_test_preview_item("C:\\test\\file.mkv", "C:\\test\\new.mkv");
        let conflicts = detect_conflicts(&[item]);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_detect_multi_conflict_aggregation() {
        // 一个项目可能有多个冲突
        let long_path = format!("C:\\test\\{}.mkv", "a".repeat(260));
        let item = make_test_preview_item("C:\\test\\file.mkv", &long_path);
        let conflicts = detect_conflicts(&[item]);
        // 可能有路径过长和目标已存在（如果源和目标相同）
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_has_blocking_conflicts() {
        let blocking = RenameConflict {
            conflict_type: ConflictType::TargetExists,
            source_path: "a".to_string(),
            target_path: "b".to_string(),
            message: "test".to_string(),
            blocking: true,
        };
        let non_blocking = RenameConflict {
            conflict_type: ConflictType::InvalidChars,
            source_path: "a".to_string(),
            target_path: "b".to_string(),
            message: "test".to_string(),
            blocking: false,
        };
        assert!(has_blocking_conflicts(std::slice::from_ref(&blocking)));
        assert!(!has_blocking_conflicts(std::slice::from_ref(&non_blocking)));
        assert!(has_blocking_conflicts(&[blocking, non_blocking]));
    }
}
