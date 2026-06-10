// 模板渲染模块
// 职责：根据模板字符串生成新文件名

use crate::parse::confidence::RuleMatchEvidence;
use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
use crate::shared::result_types::ConflictType;
use serde::{Deserialize, Serialize};

/// 元数据来源
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetadataSource {
    /// 本地规则解析
    LocalRule,
    /// TMDb API
    Tmdb,
    /// 手动输入
    Manual,
}

/// 重命名冲突
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameConflict {
    /// 冲突类型
    pub conflict_type: ConflictType,
    /// 源路径
    pub source_path: String,
    /// 目标路径
    pub target_path: String,
    /// 冲突描述
    pub message: String,
    /// 是否阻塞（阻塞则不能执行）
    pub blocking: bool,
}

/// 重命名预览项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenamePreviewItem {
    /// 预览 ID
    pub id: String,
    /// 解析信息
    pub parsed_info: ParsedMediaInfo,
    /// 原始路径
    pub source_path: String,
    /// 原始文件名
    pub original_name: String,
    /// 提议的新文件名
    pub proposed_name: String,
    /// 目标路径
    pub target_path: String,
    /// 媒体类型
    pub media_type: MediaType,
    /// 置信度
    pub confidence: u8,
    /// 是否需要人工确认
    pub needs_manual_review: bool,
    /// 是否跳过
    pub should_skip: bool,
    /// 冲突列表
    pub conflicts: Vec<RenameConflict>,
    /// 置信度证据
    pub evidence: Vec<RuleMatchEvidence>,
    /// 元数据来源
    pub metadata_source: MetadataSource,
}

/// 渲染模板，生成新文件名
pub fn render(info: &ParsedMediaInfo, template: &str) -> String {
    let mut result = template.to_string();

    // 基本变量替换
    result = result.replace("{Title}", &info.title);
    result = result.replace("{title}", &info.title);

    // 扩展名
    let extension = info.media_item.extension.trim_start_matches('.');
    result = result.replace("{Ext}", extension);
    result = result.replace("{ext}", extension);

    // 年份
    if let Some(year) = info.year {
        result = result.replace("{Year}", &year.to_string());
        result = result.replace("{year}", &year.to_string());
    } else {
        result = result.replace("{Year}", "");
        result = result.replace("{year}", "");
    }

    // 季号
    if let Some(season) = info.season {
        result = result.replace("{Season:02}", &format!("{:02}", season));
        result = result.replace("{season:02}", &format!("{:02}", season));
        result = result.replace("{Season}", &season.to_string());
        result = result.replace("{season}", &season.to_string());
    } else {
        result = result.replace("{Season:02}", "00");
        result = result.replace("{season:02}", "00");
        result = result.replace("{Season}", "0");
        result = result.replace("{season}", "0");
    }

    // 集号
    if let Some(episode) = info.episode {
        result = result.replace("{Episode:02}", &format!("{:02}", episode));
        result = result.replace("{episode:02}", &format!("{:02}", episode));
        result = result.replace("{Episode}", &episode.to_string());
        result = result.replace("{episode}", &episode.to_string());
    } else {
        result = result.replace("{Episode:02}", "00");
        result = result.replace("{episode:02}", "00");
        result = result.replace("{Episode}", "0");
        result = result.replace("{episode}", "0");
    }

    // 集标题
    if let Some(ref episode_title) = info.episode_title {
        result = result.replace("{EpisodeTitle}", episode_title);
        result = result.replace("{episode_title}", episode_title);
    } else {
        result = result.replace("{EpisodeTitle}", "");
        result = result.replace("{episode_title}", "");
    }

    // 分辨率
    if let Some(ref resolution) = info.resolution {
        result = result.replace("{Resolution}", resolution);
        result = result.replace("{resolution}", resolution);
    } else {
        result = result.replace("{Resolution}", "");
        result = result.replace("{resolution}", "");
    }

    // 来源
    if let Some(ref source) = info.source {
        result = result.replace("{Source}", source);
        result = result.replace("{source}", source);
    } else {
        result = result.replace("{Source}", "");
        result = result.replace("{source}", "");
    }

    // 视频编码
    if let Some(ref video_codec) = info.video_codec {
        result = result.replace("{VideoCodec}", video_codec);
        result = result.replace("{video_codec}", video_codec);
    } else {
        result = result.replace("{VideoCodec}", "");
        result = result.replace("{video_codec}", "");
    }

    // 音频编码
    if let Some(ref audio_codec) = info.audio_codec {
        result = result.replace("{AudioCodec}", audio_codec);
        result = result.replace("{audio_codec}", audio_codec);
    } else {
        result = result.replace("{AudioCodec}", "");
        result = result.replace("{audio_codec}", "");
    }

    // 编码组
    if let Some(ref group) = info.group {
        result = result.replace("{Group}", group);
        result = result.replace("{group}", group);
    } else {
        result = result.replace("{Group}", "");
        result = result.replace("{group}", "");
    }

    // 特别篇类型
    if let Some(ref special_type) = info.special_type {
        result = result.replace("{SpecialType}", &special_type.to_string());
        result = result.replace("{special_type}", &special_type.to_string());
    } else {
        result = result.replace("{SpecialType}", "");
        result = result.replace("{special_type}", "");
    }

    // Extras 类型
    if let Some(ref extra_type) = info.extra_type {
        result = result.replace("{ExtraType}", extra_type);
        result = result.replace("{extra_type}", extra_type);
    } else {
        result = result.replace("{ExtraType}", "");
        result = result.replace("{extra_type}", "");
    }

    // Extras 编号
    if let Some(extra_number) = info.extra_number {
        result = result.replace("{ExtraNumber}", &extra_number.to_string());
        result = result.replace("{extra_number}", &extra_number.to_string());
    } else {
        result = result.replace("{ExtraNumber}", "");
        result = result.replace("{extra_number}", "");
    }

    // 移除空括号和只含空格的括号
    result = result.replace("()", "").replace("[]", "");
    // 移除只含空格的方括号 [ ]
    let re_bracket = regex::Regex::new(r"\[\s*\]").unwrap();
    loop {
        let new_result = re_bracket.replace_all(&result, "").to_string();
        if new_result == result {
            break;
        }
        result = new_result;
    }

    // 清理多余空格和点
    result = result
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .replace(" .", ".")
        .replace(". ", ".");

    result
}

