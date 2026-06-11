use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::path_utils;
use crate::shared::result_types::ConflictType;

pub fn check_path_exists(item: &RenamePreviewItem) -> Option<RenameConflict> {
    if is_same_path(&item.source_path, &item.target_path) {
        return None;
    }

    None
}

fn is_same_path(left: &str, right: &str) -> bool {
    path_utils::normalize_path(left).to_lowercase() == path_utils::normalize_path(right).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::template::MetadataSource;
    use crate::scan::MediaItem;

    fn make_item(source: &str, target: &str) -> RenamePreviewItem {
        RenamePreviewItem {
            id: "test-id".to_string(),
            parsed_info: ParsedMediaInfo {
                media_item: MediaItem {
                    id: "media-id".to_string(),
                    file_path: source.to_string(),
                    file_name: "movie.mkv".to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: MediaType::Movie,
                title: "Movie".to_string(),
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
            source_path: source.to_string(),
            original_name: "movie.mkv".to_string(),
            proposed_name: "movie.mkv".to_string(),
            target_path: target.to_string(),
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
    fn same_path_is_noop_not_conflict() {
        let item = make_item("C:/Media/第一滴血.mkv", "C:\\Media\\第一滴血.mkv");
        assert!(check_path_exists(&item).is_none());
    }

    #[test]
    fn different_path_is_not_claimed_as_existing_without_filesystem_probe() {
        let item = make_item("C:/Media/a.mkv", "C:/Media/b.mkv");
        assert!(check_path_exists(&item).is_none());
    }
}
