// rename_commands 模块 - 重命名相关 Tauri 命令
// 职责：将 rename::candidate_apply 暴露为 Tauri 命令
// 不做业务逻辑，只做参数转换 + 错误映射

use crate::rename::candidate_apply::{
    apply_movie_candidate, apply_tv_candidate, PreviewApplyResult,
};
use crate::rename::safety_checker::{self, SafetyReport};
use crate::rename::template::RenamePreviewItem;
use crate::tmdb_search_contract::{TmdbCandidate, TmdbSearchMediaType};
use serde::{Deserialize, Serialize};

/// TMDb 候选应用输入
#[derive(Debug, Clone, Deserialize)]
pub struct ApplyTmdbCandidateInput {
    /// 当前预览项
    pub item: RenamePreviewItem,
    /// TMDb 候选
    pub candidate: TmdbCandidate,
}

/// TMDb 候选应用输出
#[derive(Debug, Clone, Serialize)]
pub struct ApplyTmdbCandidateOutput {
    /// 应用结果（成功时有值）
    pub result: Option<PreviewApplyResult>,
    /// 错误信息（失败时有值）
    pub error: Option<String>,
    /// 应用后的安全检查报告
    pub safety: Option<SafetyReport>,
}

/// 安全摘要输入（用于重新检查更新后的预览列表）
#[derive(Debug, Clone, Deserialize)]
pub struct SafetySummaryInput {
    /// 更新后的预览项列表
    pub previews: Vec<RenamePreviewItem>,
}

/// 应用 TMDb 候选到预览项（Tauri 命令）
///
/// 根据候选的媒体类型调用对应的 apply 函数：
/// - Movie → apply_movie_candidate
/// - Tv → apply_tv_candidate（使用原解析的 season/episode 信息）
///
/// # Arguments
/// * `input` - 包含原预览项和 TMDb 候选
///
/// # Returns
/// `ApplyTmdbCandidateOutput` 包含更新后的预览项、消息和安全报告
#[tauri::command]
pub fn apply_tmdb_candidate(
    input: ApplyTmdbCandidateInput,
) -> Result<ApplyTmdbCandidateOutput, String> {
    let item = &input.item;
    let candidate = &input.candidate;

    // 映射 TmdbSearchMediaType → MediaType 来决定调用哪个 apply 函数
    let result = match candidate.media_type {
        TmdbSearchMediaType::Movie => apply_movie_candidate(
            item,
            &candidate.title,
            candidate.release_year,
            candidate.tmdb_id,
        ),
        TmdbSearchMediaType::Tv => {
            // 对于 TV 候选，TMDb 搜索不返回 season/episode
            // 使用原解析的 season/episode/episode_title
            apply_tv_candidate(
                item,
                &candidate.title,
                candidate.release_year,
                item.parsed_info.season,
                item.parsed_info.episode,
                item.parsed_info.episode_title.as_deref(),
                candidate.tmdb_id,
            )
        }
    };

    match result {
        Ok(apply_result) => {
            // 对更新后的单项做安全检查
            let safety_checks = safety_checker::check_single(&apply_result.updated_item);
            let has_blocking = safety_checks.iter().any(|c| !c.passed);

            let safety = SafetyReport {
                can_execute: !has_blocking,
                dry_run: true,
                checks: safety_checks,
                blocking_reasons: if has_blocking {
                    vec!["单项安全检查未通过".to_string()]
                } else {
                    vec![]
                },
            };

            Ok(ApplyTmdbCandidateOutput {
                result: Some(apply_result),
                error: None,
                safety: Some(safety),
            })
        }
        Err(e) => Ok(ApplyTmdbCandidateOutput {
            result: None,
            error: Some(e.to_string()),
            safety: None,
        }),
    }
}

