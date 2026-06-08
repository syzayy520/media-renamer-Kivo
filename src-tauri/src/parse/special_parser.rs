// 特别篇解析器模块
// 职责：解析特别篇/SP/OVA/OAD/NCOP/NCED/Extras 文件名
// 特别篇不得误判为普通剧集

use super::movie_parser::{MediaType, ParsedMediaInfo, SpecialType};
use crate::scan::MediaItem;
use regex::Regex;

/// 解析特别篇文件名
pub fn parse_special(filename: &str) -> Option<ParsedMediaInfo> {
    // 尝试多种模式，取第一个匹配
    // Extras 必须在 keyword 之前，避免 "Extra - NCOP01" 被误判为 NCOP
    parse_s00_episode(filename)
        .or_else(|| parse_extras(filename))
        .or_else(|| parse_keyword_special(filename))
}

/// 模式1：Title - S00E01 - Special Title
fn parse_s00_episode(filename: &str) -> Option<ParsedMediaInfo> {
    let re = Regex::new(
        r"(?x)
        ^(?P<title>.+?)                    # 标题
        [\s\.\-_]+                         # 分隔符
        S00E(?P<episode>\d{1,3})           # S00E01
        (?:[\s\.\-_]+                      # 分隔符
        (?P<episode_title>.+?))?           # 集标题（可选）
        (?:[\s\.\[\]]+                     # token 分隔符
        (?:
            (?P<resolution>\d{3,4}p|4K|2160p)
            |(?P<source>BluRay|WEB-DL|WEBRip|HDTV|DVD|Remux)
            |(?P<video_codec>x264|x265|H\.?264|H\.?265|HEVC|AV1)
            |(?P<audio_codec>AAC|DTS|FLAC|AC3|Atmos|TrueHD)
        )
        \]?)*
        $                                  # 字符串结尾
        ",
    )
    .ok()?;

    let caps = re.captures(filename)?;
    let title = caps.name("title")?.as_str().trim().to_string();
    let episode = caps.name("episode")?.as_str().parse::<u16>().ok()?;
    let episode_title = caps
        .name("episode_title")
        .map(|m| m.as_str().trim().to_string());
    let resolution = caps.name("resolution").map(|m| m.as_str().to_string());
    let source = caps.name("source").map(|m| m.as_str().to_string());
    let video_codec = caps.name("video_codec").map(|m| m.as_str().to_string());
    let audio_codec = caps.name("audio_codec").map(|m| m.as_str().to_string());

    if title.is_empty() {
        return None;
    }

    Some(ParsedMediaInfo {
        media_item: make_media_item(filename),
        media_type: MediaType::Special,
        title,
        year: None,
        season: Some(0),
        episode: Some(episode),
        episode_end: None,
        episode_title,
        resolution,
        source,
        video_codec,
        audio_codec,
        group: None,
        confidence: 90,
        rule_sources: vec!["special_parser:s00e".to_string()],
        special_type: Some(SpecialType::Special),
        extra_type: None,
        extra_number: None,
    })
}