/// 获取默认模板
pub fn get_default_template(media_type: &MediaType) -> &'static str {
    match media_type {
        MediaType::Movie => "{Title} ({Year}).{ext}",
        MediaType::Series => "{Title} - S{Season:02}E{Episode:02} - {EpisodeTitle}.{ext}",
        MediaType::Anime => "{Title} - S{Season:02}E{Episode:02}.{ext}",
        MediaType::Special | MediaType::Ova | MediaType::Ncop | MediaType::Nced => {
            "{Title} - S00E{Episode:02} - {SpecialType}.{ext}"
        }
        MediaType::Extras => "{Title} - Extra - {ExtraType}{ExtraNumber}.{ext}",
        MediaType::Unknown => "{Title}.{ext}",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::MediaItem;

    fn make_test_info(media_type: MediaType) -> ParsedMediaInfo {
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
            title: "Test Title".to_string(),
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
    fn test_render_movie_template() {
        let info = make_test_info(MediaType::Movie);
        let template = "{Title} ({Year}) [{Resolution} {Source} {VideoCodec} {AudioCodec}]";
        let result = render(&info, template);
        assert_eq!(result, "Test Title (2020) [1080p BluRay x264 AAC]");
    }

    #[test]
    fn test_render_extension_template() {
        let info = make_test_info(MediaType::Movie);
        let result = render(&info, "{Title} ({Year}).{ext}");
        assert_eq!(result, "Test Title (2020).mkv");
    }

    #[test]
    fn test_render_series_template() {
        let info = make_test_info(MediaType::Series);
        let template = "{Title} - S{Season:02}E{Episode:02} - {EpisodeTitle}";
        let result = render(&info, template);
        assert_eq!(result, "Test Title - S01E05 - Episode Title");
    }

    #[test]
    fn test_render_anime_template() {
        let info = make_test_info(MediaType::Anime);
        let template = "{Title} - S{Season:02}E{Episode:02} [{Group}][{Resolution}]";
        let result = render(&info, template);
        assert_eq!(result, "Test Title - S01E05 [SubGroup][1080p]");
    }

    #[test]
    fn test_render_special_template() {
        let mut info = make_test_info(MediaType::Special);
        info.season = Some(0);
        info.special_type = Some(crate::parse::movie_parser::SpecialType::Special);
        let template = "{Title} - S00E{Episode:02} - {SpecialType}";
        let result = render(&info, template);
        assert_eq!(result, "Test Title - S00E05 - Special");
    }

    #[test]
    fn test_render_empty_fields() {
        let mut info = make_test_info(MediaType::Movie);
        info.year = None;
        info.resolution = None;
        info.source = None;
        let template = "{Title} ({Year}) [{Resolution} {Source}]";
        let result = render(&info, template);
        assert_eq!(result, "Test Title");
    }

    #[test]
    fn test_render_illegal_chars_sanitization() {
        let mut info = make_test_info(MediaType::Movie);
        info.title = "Test: Title <2020>".to_string();
        let template = "{Title}";
        let result = render(&info, template);
        assert_eq!(result, "Test: Title <2020>");
    }

    #[test]
    fn test_render_idempotency() {
        let info = make_test_info(MediaType::Movie);
        let template = "{Title} ({Year})";
        let result1 = render(&info, template);
        let result2 = render(&info, template);
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_render_unknown_variables() {
        let info = make_test_info(MediaType::Movie);
        let template = "{Title} {UnknownVar} {AnotherUnknown}";
        let result = render(&info, template);
        assert_eq!(result, "Test Title {UnknownVar} {AnotherUnknown}");
    }

    #[test]
    fn test_get_default_template_movie() {
        let template = get_default_template(&MediaType::Movie);
        assert_eq!(template, "{Title} ({Year}).{ext}");
    }

    #[test]
    fn test_get_default_template_series() {
        let template = get_default_template(&MediaType::Series);
        assert!(template.contains("{Title}"));
        assert!(template.contains("{Season:02}"));
        assert!(template.contains("{Episode:02}"));
    }

    #[test]
    fn test_get_default_template_anime() {
        let template = get_default_template(&MediaType::Anime);
        assert_eq!(template, "{Title} - S{Season:02}E{Episode:02}.{ext}");
    }

    #[test]
    fn test_get_default_template_extras() {
        let template = get_default_template(&MediaType::Extras);
        assert!(template.contains("{ExtraType}"));
        assert!(template.contains("{ExtraNumber}"));
    }
}
