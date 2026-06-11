use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
use crate::rename::naming_context::build_token_context;
use crate::rename::template::{MetadataSource, RenamePreviewItem};
use crate::scan::MediaItem;

fn preview_item(file_name: &str) -> RenamePreviewItem {
    RenamePreviewItem {
        id: "test-item".to_string(),
        parsed_info: ParsedMediaInfo {
            media_item: MediaItem {
                id: "media-1".to_string(),
                file_path: format!("C:\\media\\{}", file_name),
                file_name: file_name.to_string(),
                extension: "mkv".to_string(),
                file_size: 0,
                is_video: true,
                is_companion: false,
            },
            media_type: MediaType::Movie,
            title: "Unknown".to_string(),
            year: Some(1982),
            season: None,
            episode: None,
            episode_end: None,
            episode_title: None,
            resolution: None,
            source: None,
            video_codec: None,
            audio_codec: None,
            group: None,
            confidence: 95,
            rule_sources: vec![],
            special_type: None,
            extra_type: None,
            extra_number: None,
        },
        source_path: format!("C:\\media\\{}", file_name),
        original_name: file_name.to_string(),
        proposed_name: file_name.to_string(),
        target_path: format!("C:\\media\\{}", file_name),
        media_type: MediaType::Movie,
        confidence: 95,
        needs_manual_review: false,
        should_skip: false,
        conflicts: vec![],
        evidence: vec![],
        metadata_source: MetadataSource::LocalRule,
    }
}

#[test]
fn builds_context_from_release_filename() {
    let item = preview_item("First.Blood.1982.1080p.BluRay.x265.DTS.5.1-PTer.mkv");
    let ctx = build_token_context(&item);

    assert_eq!(ctx.year, Some(1982));
    assert_eq!(ctx.resolution.as_deref(), Some("1080p"));
    assert_eq!(ctx.source.as_deref(), Some("BluRay"));
    assert_eq!(ctx.video_codec.as_deref(), Some("x265"));
    assert_eq!(ctx.audio_codec.as_deref(), Some("DTS"));
    assert_eq!(ctx.audio_channels.as_deref(), Some("5.1"));
    assert_eq!(ctx.release_group.as_deref(), Some("PTer"));
}
