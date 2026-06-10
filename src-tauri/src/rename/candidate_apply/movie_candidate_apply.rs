// rename/candidate_apply/movie_candidate_apply 模块 - 电影候选应用
// 职责：将 TMDb 电影候选信息应用到预览项

use super::preview_apply_result::{PreviewApplyError, PreviewApplyResult};
use super::target_name_builder;
use crate::rename::template::{MetadataSource, RenamePreviewItem};

/// 应用电影候选到预览项
///
/// # Arguments
/// * `item` - 原始预览项
/// * `title` - 电影标题
/// * `year` - 年份
/// * `tmdb_id` - TMDb ID
pub fn apply_movie_candidate(
    item: &RenamePreviewItem,
    title: &str,
    year: Option<u16>,
    tmdb_id: u64,
) -> Result<PreviewApplyResult, PreviewApplyError> {
    // 验证标题
    if title.is_empty() {
        return Err(PreviewApplyError::IncompleteCandidate(
            "电影标题不能为空".to_string(),
        ));
    }

    // 获取文件扩展名
    let extension = std::path::Path::new(&item.original_name)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    // 构建新的文件名
    let new_proposed_name = target_name_builder::build_movie_target_name(title, year, &extension)
        .map_err(PreviewApplyError::InvalidFileName)?;

    // 更新预览项
    let mut updated_item =
        target_name_builder::update_preview_item_target(item, &new_proposed_name);

    // 更新解析信息
    updated_item.parsed_info.title = title.to_string();
    updated_item.parsed_info.year = year;
    updated_item.media_type = crate::parse::movie_parser::MediaType::Movie;
    updated_item.metadata_source = MetadataSource::Tmdb;
    updated_item.confidence = 95; // TMDb 匹配置信度高
    updated_item.needs_manual_review = false;
    updated_item.should_skip = false;

    // 生成警告
    let mut warnings = Vec::new();
    if year.is_none() {
        warnings.push("TMDb 候选缺少年份信息".to_string());
    }

    Ok(PreviewApplyResult {
        updated_item,
        success: true,
        message: format!("已应用 TMDb 电影候选: {} (TMDb ID: {})", title, tmdb_id),
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
                    file_path: "C:\\test\\movie.mkv".to_string(),
                    file_name: "movie.mkv".to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: crate::parse::movie_parser::MediaType::Movie,
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
            source_path: "C:\\test\\movie.mkv".to_string(),
            original_name: "movie.mkv".to_string(),
            proposed_name: "Unknown.mkv".to_string(),
            target_path: "C:\\test\\Unknown.mkv".to_string(),
            media_type: crate::parse::movie_parser::MediaType::Movie,
            confidence: 50,
            needs_manual_review: true,
            should_skip: true,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    #[test]
    fn test_apply_movie_candidate_success() {
        let item = make_test_item();
        let result = apply_movie_candidate(&item, "Inception", Some(2010u16), 550u64);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.updated_item.proposed_name, "Inception (2010).mkv");
        assert_eq!(result.updated_item.parsed_info.title, "Inception");
        assert_eq!(result.updated_item.parsed_info.year, Some(2010));
        assert_eq!(result.updated_item.metadata_source, MetadataSource::Tmdb);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_apply_movie_candidate_no_year() {
        let item = make_test_item();
        let result = apply_movie_candidate(&item, "Inception", None, 550u64);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.updated_item.proposed_name, "Inception.mkv");
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_apply_movie_candidate_empty_title() {
        let item = make_test_item();
        let result = apply_movie_candidate(&item, "", Some(2010u16), 550u64);
        assert!(result.is_err());
    }
}
