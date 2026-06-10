// rename/candidate_apply/tv_candidate_apply 模块 - 剧集候选应用
// 职责：将 TMDb 剧集候选信息应用到预览项

use super::preview_apply_result::{PreviewApplyError, PreviewApplyResult};
use super::target_name_builder;
use crate::rename::template::{MetadataSource, RenamePreviewItem};

/// 应用剧集候选到预览项
///
/// # Arguments
/// * `item` - 原始预览项
/// * `title` - 剧集标题
/// * `year` - 年份
/// * `season` - 季号
/// * `episode` - 集号
/// * `episode_title` - 集标题
/// * `tmdb_id` - TMDb ID
pub fn apply_tv_candidate(
    item: &RenamePreviewItem,
    title: &str,
    year: Option<u16>,
    season: Option<u16>,
    episode: Option<u16>,
    episode_title: Option<&str>,
    tmdb_id: u64,
) -> Result<PreviewApplyResult, PreviewApplyError> {
    // 验证标题
    if title.is_empty() {
        return Err(PreviewApplyError::IncompleteCandidate(
            "剧集标题不能为空".to_string(),
        ));
    }

    // 获取文件扩展名
    let extension = std::path::Path::new(&item.original_name)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    // 构建新的文件名
    let new_proposed_name = target_name_builder::build_tv_target_name(
        title,
        season,
        episode,
        episode_title,
        &extension,
    )
    .map_err(PreviewApplyError::InvalidFileName)?;

    // 更新预览项
    let mut updated_item =
        target_name_builder::update_preview_item_target(item, &new_proposed_name);

    // 更新解析信息
    updated_item.parsed_info.title = title.to_string();
    updated_item.parsed_info.year = year;
    updated_item.parsed_info.season = season;
    updated_item.parsed_info.episode = episode;
    updated_item.parsed_info.episode_title = episode_title.map(|s| s.to_string());
    updated_item.media_type = crate::parse::movie_parser::MediaType::Series;
    updated_item.metadata_source = MetadataSource::Tmdb;
    updated_item.confidence = 95; // TMDb 匹配置信度高
    updated_item.needs_manual_review = false;
    updated_item.should_skip = false;

    // 生成警告
    let mut warnings = Vec::new();
    if season.is_none() {
        warnings.push("TMDb 候选缺少季号信息".to_string());
    }
    if episode.is_none() {
        warnings.push("TMDb 候选缺少集号信息".to_string());
    }
    if episode_title.is_none() {
        warnings.push("TMDb 候选缺少集标题信息".to_string());
    }

    Ok(PreviewApplyResult {
        updated_item,
        success: true,
        message: format!("已应用 TMDb 剧集候选: {} (TMDb ID: {})", title, tmdb_id),
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::MediaItem;

    fn make_test_item() -> RenamePreviewItem {
        RenamePreviewItem {
            id: "test-1".to_string(),
            parsed_info: crate::parse::movie_parser::ParsedMediaInfo {
                media_item: MediaItem {
                    id: "media-1".to_string(),
                    file_path: "C:\\test\\episode.mkv".to_string(),
                    file_name: "episode.mkv".to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: crate::parse::movie_parser::MediaType::Series,
                title: "Unknown".to_string(),
                year: None,
                season: None,
                episode: None,
                episode_end: None,
                episode_title: None,
                resolution: None,
                source: None,
                video_codec: None,
                audio_codec: None,
                group: None,
                confidence: 50,
                rule_sources: vec![],
                special_type: None,
                extra_type: None,
                extra_number: None,
            },
            source_path: "C:\\test\\episode.mkv".to_string(),
            original_name: "episode.mkv".to_string(),
            proposed_name: "Unknown.mkv".to_string(),
            target_path: "C:\\test\\Unknown.mkv".to_string(),
            media_type: crate::parse::movie_parser::MediaType::Series,
            confidence: 50,
            needs_manual_review: true,
            should_skip: true,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    #[test]
    fn test_apply_tv_candidate_success() {
        let item = make_test_item();
        let result = apply_tv_candidate(
            &item,
            "Breaking Bad",
            Some(2008u16),
            Some(1u16),
            Some(5u16),
            Some("Gray Matter"),
            1396u64,
        );
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(
            result.updated_item.proposed_name,
            "Breaking Bad - S01E05 - Gray Matter.mkv"
        );
        assert_eq!(result.updated_item.parsed_info.title, "Breaking Bad");
        assert_eq!(result.updated_item.parsed_info.year, Some(2008));
        assert_eq!(result.updated_item.parsed_info.season, Some(1));
        assert_eq!(result.updated_item.parsed_info.episode, Some(5));
        assert_eq!(
            result.updated_item.parsed_info.episode_title,
            Some("Gray Matter".to_string())
        );
        assert_eq!(result.updated_item.metadata_source, MetadataSource::Tmdb);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_apply_tv_candidate_missing_fields() {
        let item = make_test_item();
        let result = apply_tv_candidate(
            &item,
            "Breaking Bad",
            Some(2008u16),
            None,
            None,
            None,
            1396u64,
        );
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(
            result.updated_item.proposed_name,
            "Breaking Bad - S00E00.mkv"
        );
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_apply_tv_candidate_empty_title() {
        let item = make_test_item();
        let result = apply_tv_candidate(
            &item,
            "",
            Some(2008u16),
            Some(1u16),
            Some(5u16),
            None,
            1396u64,
        );
        assert!(result.is_err());
    }
}