/// 重新运行安全摘要检查（Tauri 命令）
///
/// 在批量应用候选后，重新检查整个预览列表的安全性。
///
/// # Arguments
/// * `input` - 包含更新后的预览项列表
///
/// # Returns
/// `SafetyReport` 包含完整安全检查结果
#[tauri::command]
pub fn get_safety_summary(input: SafetySummaryInput) -> Result<SafetyReport, String> {
    Ok(safety_checker::check_all(&input.previews))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::MediaType;
    use crate::rename::template::MetadataSource;
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
                media_type: MediaType::Movie,
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
            media_type: MediaType::Movie,
            confidence: 50,
            needs_manual_review: true,
            should_skip: true,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    fn make_movie_candidate(title: &str, year: Option<u16>, tmdb_id: u64) -> TmdbCandidate {
        TmdbCandidate {
            id: format!("candidate-{}", tmdb_id),
            tmdb_id,
            title: title.to_string(),
            original_title: None,
            media_type: TmdbSearchMediaType::Movie,
            release_year: year,
            overview: None,
            poster_path: None,
            backdrop_path: None,
            language: Some("en".to_string()),
            popularity: Some(100.0),
            vote_average: Some(8.5),
            confidence_hint: Some(0.95),
            match_reasons: vec!["title_match".to_string()],
        }
    }

    fn make_tv_candidate(title: &str, year: Option<u16>, tmdb_id: u64) -> TmdbCandidate {
        TmdbCandidate {
            id: format!("candidate-{}", tmdb_id),
            tmdb_id,
            title: title.to_string(),
            original_title: None,
            media_type: TmdbSearchMediaType::Tv,
            release_year: year,
            overview: None,
            poster_path: None,
            backdrop_path: None,
            language: Some("en".to_string()),
            popularity: Some(100.0),
            vote_average: Some(8.5),
            confidence_hint: Some(0.95),
            match_reasons: vec!["title_match".to_string()],
        }
    }

    #[test]
    fn test_apply_tmdb_movie_candidate_success() {
        let item = make_test_item();
        let candidate = make_movie_candidate("Inception", Some(2010u16), 550u64);

        let input = ApplyTmdbCandidateInput { item, candidate };
        let output = apply_tmdb_candidate(input).unwrap();

        assert!(output.result.is_some());
        assert!(output.error.is_none());
        assert!(output.safety.is_some());

        let result = output.result.unwrap();
        assert!(result.success);
        assert_eq!(result.updated_item.proposed_name, "Inception (2010).mkv");
        assert_eq!(result.updated_item.metadata_source, MetadataSource::Tmdb);
        assert_eq!(result.updated_item.confidence, 95);
    }

    #[test]
    fn test_apply_tmdb_tv_candidate_success() {
        let mut item = make_test_item();
        item.parsed_info.media_type = MediaType::Series;
        item.parsed_info.season = Some(1u16);
        item.parsed_info.episode = Some(5u16);

        let candidate = make_tv_candidate("Breaking Bad", Some(2008u16), 1396u64);

        let input = ApplyTmdbCandidateInput { item, candidate };
        let output = apply_tmdb_candidate(input).unwrap();

        assert!(output.result.is_some());
        let result = output.result.unwrap();
        assert!(result.success);
        assert_eq!(
            result.updated_item.proposed_name,
            "Breaking Bad - S01E05.mkv"
        );
    }

    #[test]
    fn test_apply_tmdb_candidate_empty_title() {
        let item = make_test_item();
        let candidate = make_movie_candidate("", Some(2010u16), 550u64);

        let input = ApplyTmdbCandidateInput { item, candidate };
        let output = apply_tmdb_candidate(input).unwrap();

        assert!(output.result.is_none());
        assert!(output.error.is_some());
    }

    #[test]
    fn test_apply_tmdb_candidate_safety_report() {
        let item = make_test_item();
        let candidate = make_movie_candidate("Inception", Some(2010u16), 550u64);

        let input = ApplyTmdbCandidateInput { item, candidate };
        let output = apply_tmdb_candidate(input).unwrap();

        let safety = output.safety.unwrap();
        // TMDb 候选应用后置信度为 95，应通过安全检查
        assert!(safety.can_execute);
        assert!(safety.dry_run);
    }

    #[test]
    fn test_get_safety_summary_empty() {
        let input = SafetySummaryInput { previews: vec![] };
        let report = get_safety_summary(input).unwrap();
        assert!(report.can_execute);
    }

    #[test]
    fn test_get_safety_summary_with_items() {
        let item = make_test_item();
        let input = SafetySummaryInput {
            previews: vec![item],
        };
        let report = get_safety_summary(input).unwrap();
        // 低置信度 (50) 应该阻塞
        assert!(!report.can_execute);
    }
}
