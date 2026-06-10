// rename/template/render.rs
// 职责：根据模板字符串渲染新文件名
// 委托：extension_token.rs 处理扩展名规范化

use super::extension_token;
use crate::parse::movie_parser::ParsedMediaInfo;

/// 渲染模板，生成新文件名
///
/// 支持的令牌：
/// - {title}/{Title} — 标题
/// - {ext}/{Ext} — 文件扩展名（含前导点，无扩展名时为空）
/// - {year}/{Year} — 年份
/// - {season:02}/{Season:02} — 季号（补零）
/// - {episode:02}/{Episode:02} — 集号（补零）
/// - {EpisodeTitle}/{episode_title} — 集标题
/// - {Resolution}/{resolution} — 分辨率
/// - {Source}/{source} — 来源
/// - {VideoCodec}/{video_codec} — 视频编码
/// - {AudioCodec}/{audio_codec} — 音频编码
/// - {Group}/{group} — 编码组
/// - {SpecialType}/{special_type} — 特别篇类型
/// - {ExtraType}/{extra_type} — Extras 类型
/// - {ExtraNumber}/{extra_number} — Extras 编号
///
/// # 关键行为
/// - {ext} 渲染为 ".mkv"（含前导点）或空字符串 — 不会产生尾随点
/// - 模板不应使用字面量 ".{ext}"；应使用 "{ext}"（含前导点的令牌）
/// - 渲染后清理连续空格、点修复
pub fn render(info: &ParsedMediaInfo, template: &str) -> String {
    let mut result = template.to_string();

    // 基本变量替换
    result = result.replace("{Title}", &info.title);
    result = result.replace("{title}", &info.title);

    // 扩展名 — 使用 extension_token 规范化
    // {ext} 小写，{Ext} 大写
    let extension = extension_token::ext_token_value(&info.media_item.extension);
    let extension_upper = extension_token::ext_token_value_uppercase(&info.media_item.extension);
    result = result.replace("{Ext}", &extension_upper);
    result = result.replace("{ext}", &extension);

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

    // 清理多余空格和点连接
    result = result
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .replace(" .", ".");

    // 修复可能由扩展名规范化产生的连续点
    // 例: 模板 ".{ext}"  + extension_token 输出 ".mkv" → "..mkv" → ".mkv"
    result = result.replace("..", ".");

    // 移除尾随点（扩展名为空时遗留）
    result = result.trim_end_matches('.').to_string();

    // 二次清理：尾随点移除后再修复 "." 结尾后的空格
    // （已在首次处理中覆盖）

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::MediaType;
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

    fn make_info_empty_ext() -> ParsedMediaInfo {
        let mut info = make_test_info(MediaType::Movie);
        info.title = "第一滴血".to_string();
        info.media_item.extension = String::new();
        info.media_item.file_name = "第一滴血".to_string();
        info.year = Some(1982);
        info
    }

    fn make_info_dot_ext() -> ParsedMediaInfo {
        let mut info = make_test_info(MediaType::Movie);
        info.title = "第一滴血".to_string();
        info.media_item.extension = ".mkv".to_string();
        info.media_item.file_name = "第一滴血.mkv".to_string();
        info.year = Some(1982);
        info
    }

    // ─── 基本渲染测试 ──────────────────────────────────────────

    #[test]
    fn test_render_movie_template() {
        let info = make_test_info(MediaType::Movie);
        let template = "{Title} ({Year}) [{Resolution} {Source} {VideoCodec} {AudioCodec}]";
        let result = render(&info, template);
        assert_eq!(result, "Test Title (2020) [1080p BluRay x264 AAC]");
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

    // ─── 扩展名测试 — Target A 核心修复 ────────────────────────────

    #[test]
    fn test_movie_default_template_renders_real_extension() {
        let info = make_test_info(MediaType::Movie);
        // {ext} 现在输出 ".mkv"（含前导点）
        let result = render(&info, "{Title} ({Year}){ext}");
        assert_eq!(result, "Test Title (2020).mkv");
    }

    #[test]
    fn test_movie_default_template_omits_dangling_dot_when_extension_empty() {
        let info = make_info_empty_ext();
        // 没有扩展名 → {ext} 输出空字符串 → 无尾随点
        let result = render(&info, "{Title} ({Year}){ext}");
        assert_eq!(result, "第一滴血 (1982)");
        assert!(!result.ends_with('.'));
    }

    #[test]
    fn test_lowercase_ext_works() {
        let info = make_test_info(MediaType::Movie);
        let result = render(&info, "{title}{ext}");
        assert_eq!(result, "Test Title.mkv");
    }

    #[test]
    fn test_uppercase_ext_works() {
        let info = make_test_info(MediaType::Movie);
        let result = render(&info, "{Title}{Ext}");
        assert_eq!(result, "Test Title.MKV");
    }

    #[test]
    fn test_extension_with_leading_dot_does_not_become_double_dot() {
        let info = make_info_dot_ext();
        // .mkv → {ext} 输出 ".mkv"，不是 "..mkv"
        let result = render(&info, "{Title} ({Year}){ext}");
        assert_eq!(result, "第一滴血 (1982).mkv");
        assert!(!result.contains(".."));
    }

    #[test]
    fn test_extension_empty_all_variants() {
        // 空字符串
        let mut info = make_test_info(MediaType::Movie);
        info.media_item.extension = String::new();
        let result1 = render(&info, "{Title} ({Year}){ext}");
        assert_eq!(result1, "Test Title (2020)");
        assert!(!result1.ends_with('.'));

        // 仅含空白（trim 后为空）
        info.media_item.extension = "   ".to_string();
        let result2 = render(&info, "{Title} ({Year}){ext}");
        assert!(!result2.ends_with('.'));
    }

    #[test]
    fn test_no_ext_with_series_template_safe() {
        let mut info = make_test_info(MediaType::Series);
        info.media_item.extension = String::new();
        let result = render(
            &info,
            "{Title} - S{Season:02}E{Episode:02} - {EpisodeTitle}{ext}",
        );
        assert_eq!(result, "Test Title - S01E05 - Episode Title");
        assert!(!result.ends_with('.'));
    }

    #[test]
    fn test_legacy_dot_ext_template_also_works() {
        // 向后兼容：旧模板 "{Title}.{ext}" — {ext} 输出 ".mkv" → "..mkv" → 修复为 ".mkv"
        let info = make_test_info(MediaType::Movie);
        let result = render(&info, "{Title} ({Year}).{ext}");
        assert_eq!(result, "Test Title (2020).mkv");
    }

    #[test]
    fn test_legacy_dot_ext_empty_extension_no_trailing_dot() {
        let info = make_info_empty_ext();
        let result = render(&info, "{Title} ({Year}).{ext}");
        assert_eq!(result, "第一滴血 (1982)");
        assert!(!result.ends_with('.'));
    }
}
