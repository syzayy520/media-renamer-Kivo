// DuplicateTargetChecker 测试
// 职责：测试重复目标检查

use app_lib::rename::duplicate_target_checker::check_duplicate_targets;
use app_lib::rename::template::{RenamePreviewItem, MetadataSource};
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
fn test_duplicate_target_detected() {
    let item1 = make_preview_item("C:\\test\\file1.mkv", "C:\\test\\new.mkv");
    let item2 = make_preview_item("C:\\test\\file2.mkv", "C:\\test\\new.mkv");
    let conflicts = check_duplicate_targets(&[item1, item2]);
    assert!(conflicts.len() >= 2);
    assert!(conflicts
        .iter()
        .any(|c| c.conflict_type == ConflictType::DuplicateTarget));
}

#[test]
fn test_no_duplicate_target() {
    let item1 = make_preview_item("C:\\test\\file1.mkv", "C:\\test\\new1.mkv");
    let item2 = make_preview_item("C:\\test\\file2.mkv", "C:\\test\\new2.mkv");
    let conflicts = check_duplicate_targets(&[item1, item2]);
    assert!(conflicts.is_empty());
}