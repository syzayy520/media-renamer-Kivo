// 置信度评分模块
// 职责：计算解析结果的置信度
// 每种媒体类型有独立的评分依据，低于阈值必须进入人工确认

use super::movie_parser::{MediaType, ParsedMediaInfo};

/// 置信度阈值常量
pub const DEFAULT_CONFIDENCE_THRESHOLD: u8 = 70;

/// 规则匹配证据
#[derive(Debug, Clone)]
pub struct RuleMatchEvidence {
    /// 规则名称
    pub rule_name: String,
    /// 匹配到的模式描述
    pub matched_pattern: String,
    /// 置信度增量（正数加分，负数扣分）
    pub confidence_delta: i16,
}

/// 置信度评分结果
#[derive(Debug, Clone)]
pub struct ConfidenceResult {
    /// 最终置信度
    pub score: u8,
    /// 匹配证据列表
    pub evidences: Vec<RuleMatchEvidence>,
    /// 是否需要人工确认
    pub needs_review: bool,
}

/// 根据媒体类型计算置信度并生成证据
pub fn evaluate_confidence(info: &mut ParsedMediaInfo) -> ConfidenceResult {
    let mut evidences = Vec::new();

    match info.media_type {
        MediaType::Movie => evaluate_movie(info, &mut evidences),
        MediaType::Series => evaluate_series(info, &mut evidences),
        MediaType::Anime => evaluate_anime(info, &mut evidences),
        MediaType::Special | MediaType::Ova | MediaType::Ncop | MediaType::Nced => {
            evaluate_special(info, &mut evidences)
        }
        MediaType::Extras => evaluate_extras(info, &mut evidences),
        MediaType::Unknown => evaluate_unknown(info, &mut evidences),
    }

    // 通用规则：标题质量
    if info.title.is_empty() {
        evidences.push(RuleMatchEvidence {
            rule_name: "title_empty".to_string(),
            matched_pattern: "empty title".to_string(),
            confidence_delta: -30,
        });
    } else if info.title.len() < 3 {
        evidences.push(RuleMatchEvidence {
            rule_name: "title_short".to_string(),
            matched_pattern: format!("title length={}", info.title.len()),
            confidence_delta: -20,
        });
    }

    // 计算最终分数
    let mut score = info.confidence as i16;
    for ev in &evidences {
        score += ev.confidence_delta;
    }
    score = score.clamp(0, 100);
    info.confidence = score as u8;

    ConfidenceResult {
        score: info.confidence,
        evidences,
        needs_review: info.confidence < DEFAULT_CONFIDENCE_THRESHOLD,
    }
}

/// 电影评分依据
fn evaluate_movie(info: &ParsedMediaInfo, evidences: &mut Vec<RuleMatchEvidence>) {
    // 有年份 → 高可信度
    if info.year.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "movie_year".to_string(),
            matched_pattern: format!("year={:?}", info.year),
            confidence_delta: 10,
        });
    } else {
        evidences.push(RuleMatchEvidence {
            rule_name: "movie_no_year".to_string(),
            matched_pattern: "no year found".to_string(),
            confidence_delta: -15,
        });
    }

    // 有分辨率/来源/编码 → 加分
    if info.resolution.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "movie_resolution".to_string(),
            matched_pattern: format!("resolution={:?}", info.resolution),
            confidence_delta: 5,
        });
    }
    if info.source.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "movie_source".to_string(),
            matched_pattern: format!("source={:?}", info.source),
            confidence_delta: 3,
        });
    }
}

/// 剧集评分依据
fn evaluate_series(info: &ParsedMediaInfo, evidences: &mut Vec<RuleMatchEvidence>) {
    // 有季号+集号 → 高可信度
    if info.season.is_some() && info.episode.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "series_season_episode".to_string(),
            matched_pattern: format!(
                "S{:02}E{:02}",
                info.season.unwrap_or(0),
                info.episode.unwrap_or(0)
            ),
            confidence_delta: 10,
        });
    }

    // 有 episode_title → 加分
    if info.episode_title.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "series_episode_title".to_string(),
            matched_pattern: format!("episode_title={:?}", info.episode_title),
            confidence_delta: 5,
        });
    }

    // 有分辨率 → 加分
    if info.resolution.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "series_resolution".to_string(),
            matched_pattern: format!("resolution={:?}", info.resolution),
            confidence_delta: 3,
        });
    }
}

/// 动漫评分依据
fn evaluate_anime(info: &ParsedMediaInfo, evidences: &mut Vec<RuleMatchEvidence>) {
    // 有 Group → 高可信度
    if info.group.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "anime_group".to_string(),
            matched_pattern: format!("group={:?}", info.group),
            confidence_delta: 10,
        });
    } else {
        evidences.push(RuleMatchEvidence {
            rule_name: "anime_no_group".to_string(),
            matched_pattern: "no group tag".to_string(),
            confidence_delta: -10,
        });
    }

    // 有集号 → 基础可信度
    if info.episode.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "anime_episode".to_string(),
            matched_pattern: format!("episode={:?}", info.episode),
            confidence_delta: 5,
        });
    }

    // 有分辨率 → 加分
    if info.resolution.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "anime_resolution".to_string(),
            matched_pattern: format!("resolution={:?}", info.resolution),
            confidence_delta: 5,
        });
    }
}

