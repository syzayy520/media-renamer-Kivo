// 动漫解析器模块
// 职责：解析动漫文件名
// 支持格式：[Group] Title - 01 [1080p][x264]、Title - 01、Title.E01、Title - 01v2

use super::movie_parser::{MediaType, ParsedMediaInfo};
use crate::scan::MediaItem;
use regex::Regex;

/// 解析动漫文件名
pub fn parse_anime(filename: &str) -> Option<ParsedMediaInfo> {
    // 尝试多种模式，取第一个匹配
    parse_with_group_episode(filename)
        .or_else(|| parse_with_simple_episode(filename))
        .or_else(|| parse_with_dot_episode(filename))
}

/// 模式1：[Group] Title - 01[v2] [1080p][x264]
fn parse_with_group_episode(filename: &str) -> Option<ParsedMediaInfo> {
    let re = Regex::new(
        r"(?x)
        ^\[(?P<group>[^\]]+)\]            # [Group]
        \s*(?P<title>.+?)                  # 标题
        \s*-\s*                            # 分隔符
        (?P<episode>\d{1,4})               # 集号
        (?P<version>v\d)?                  # 版本号（可选）
        (?:[\s\.\[\]]+                     # token 分隔符
        (?:
            (?P<resolution>\d{3,4}p|4K|2160p)
            |(?P<source>BluRay|WEB-DL|WEBRip|HDTV|DVD|Remux)
            |(?P<video_codec>x264|x265|H\.?264|H\.?265|HEVC|AV1)
            |(?P<audio_codec>AAC|DTS|FLAC|AC3|Atmos|TrueHD)
        )
        \]?)*
        ",
    )
    .ok()?;

    let caps = re.captures(filename)?;
    build_anime_info(filename, &caps)
}

/// 模式2：Title - 01[v2]（无 Group）
fn parse_with_simple_episode(filename: &str) -> Option<ParsedMediaInfo> {
    let re = Regex::new(
        r"(?x)
        ^(?P<title>.+?)                    # 标题
        \s*-\s*                            # 分隔符
        (?P<episode>\d{1,4})               # 集号
        (?P<version>v\d)?                  # 版本号（可选）
        (?:[\s\.\[\]]+                     # token 分隔符
        (?:
            (?P<resolution>\d{3,4}p|4K|2160p)
            |(?P<source>BluRay|WEB-DL|WEBRip|HDTV|DVD|Remux)
            |(?P<video_codec>x264|x265|H\.?264|H\.?265|HEVC|AV1)
            |(?P<audio_codec>AAC|DTS|FLAC|AC3|Atmos|TrueHD)
        )
        \]?)*
        ",
    )
    .ok()?;

    let caps = re.captures(filename)?;

    // 排除 S01E01 格式（那是剧集）
    if filename.contains("S01E") || filename.contains("s01e") {
        return None;
    }

    build_anime_info(filename, &caps)
}

/// 模式3：Title.E01（点号分隔，E 前无 S）
fn parse_with_dot_episode(filename: &str) -> Option<ParsedMediaInfo> {
    let re = Regex::new(
        r"(?x)
        ^(?P<title>.+?)                    # 标题
        \.                                 # 点号分隔
        E(?P<episode>\d{1,4})              # E + 集号
        (?P<version>v\d)?                  # 版本号（可选）
        (?:[\s\.\[\]]+                     # token 分隔符
        (?:
            (?P<resolution>\d{3,4}p|4K|2160p)
            |(?P<source>BluRay|WEB-DL|WEBRip|HDTV|DVD|Remux)
            |(?P<video_codec>x264|x265|H\.?264|H\.?265|HEVC|AV1)
            |(?P<audio_codec>AAC|DTS|FLAC|AC3|Atmos|TrueHD)
        )
        \]?)*
        ",
    )
    .ok()?;

    let caps = re.captures(filename)?;

    // 排除 S01E01 格式
    if regex::Regex::new(r"(?i)S\d{1,2}E\d")
        .ok()
        .and_then(|re| re.find(filename))
        .is_some()
    {
        return None;
    }

    build_anime_info(filename, &caps)
}

