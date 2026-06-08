// 预览生成模块
// 职责：从解析信息生成重命名预览项

use super::conflict_detector;
use super::template::{self, MetadataSource, RenamePreviewItem};
use crate::parse::confidence;
use crate::parse::movie_parser::ParsedMediaInfo;
use crate::shared::path_utils;
use std::path::Path;

/// 生成重命名预览
pub fn generate(parsed_items: &[ParsedMediaInfo], template_str: &str) -> Vec<RenamePreviewItem> {
    let mut preview_items = Vec::new();

    for (index, info) in parsed_items.iter().enumerate() {
        // 渲染模板生成新文件名
        let proposed_name = template::render(info, template_str);

        // 获取原始路径和文件名
        let source_path = info.media_item.file_path.clone();
        let original_name = info.media_item.file_name.clone();

        // 计算目标路径（同目录下）
        let parent_dir = Path::new(&source_path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let target_path = format!("{}\\{}", parent_dir, proposed_name);

        // 评估置信度
        let mut info_clone = info.clone();
        let confidence_result = confidence::evaluate_confidence(&mut info_clone);
        let evidence = confidence_result.evidences.clone();

        // 判断是否需要人工确认
        let needs_manual_review = confidence_result.needs_review;

        // 判断是否跳过（低置信度或需要人工确认）
        let should_skip = needs_manual_review;

        // 创建预览项
        let preview_item = RenamePreviewItem {
            id: format!("preview-{}", index),
            parsed_info: info_clone,
            source_path,
            original_name,
            proposed_name,
            target_path,
            media_type: info.media_type.clone(),
            confidence: confidence_result.score,
            needs_manual_review,
            should_skip,
            conflicts: vec![],
            evidence,
            metadata_source: MetadataSource::LocalRule,
        };

        preview_items.push(preview_item);
    }

    // 检测冲突
    let conflicts = conflict_detector::detect_conflicts(&preview_items);

    // 将冲突分配到对应的预览项
    for conflict in conflicts {
        if let Some(item) = preview_items
            .iter_mut()
            .find(|item| item.source_path == conflict.source_path)
        {
            item.conflicts.push(conflict);
        }
    }

    preview_items
}

/// 使用默认模板生成预览
pub fn generate_with_default_template(parsed_items: &[ParsedMediaInfo]) -> Vec<RenamePreviewItem> {
    // 使用第一个项目的媒体类型决定默认模板
    let default_template = if let Some(first) = parsed_items.first() {
        template::get_default_template(&first.media_type)
    } else {
        "{Title}.{ext}"
    };

    generate(parsed_items, default_template)
}

/// 清理文件名中的非法字符
pub fn sanitize_proposed_name(name: &str) -> String {
    path_utils::sanitize_filename(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::MediaType;
    use crate::scan::MediaItem;

    fn make_test_info(media_type: MediaType, title: &str) -> ParsedMediaInfo {
        ParsedMediaInfo {
            media_item: MediaItem {
                id: "test-id".to_string(),
                file_path: "C:\\test\\file.mkv".to_string(),
                file_name: "file.mkv".to_string(),
                extension: "mkv".to_string(),
                file_size: 0,
                is_video: true,
                is_companion: false,
            },
            media_type,
            title: title.to_string(),
            year: Some(2020),
            season: Some(1),
            episode: Some(5),
            episode_end: None,
            episode_title: Some("Episode Title".to_string()),
            resolution: Some("1080p".to_string()),
            source: Some("BluRay".to_string()),
            video_codec: Some("x264".to_string()),
            audio_codec: Some("AAC".to_string()),
            group: Some("SubGroup".to_string()),
            confidence: 85,
            rule_sources: vec!["test".to_string()],
            special_type: None,
            extra_type: None,
            extra_number: None,
        }
    }

    #[test]
    fn test_generate_movie_preview() {
        let info = make_test_info(MediaType::Movie, "Test Movie");
        let template = "{Title} ({Year}) [{Resolution}]";
        let previews = generate(&[info], template);
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].proposed_name, "Test Movie (2020) [1080p]");
        assert_eq!(previews[0].media_type, MediaType::Movie);
    }

    #[test]
    fn test_generate_series_preview() {
        let info = make_test_info(MediaType::Series, "Test Series");
        let template = "{Title} - S{Season:02}E{Episode:02} - {EpisodeTitle}";
        let previews = generate(&[info], template);
        assert_eq!(previews.len(), 1);
        assert_eq!(
            previews[0].proposed_name,
            "Test Series - S01E05 - Episode Title"
        );
    }

    #[test]
    fn test_generate_anime_preview() {
        let info = make_test_info(MediaType::Anime, "Test Anime");
        let template = "{Title} - S{Season:02}E{Episode:02} [{Group}][{Resolution}]";
        let previews = generate(&[info], template);
        assert_eq!(previews.len(), 1);
        assert_eq!(
            previews[0].proposed_name,
            "Test Anime - S01E05 [SubGroup][1080p]"
        );
    }

    #[test]
    fn test_generate_special_preview() {
        let mut info = make_test_info(MediaType::Special, "Test Special");
        info.season = Some(0);
        info.special_type = Some(crate::parse::movie_parser::SpecialType::Special);
        let template = "{Title} - S00E{Episode:02} - {SpecialType}";
        let previews = generate(&[info], template);
        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].proposed_name, "Test Special - S00E05 - Special");
    }

    #[test]
    fn test_generate_low_confidence_marks_manual_review() {
        let mut info = make_test_info(MediaType::Movie, "Test");
        info.confidence = 50; // 低置信度
        info.title = "T".to_string(); // 短标题，会进一步降低置信度
        let template = "{Title}";
        let previews = generate(&[info], template);
        assert_eq!(previews.len(), 1);
        assert!(previews[0].needs_manual_review);
        assert!(previews[0].should_skip);
    }

    #[test]
    fn test_generate_conflict_items_marked() {
        let info1 = make_test_info(MediaType::Movie, "Movie1");
        let info2 = make_test_info(MediaType::Movie, "Movie2");
        // 两个项目生成相同的目标文件名
        let template = "Same Name ({Year})";
        let previews = generate(&[info1, info2], template);
        assert_eq!(previews.len(), 2);
        // 应该检测到重复目标冲突
        let total_conflicts: usize = previews.iter().map(|p| p.conflicts.len()).sum();
        assert!(total_conflicts > 0);
    }

    #[test]
    fn test_generate_with_default_template() {
        let info = make_test_info(MediaType::Movie, "Default Movie");
        let previews = generate_with_default_template(&[info]);
        assert_eq!(previews.len(), 1);
        // 默认模板包含年份和分辨率
        assert!(previews[0].proposed_name.contains("2020"));
        assert!(previews[0].proposed_name.contains("1080p"));
    }

    #[test]
    fn test_sanitize_proposed_name() {
        assert_eq!(sanitize_proposed_name("test<>file.txt"), "test__file.txt");
        assert_eq!(sanitize_proposed_name("normal_file.txt"), "normal_file.txt");
    }
}
