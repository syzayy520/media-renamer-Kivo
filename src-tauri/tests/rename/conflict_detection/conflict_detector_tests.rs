// ConflictDetector 测试
// 职责：测试冲突检测编排

use app_lib::rename::conflict_detector::{detect_conflicts, has_blocking_conflicts};
use app_lib::rename::template::{RenamePreviewItem, RenameConflict, MetadataSource};
use app_lib::shared::result_types::ConflictType;
use app_lib::parse::movie_parser::{MediaType, ParsedMediaInfo};
use app_lib::scan::MediaItem;
use uuid::Uuid;

fn make_preview_item(source: &str, target: &str) -> RenamePreviewItem {
    RenamePreviewItem {
        id: Uuid::new_v4().to_string(),
        parsed_info: ParsedMediaInfo {
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
        metadata_source: MetadataSource::LocalRule,
    }
}

#[test]
fn test_detect_no_conflict() {
    let item = make_preview_item("C:\\test\\file.mkv", "C:\\test\\new.mkv");
    let conflicts = detect_conflicts(&[item]);
    assert!(conflicts.is_empty());
}

#[test]
fn test_detect_target_exists() {
    let item = make_preview_item(
        "C:\\test\\file.mkv",
        "C:\\test\\file.mkv", // 目标与源相同
    );
    let conflicts = detect_conflicts(&[item]);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].conflict_type, ConflictType::TargetExists);
    assert!(conflicts[0].blocking);
}

#[test]
fn test_detect_path_too_long() {
    let long_path = format!("C:\\test\\{}.mkv", "a".repeat(260));
    let item = make_preview_item("C:\\test\\file.mkv", &long_path);
    let conflicts = detect_conflicts(&[item]);
    assert!(conflicts
        .iter()
        .any(|c| c.conflict_type == ConflictType::PathTooLong));
}

#[test]
fn test_detect_multi_conflict_aggregation() {
    // 一个项目可能有多个冲突
    let long_path = format!("C:\\test\\{}.mkv", "a".repeat(260));
    let item = make_preview_item("C:\\test\\file.mkv", &long_path);
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