/// 从 regex captures 构建 ParsedMediaInfo
fn build_anime_info(filename: &str, caps: &regex::Captures) -> Option<ParsedMediaInfo> {
    let title = caps.name("title")?.as_str().trim().to_string();
    let episode = caps.name("episode")?.as_str().parse::<u16>().ok()?;
    let group = caps.name("group").map(|m| m.as_str().to_string());
    let version = caps.name("version").map(|m| m.as_str().to_string());
    let resolution = caps.name("resolution").map(|m| m.as_str().to_string());
    let source = caps.name("source").map(|m| m.as_str().to_string());
    let video_codec = caps.name("video_codec").map(|m| m.as_str().to_string());
    let audio_codec = caps.name("audio_codec").map(|m| m.as_str().to_string());

    if title.is_empty() {
        return None;
    }

    // 低置信度情况：无 group 且无 resolution → 可能不是动漫
    let mut confidence: u8 = 85;
    if group.is_some() {
        confidence = 90;
    }
    if group.is_some() && resolution.is_some() {
        confidence = 95;
    }
    if group.is_none() && resolution.is_none() {
        confidence = 70;
    }

    let media_item = MediaItem {
        id: uuid::Uuid::new_v4().to_string(),
        file_path: String::new(),
        file_name: filename.to_string(),
        extension: String::new(),
        file_size: 0,
        is_video: true,
        is_companion: false,
    };

    // 将版本号信息附加到 rule_sources
    let mut rule_sources = vec!["anime_parser".to_string()];
    if let Some(v) = version {
        rule_sources.push(format!("version:{}", v));
    }

    Some(ParsedMediaInfo {
        media_item,
        media_type: MediaType::Anime,
        title,
        year: None,
        season: Some(1), // 动漫默认 Season 1
        episode: Some(episode),
        episode_end: None,
        episode_title: None,
        resolution,
        source,
        video_codec,
        audio_codec,
        group,
        confidence,
        rule_sources,
        special_type: None,
        extra_type: None,
        extra_number: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_anime_group_basic() {
        let result = parse_anime("[SubGroup] Attack on Titan - 01 [1080p]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Attack on Titan");
        assert_eq!(info.episode, Some(1));
        assert_eq!(info.group, Some("SubGroup".to_string()));
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.media_type, MediaType::Anime);
    }

    #[test]
    fn test_parse_anime_group_with_codec() {
        let result = parse_anime("[FFF] Naruto Shippuuden - 42 [1080p][x264][AAC]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.group, Some("FFF".to_string()));
        assert_eq!(info.episode, Some(42));
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.video_codec, Some("x264".to_string()));
        assert_eq!(info.audio_codec, Some("AAC".to_string()));
    }

    #[test]
    fn test_parse_anime_simple_episode() {
        let result = parse_anime("One Piece - 1050");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "One Piece");
        assert_eq!(info.episode, Some(1050));
        assert_eq!(info.group, None);
    }

    #[test]
    fn test_parse_anime_dot_episode() {
        let result = parse_anime("Bleach.E366");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Bleach");
        assert_eq!(info.episode, Some(366));
    }

    #[test]
    fn test_parse_anime_versioned() {
        let result = parse_anime("[SubGroup] Title - 05v2 [720p]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.episode, Some(5));
        assert!(info.rule_sources.iter().any(|r| r.contains("v2")));
    }

    #[test]
    fn test_parse_anime_no_match() {
        assert!(parse_anime("random file name.mkv").is_none());
        assert!(parse_anime("The Matrix (1999)").is_none());
        assert!(parse_anime("Breaking Bad - S01E01 - Pilot").is_none());
    }

    #[test]
    fn test_parse_anime_low_confidence() {
        // 无 group、无 resolution → 低置信度
        let result = parse_anime("Some Show - 01");
        assert!(result.is_some());
        let info = result.unwrap();
        assert!(info.confidence <= 70);
    }

    #[test]
    fn test_parse_anime_high_confidence() {
        // 有 group + resolution → 高置信度
        let result = parse_anime("[SubGroup] Title - 01 [1080p]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert!(info.confidence >= 90);
    }
}