/// 模式2：SP/OVA/OAD/NCOP/NCED 关键字
fn parse_keyword_special(filename: &str) -> Option<ParsedMediaInfo> {
    let re = Regex::new(
        r"(?x)
        ^(?P<title>.+?)                    # 标题
        [\s\.\-_]+                         # 分隔符
        (?P<keyword>SP|OVA|OAD|NCOP|NCED)  # 关键字
        (?P<number>\d{1,3})?               # 编号（可选）
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
    let title = caps.name("title")?.as_str().trim().to_string();
    let keyword = caps.name("keyword")?.as_str().to_uppercase();
    let number = caps
        .name("number")
        .and_then(|m| m.as_str().parse::<u16>().ok());
    let resolution = caps.name("resolution").map(|m| m.as_str().to_string());
    let source = caps.name("source").map(|m| m.as_str().to_string());
    let video_codec = caps.name("video_codec").map(|m| m.as_str().to_string());
    let audio_codec = caps.name("audio_codec").map(|m| m.as_str().to_string());

    if title.is_empty() {
        return None;
    }

    let (media_type, special_type) = match keyword.as_str() {
        "SP" => (MediaType::Special, SpecialType::Sp),
        "OVA" => (MediaType::Ova, SpecialType::Ova),
        "OAD" => (MediaType::Special, SpecialType::Special), // OAD 归类为 Special
        "NCOP" => (MediaType::Ncop, SpecialType::Ncop),
        "NCED" => (MediaType::Nced, SpecialType::Nced),
        _ => return None,
    };

    let confidence = if number.is_some() { 90 } else { 85 };

    Some(ParsedMediaInfo {
        media_item: make_media_item(filename),
        media_type,
        title,
        year: None,
        season: Some(0),
        episode: number,
        episode_end: None,
        episode_title: None,
        resolution,
        source,
        video_codec,
        audio_codec,
        group: None,
        confidence,
        rule_sources: vec![format!("special_parser:{}", keyword.to_lowercase())],
        special_type: Some(special_type),
        extra_type: None,
        extra_number: None,
    })
}

/// 模式3：Extras 格式 Title - Extra - NCOP01
fn parse_extras(filename: &str) -> Option<ParsedMediaInfo> {
    let re = Regex::new(
        r"(?x)
        ^(?P<title>.+?)                    # 标题
        [\s\.\-_]+                         # 分隔符
        Extra                              # Extra 关键字
        [\s\.\-_]+                         # 分隔符
        (?P<extra_type>\w+?)               # Extra 类型
        (?P<extra_number>\d{1,3})          # Extra 编号
        ",
    )
    .ok()?;

    let caps = re.captures(filename)?;
    let title = caps.name("title")?.as_str().trim().to_string();
    let extra_type = caps.name("extra_type").map(|m| m.as_str().to_string());
    let extra_number = caps
        .name("extra_number")
        .and_then(|m| m.as_str().parse::<u16>().ok());

    if title.is_empty() {
        return None;
    }

    Some(ParsedMediaInfo {
        media_item: make_media_item(filename),
        media_type: MediaType::Extras,
        title,
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
        confidence: 80,
        rule_sources: vec!["special_parser:extras".to_string()],
        special_type: Some(SpecialType::Extra),
        extra_type,
        extra_number,
    })
}

/// 构造 MediaItem
fn make_media_item(filename: &str) -> MediaItem {
    MediaItem {
        id: uuid::Uuid::new_v4().to_string(),
        file_path: String::new(),
        file_name: filename.to_string(),
        extension: String::new(),
        file_size: 0,
        is_video: true,
        is_companion: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_special_s00e01() {
        let result = parse_special("Naruto - S00E01 - Prologue");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Special);
        assert_eq!(info.season, Some(0));
        assert_eq!(info.episode, Some(1));
        assert_eq!(info.episode_title, Some("Prologue".to_string()));
    }

    #[test]
    fn test_parse_special_sp() {
        let result = parse_special("Attack on Titan - SP01");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Special);
        assert_eq!(info.special_type, Some(SpecialType::Sp));
        assert_eq!(info.episode, Some(1));
    }

    #[test]
    fn test_parse_special_ova() {
        let result = parse_special("Sword Art Online - OVA01");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Ova);
        assert_eq!(info.special_type, Some(SpecialType::Ova));
    }

    #[test]
    fn test_parse_special_oad() {
        let result = parse_special("Shingeki no Kyojin - OAD03");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Special);
        assert_eq!(info.special_type, Some(SpecialType::Special));
        assert_eq!(info.episode, Some(3));
    }

    #[test]
    fn test_parse_special_ncop() {
        let result = parse_special("JoJo - NCOP01");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Ncop);
        assert_eq!(info.special_type, Some(SpecialType::Ncop));
    }

    #[test]
    fn test_parse_special_nced() {
        let result = parse_special("JoJo - NCED02");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Nced);
        assert_eq!(info.special_type, Some(SpecialType::Nced));
        assert_eq!(info.episode, Some(2));
    }

    #[test]
    fn test_parse_extras() {
        let result = parse_special("Naruto - Extra - NCOP01");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Extras);
        assert_eq!(info.special_type, Some(SpecialType::Extra));
    }

    #[test]
    fn test_parse_special_no_match() {
        // 普通剧集不应被识别为特别篇
        assert!(parse_special("Breaking Bad - S01E01 - Pilot").is_none());
        // 普通电影不应被识别为特别篇
        assert!(parse_special("The Matrix (1999)").is_none());
        // 随机文件名
        assert!(parse_special("random_file.mkv").is_none());
    }

    #[test]
    fn test_parse_special_s00_with_tokens() {
        let result = parse_special("Title - S00E05 - Special [1080p] [BluRay]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.source, Some("BluRay".to_string()));
    }
}
