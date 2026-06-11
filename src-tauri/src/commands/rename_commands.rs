use crate::media_info::probe::filename_media_info_parser::parse_filename_media_info;
use crate::rename::candidate_apply::{
    apply_movie_candidate, apply_tv_candidate, PreviewApplyResult,
};
use crate::rename::folder_policy::policy::FolderPolicy;
use crate::rename::naming_rule::{render_naming_rule, NamingRule, TitleStrategy, TokenContext};
use crate::rename::safety_checker::{self, SafetyReport};
use crate::rename::template::RenamePreviewItem;
use crate::tmdb_search_contract::{TmdbCandidate, TmdbSearchMediaType};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyTmdbCandidateInput {
    pub item: RenamePreviewItem,
    pub candidate: TmdbCandidate,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplyTmdbCandidateOutput {
    pub result: Option<PreviewApplyResult>,
    pub error: Option<String>,
    pub safety: Option<SafetyReport>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SafetySummaryInput {
    pub previews: Vec<RenamePreviewItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyNamingRuleInput {
    pub previews: Vec<RenamePreviewItem>,
    pub naming_rule: NamingRule,
    pub title_strategy: TitleStrategy,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplyNamingRuleOutput {
    pub previews: Vec<RenamePreviewItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyFolderPolicyInput {
    pub previews: Vec<RenamePreviewItem>,
    pub folder_policy: FolderPolicy,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplyFolderPolicyOutput {
    pub previews: Vec<RenamePreviewItem>,
}

fn build_token_context(item: &RenamePreviewItem) -> TokenContext {
    let info = &item.parsed_info;
    let title = &info.title;
    let media_info = parse_filename_media_info(&info.media_item.file_name);

    TokenContext {
        zh_title: if title.is_empty() || title == "Unknown" {
            None
        } else {
            Some(title.clone())
        },
        english_title: None,
        original_title: None,
        original_name_without_ext: Some(info.media_item.file_name.clone()),
        original_release_name: Some(info.media_item.file_name.clone()),
        year: info.year,
        release_date: None,
        air_date: None,
        season_year: None,
        resolution: info.resolution.clone().or(media_info.resolution.clone()),
        source: info.source.clone().or(media_info.source.clone()),
        edition: None,
        remux: None,
        video_codec: info.video_codec.clone().or(media_info.video.codec.clone()),
        video_bit_depth: media_info.video.bit_depth.clone(),
        hdr_format: media_info.video.hdr_format.clone(),
        dolby_vision: media_info.video.dolby_vision,
        audio_codec: info.audio_codec.clone().or(media_info.audio.codec.clone()),
        audio_channels: media_info.audio.channels.clone(),
        audio_language: media_info.audio.language.clone(),
        release_group: info.group.clone().or(media_info.release_group.clone()),
        tmdb_id: None,
        imdb_id: None,
        tvdb_id: None,
        show_title: None,
        season: info.season.map(|s| s as u32),
        episode: info.episode.map(|e| vec![e as u32]),
        episode_title: info.episode_title.clone(),
        absolute_episode: None,
        season_title: None,
        ext: Some(info.media_item.extension.clone()),
        subtitle_language: None,
        file_role: None,
    }
}

fn parent_dir(path: &str) -> String {
    Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

fn stem_from_name(name: &str) -> String {
    match name.rfind('.') {
        Some(pos) => name[..pos].to_string(),
        None => name.to_string(),
    }
}

#[tauri::command]
pub fn apply_naming_rule(input: ApplyNamingRuleInput) -> Result<ApplyNamingRuleOutput, String> {
    let mut updated = Vec::with_capacity(input.previews.len());

    for item in &input.previews {
        if item.should_skip {
            updated.push(item.clone());
            continue;
        }

        let ctx = build_token_context(item);
        let result = render_naming_rule(&input.naming_rule, &ctx, input.title_strategy);

        let new_name = result.name;
        let dir = parent_dir(&item.source_path);
        let new_target = format!("{}\\{}", dir, new_name);

        let mut new_item = item.clone();
        new_item.proposed_name = new_name;
        new_item.target_path = new_target;
        new_item.needs_manual_review = result.needs_review;
        updated.push(new_item);
    }

    Ok(ApplyNamingRuleOutput { previews: updated })
}

#[tauri::command]
pub fn apply_folder_policy(
    input: ApplyFolderPolicyInput,
) -> Result<ApplyFolderPolicyOutput, String> {
    let mut updated = Vec::with_capacity(input.previews.len());
    let policy = input.folder_policy;

    for item in &input.previews {
        if item.should_skip {
            updated.push(item.clone());
            continue;
        }

        let mut new_item = item.clone();
        let current_dir = parent_dir(&item.source_path);
        let current_parent = parent_dir(&current_dir);
        let folder_from_name = stem_from_name(&item.proposed_name);

        let new_target = match policy {
            FolderPolicy::KeepOriginalStructure | FolderPolicy::NoFolder => {
                format!("{}\\{}", current_dir, item.proposed_name)
            }
            FolderPolicy::OneMovieOneFolder => {
                format!(
                    "{}\\{}\\{}",
                    current_dir, folder_from_name, item.proposed_name
                )
            }
            FolderPolicy::NormalizeExistingFolders | FolderPolicy::ChineseFolderPtFile => {
                let folder = extract_last_dir(&current_dir);
                if folder.is_empty() || folder == folder_from_name {
                    format!("{}\\{}", current_dir, item.proposed_name)
                } else {
                    format!(
                        "{}\\{}\\{}",
                        current_parent, folder_from_name, item.proposed_name
                    )
                }
            }
            FolderPolicy::Flatten => {
                format!("{}\\{}", current_parent, item.proposed_name)
            }
            FolderPolicy::TvShowStructure => {
                format!("{}\\{}", current_dir, item.proposed_name)
            }
        };

        new_item.target_path = new_target;
        updated.push(new_item);
    }

    Ok(ApplyFolderPolicyOutput { previews: updated })
}

fn extract_last_dir(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    match normalized.rfind('/') {
        Some(pos) => normalized[pos + 1..].to_string(),
        None => path.to_string(),
    }
}

#[tauri::command]
pub fn apply_tmdb_candidate(
    input: ApplyTmdbCandidateInput,
) -> Result<ApplyTmdbCandidateOutput, String> {
    let item = &input.item;
    let candidate = &input.candidate;

    let result = match candidate.media_type {
        TmdbSearchMediaType::Movie => apply_movie_candidate(
            item,
            &candidate.title,
            candidate.release_year,
            candidate.tmdb_id,
        ),
        TmdbSearchMediaType::Tv => apply_tv_candidate(
            item,
            &candidate.title,
            candidate.release_year,
            item.parsed_info.season,
            item.parsed_info.episode,
            item.parsed_info.episode_title.as_deref(),
            candidate.tmdb_id,
        ),
    };

    match result {
        Ok(apply_result) => {
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
        assert!(!report.can_execute);
    }
}
