// 剧集解析器模块
// 职责：解析剧集文件名

use super::movie_parser::{MediaType, ParsedMediaInfo};
use crate::scan::MediaItem;
use regex::Regex;

/// 解析剧集文件名
pub fn parse_series(filename: &str) -> Option<ParsedMediaInfo> {
    // 剧集格式：Title - S01E01 - Episode Title [1080p] [WEB-DL]
    // 支持格式：点号/空格/下划线分隔，可选 episode title 和 token
    let re = Regex::new(
        r"(?x)
        ^(?P<title>.+?)                              # 标题（非贪婪）
        [\s\.\-_]+                                    # 分隔符
        S(?P<season>\d{1,2})                          # 季号
        E(?P<episode>\d{1,3})                         # 集号
        (?:-E?(?P<episode_end>\d{1,3}))?              # 多集结束号（可选）
        (?:                                           # episode title 组（可选）
            [\s\.\-_]+                                # 分隔符
            (?P<episode_title>                        # episode title
                [^\.\[\]]+                            # 匹配到点号或方括号停止（至少1字符）
            )
        )?
        (?:[\s\.\[\]]+                                # token 分隔符
        (?:                                           # token 组
            (?P<resolution>\d{3,4}p|4K|2160p)         # 分辨率
            |(?P<source>BluRay|WEB-DL|WEBRip|HDTV|DVD|Remux)  # 来源
            |(?P<video_codec>x264|x265|H\.?264|H\.?265|HEVC|AV1)  # 视频编码
            |(?P<audio_codec>AAC|DTS|FLAC|AC3|Atmos|TrueHD)  # 音频编码
        )
        \]?                                           # 可选右方括号
        )*                                            # 可重复多个 token
        $                                             # 字符串结尾
        ",
    )
    .ok()?;

    let caps = re.captures(filename)?;

    let title = caps.name("title")?.as_str().trim().to_string();
    let season = caps.name("season")?.as_str().parse::<u16>().ok()?;
    let episode = caps.name("episode")?.as_str().parse::<u16>().ok()?;
    let episode_end = caps
        .name("episode_end")
        .and_then(|m| m.as_str().parse::<u16>().ok());
    let episode_title = caps
        .name("episode_title")
        .map(|m| m.as_str().trim().to_string());
    let resolution = caps.name("resolution").map(|m| m.as_str().to_string());
    let source = caps.name("source").map(|m| m.as_str().to_string());
    let video_codec = caps.name("video_codec").map(|m| m.as_str().to_string());
    let audio_codec = caps.name("audio_codec").map(|m| m.as_str().to_string());

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
        media_type: MediaType::Series,
        title,
        year: None,
        season: Some(season),
        episode: Some(episode),
        episode_end,
        episode_title,
        resolution,
        source,
        video_codec,
        audio_codec,
        group: None,
        confidence: 90, // 剧集格式明确，置信度较高
        rule_sources: vec!["series_parser".to_string()],
        special_type: None,
        extra_type: None,
        extra_number: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_series_basic() {
        let result = parse_series("Breaking Bad - S01E01 - Pilot [1080p] [BluRay] [x264]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Breaking Bad");
        assert_eq!(info.season, Some(1));
        assert_eq!(info.episode, Some(1));
        assert_eq!(info.episode_title, Some("Pilot".to_string()));
    }

    #[test]
    fn test_parse_series_multi_episode() {
        let result = parse_series("Breaking Bad - S01E01-E02 - Pilot");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.episode_end, Some(2));
    }

    #[test]
    fn test_parse_series_no_title() {
        let result = parse_series("S01E01");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_series_dot_separator() {
        let result = parse_series("Show.Name.S01E01.Pilot.1080p.WEB-DL");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Show.Name");
        assert_eq!(info.season, Some(1));
        assert_eq!(info.episode, Some(1));
        assert_eq!(info.episode_title, Some("Pilot".to_string()));
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.source, Some("WEB-DL".to_string()));
    }

    #[test]
    fn test_parse_series_underscore_separator() {
        let result = parse_series("Show_Name_S01E01_Pilot");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Show_Name");
        assert_eq!(info.season, Some(1));
        assert_eq!(info.episode, Some(1));
        assert_eq!(info.episode_title, Some("Pilot".to_string()));
    }

    #[test]
    fn test_parse_series_space_separator() {
        let result = parse_series("Show Name S01E01 Pilot [1080p] [WEB-DL]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Show Name");
        assert_eq!(info.season, Some(1));
        assert_eq!(info.episode, Some(1));
        assert_eq!(info.episode_title, Some("Pilot".to_string()));
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.source, Some("WEB-DL".to_string()));
    }

    #[test]
    fn test_parse_series_no_episode_title() {
        let result = parse_series("Show.Name.S01E01.[1080p].[WEB-DL]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.title, "Show.Name");
        assert_eq!(info.season, Some(1));
        assert_eq!(info.episode, Some(1));
        assert_eq!(info.episode_title, None);
        assert_eq!(info.resolution, Some("1080p".to_string()));
        assert_eq!(info.source, Some("WEB-DL".to_string()));
    }
}
