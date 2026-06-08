// 电影解析器模块
// 职责：解析电影文件名

use crate::scan::MediaItem;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// 解析后的媒体信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedMediaInfo {
    /// 原始文件
    pub media_item: MediaItem,
    /// 媒体类型
    pub media_type: MediaType,
    /// 标题
    pub title: String,
    /// 年份
    pub year: Option<u16>,
    /// 季号
    pub season: Option<u16>,
    /// 集号
    pub episode: Option<u16>,
    /// 多集结束号
    pub episode_end: Option<u16>,
    /// 集标题
    pub episode_title: Option<String>,
    /// 分辨率
    pub resolution: Option<String>,
    /// 来源
    pub source: Option<String>,
    /// 视频编码
    pub video_codec: Option<String>,
    /// 音频编码
    pub audio_codec: Option<String>,
    /// 编码组
    pub group: Option<String>,
    /// 置信度 (0-100)
    pub confidence: u8,
    /// 规则命中来源
    pub rule_sources: Vec<String>,
    /// 特别篇类型
    pub special_type: Option<SpecialType>,
    /// Extras 类型
    pub extra_type: Option<String>,
    /// Extras 编号
    pub extra_number: Option<u16>,
}

/// 媒体类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Movie,
    Series,
    Anime,
    Special,
    Ova,
    Ncop,
    Nced,
    Extras,
    Unknown,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::Movie => write!(f, "Movie"),
            MediaType::Series => write!(f, "Series"),
            MediaType::Anime => write!(f, "Anime"),
            MediaType::Special => write!(f, "Special"),
            MediaType::Ova => write!(f, "OVA"),
            MediaType::Ncop => write!(f, "NCOP"),
            MediaType::Nced => write!(f, "NCED"),
            MediaType::Extras => write!(f, "Extras"),
            MediaType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// 特别篇类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpecialType {
    Special,
    Sp,
    Ova,
    Ncop,
    Nced,
    Extra,
}

impl std::fmt::Display for SpecialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialType::Special => write!(f, "Special"),
            SpecialType::Sp => write!(f, "SP"),
            SpecialType::Ova => write!(f, "OVA"),
            SpecialType::Ncop => write!(f, "NCOP"),
            SpecialType::Nced => write!(f, "NCED"),
            SpecialType::Extra => write!(f, "Extra"),
        }
    }
}

/// 解析电影文件名
pub fn parse_movie(filename: &str) -> Option<ParsedMediaInfo> {
    // 电影格式：Title (Year) [Resolution] [Source] [Codec]
    // 支持格式：括号/方括号包裹年份，方括号/空格分隔的 token
    let re = Regex::new(
        r"(?x)
        ^(?P<title>.+?)                              # 标题（非贪婪）
        [\s\.]+                                       # 分隔符
        [\(\[]?                                       # 年份左括号（可选）
        (?P<year>19\d{2}|20\d{2})                     # 年份
        [\)\]]?                                       # 年份右括号（可选）
        (?:[\s\.\[\]]+                                # 分隔符（空格/点/方括号）
        (?:                                           # token 组
            (?P<resolution>\d{3,4}p|4K|2160p)         # 分辨率
            |(?P<source>BluRay|WEB-DL|WEBRip|HDTV|DVD|Remux)  # 来源
            |(?P<video_codec>x264|x265|H\.?264|H\.?265|HEVC|AV1)  # 视频编码
            |(?P<audio_codec>AAC|DTS|FLAC|AC3|Atmos|TrueHD)  # 音频编码
        )
        \]?                                           # 可选右方括号
        )*                                            # 可重复多个 token
        ",
    )
    .ok()?;

    let caps = re.captures(filename)?;

    let title = caps.name("title")?.as_str().trim().to_string();
    let year = caps.name("year")?.as_str().parse::<u16>().ok();
    let resolution = caps.name("resolution").map(|m| m.as_str().to_string());
    let source = caps.name("source").map(|m| m.as_str().to_string());
    let video_codec = caps.name("video_codec").map(|m| m.as_str().to_string());
    let audio_codec = caps.name("audio_codec").map(|m| m.as_str().to_string());

    // 需要有标题和年份才算电影
    if title.is_empty() || year.is_none() {
        return None;
    }

    // 创建一个虚拟的 MediaItem
    let media_item = MediaItem {
        id: uuid::Uuid::new_v4().to_string(),
        file_path: String::new(),
        file_name: filename.to_string(),
        extension: String::new(),
        file_size: 0,
        is_video: true,
        is_companion: false,
    };

    Some(ParsedMediaInfo {
        media_item,
        media_type: MediaType::Movie,
        title,
        year,
        season: None,
        episode: None,
        episode_end: None,
        episode_title: None,
        resolution,
        source,
        video_codec,
        audio_codec,
        group: None,
        confidence: 85, // 基础置信度
        rule_sources: vec!["movie_parser".to_string()],
        special_type: None,
        extra_type: None,
        extra_number: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_movie_basic() {
        let result = parse_movie("The Matrix (1999) [1080p] [BluRay] [x264] [AAC]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "The Matrix");
        assert_eq!(info.year, Some(1999));
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.source, Some("BluRay".to_string()));
        assert_eq!(info.video_codec, Some("x264".to_string()));
        assert_eq!(info.audio_codec, Some("AAC".to_string()));
    }

    #[test]
    fn test_parse_movie_no_year() {
        let result = parse_movie("The Matrix");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_movie_minimal() {
        let result = parse_movie("Inception (2010)");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Inception");
        assert_eq!(info.year, Some(2010));
        assert_eq!(info.resolution, None);
        assert_eq!(info.source, None);
    }

    #[test]
    fn test_parse_movie_web_dl() {
        let result = parse_movie("Dune (2021) 2160p WEB-DL x265 HEVC AAC");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Dune");
        assert_eq!(info.year, Some(2021));
        assert_eq!(info.resolution, Some("2160p".to_string()));
        assert_eq!(info.source, Some("WEB-DL".to_string()));
        // 同名组多次匹配时正则取最后一次：x265 先匹配，HEVC 后匹配覆盖
        assert_eq!(info.video_codec, Some("HEVC".to_string()));
        assert_eq!(info.audio_codec, Some("AAC".to_string()));
    }

    #[test]
    fn test_parse_movie_dotted() {
        let result = parse_movie("The.Matrix.1999.1080p.BluRay.x264");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "The.Matrix");
        assert_eq!(info.year, Some(1999));
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.source, Some("BluRay".to_string()));
    }

    #[test]
    fn test_parse_movie_h264() {
        let result = parse_movie("Avatar (2009) [1080p] [BluRay] [H.264] [DTS]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Avatar");
        assert_eq!(info.year, Some(2009));
        assert_eq!(info.video_codec, Some("H.264".to_string()));
        assert_eq!(info.audio_codec, Some("DTS".to_string()));
    }
}