/// 特别篇评分依据
fn evaluate_special(info: &ParsedMediaInfo, evidences: &mut Vec<RuleMatchEvidence>) {
    // S00E01 格式 → 高可信度
    if info.season == Some(0) {
        evidences.push(RuleMatchEvidence {
            rule_name: "special_s00".to_string(),
            matched_pattern: "S00Exx format".to_string(),
            confidence_delta: 10,
        });
    }

    // 有 special_type → 加分
    if info.special_type.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "special_type".to_string(),
            matched_pattern: format!("special_type={:?}", info.special_type),
            confidence_delta: 5,
        });
    }
}

/// Extras 评分依据
fn evaluate_extras(info: &ParsedMediaInfo, evidences: &mut Vec<RuleMatchEvidence>) {
    if info.extra_type.is_some() && info.extra_number.is_some() {
        evidences.push(RuleMatchEvidence {
            rule_name: "extras_identified".to_string(),
            matched_pattern: format!(
                "extra_type={:?}, extra_number={:?}",
                info.extra_type, info.extra_number
            ),
            confidence_delta: 10,
        });
    }
}

/// 未识别评分依据
fn evaluate_unknown(_info: &ParsedMediaInfo, evidences: &mut Vec<RuleMatchEvidence>) {
    evidences.push(RuleMatchEvidence {
        rule_name: "unknown_type".to_string(),
        matched_pattern: "no pattern matched".to_string(),
        confidence_delta: -20,
    });
}

/// 调整置信度（简化版，兼容旧接口）
pub fn adjust_confidence(info: &mut ParsedMediaInfo) {
    evaluate_confidence(info);
}

/// 是否需要人工确认
pub fn needs_review(info: &ParsedMediaInfo, threshold: u8) -> bool {
    info.confidence < threshold
}

#[cfg(test)]
mod tests {
    use super::super::movie_parser::MediaType;
    use super::*;
    use crate::scan::MediaItem;

    fn make_test_info(media_type: MediaType, confidence: u8) -> ParsedMediaInfo {
        ParsedMediaInfo {
            media_item: MediaItem {
                id: String::new(),
                file_path: String::new(),
                file_name: String::new(),
                extension: String::new(),
                file_size: 0,
                is_video: true,
                is_companion: false,
            },
            media_type,
            title: "Test".to_string(),
            year: Some(2020),
            season: None,
            episode: None,
            episode_end: None,
            episode_title: None,
            resolution: Some("1080p".to_string()),
            source: None,
            video_codec: None,
            audio_codec: None,
            group: None,
            confidence,
            rule_sources: vec![],
            special_type: None,
            extra_type: None,
            extra_number: None,
        }
    }

    #[test]
    fn test_evaluate_movie_with_year() {
        let mut info = make_test_info(MediaType::Movie, 85);
        let result = evaluate_confidence(&mut info);
        // 有年份(+10) + 有分辨率(+5) + 标题正常(0) = 100
        assert!(result.score >= 90);
        assert!(result.evidences.iter().any(|e| e.rule_name == "movie_year"));
        assert!(!result.needs_review);
    }

    #[test]
    fn test_evaluate_movie_no_year() {
        let mut info = make_test_info(MediaType::Movie, 85);
        info.year = None;
        let result = evaluate_confidence(&mut info);
        // 无年份(-15) + 有分辨率(+5) = 75
        assert!(result
            .evidences
            .iter()
            .any(|e| e.rule_name == "movie_no_year"));
        assert!(result.score < 85);
    }

    #[test]
    fn test_evaluate_anime_with_group() {
        let mut info = make_test_info(MediaType::Anime, 85);
        info.group = Some("SubGroup".to_string());
        info.year = None;
        let result = evaluate_confidence(&mut info);
        assert!(result
            .evidences
            .iter()
            .any(|e| e.rule_name == "anime_group"));
        assert!(result.score >= 90);
    }

    #[test]
    fn test_evaluate_anime_no_group_low_confidence() {
        let mut info = make_test_info(MediaType::Anime, 70);
        info.group = None;
        info.year = None;
        let result = evaluate_confidence(&mut info);
        // 无 group(-10) + 无年份不影响(非Movie) + 有分辨率(+5) = 65
        assert!(result.needs_review);
    }

    #[test]
    fn test_evaluate_series_with_season_episode() {
        let mut info = make_test_info(MediaType::Series, 85);
        info.season = Some(1);
        info.episode = Some(1);
        info.episode_title = Some("Pilot".to_string());
        info.year = None;
        let result = evaluate_confidence(&mut info);
        assert!(result.score >= 90);
        assert!(result
            .evidences
            .iter()
            .any(|e| e.rule_name == "series_season_episode"));
    }

    #[test]
    fn test_evaluate_special_s00() {
        let mut info = make_test_info(MediaType::Special, 85);
        info.season = Some(0);
        info.special_type = Some(super::super::movie_parser::SpecialType::Special);
        info.year = None;
        let result = evaluate_confidence(&mut info);
        assert!(result.score >= 90);
        assert!(result
            .evidences
            .iter()
            .any(|e| e.rule_name == "special_s00"));
    }

    #[test]
    fn test_evaluate_unknown_low_confidence() {
        let mut info = make_test_info(MediaType::Unknown, 50);
        let result = evaluate_confidence(&mut info);
        assert!(result.needs_review);
        assert!(result
            .evidences
            .iter()
            .any(|e| e.rule_name == "unknown_type"));
    }

    #[test]
    fn test_evidence_contains_rule_name_and_delta() {
        let mut info = make_test_info(MediaType::Movie, 85);
        let result = evaluate_confidence(&mut info);
        for ev in &result.evidences {
            assert!(!ev.rule_name.is_empty());
            assert!(!ev.matched_pattern.is_empty());
            // delta 不为 0
            assert!(ev.confidence_delta != 0 || ev.rule_name == "movie_year");
        }
    }
}
