use crate::rename::candidate_apply::{apply_movie_candidate, apply_tv_candidate, PreviewApplyResult};
use crate::rename::folder_policy::policy::FolderPolicy;
use crate::rename::naming_context::build_token_context;
use crate::rename::naming_rule::{render_naming_rule, NamingRule, TitleStrategy};
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
pub fn apply_folder_policy(input: ApplyFolderPolicyInput) -> Result<ApplyFolderPolicyOutput, String> {
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
            FolderPolicy::KeepOriginalStructure | FolderPolicy::NoFolder => format!("{}\\{}", current_dir, item.proposed_name),
            FolderPolicy::OneMovieOneFolder => format!("{}\\{}\\{}", current_dir, folder_from_name, item.proposed_name),
            FolderPolicy::NormalizeExistingFolders | FolderPolicy::ChineseFolderPtFile => {
                let folder = extract_last_dir(&current_dir);
                if folder.is_empty() || folder == folder_from_name {
                    format!("{}\\{}", current_dir, item.proposed_name)
                } else {
                    format!("{}\\{}\\{}", current_parent, folder_from_name, item.proposed_name)
                }
            }
            FolderPolicy::Flatten => format!("{}\\{}", current_parent, item.proposed_name),
            FolderPolicy::TvShowStructure => format!("{}\\{}", current_dir, item.proposed_name),
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
pub fn apply_tmdb_candidate(input: ApplyTmdbCandidateInput) -> Result<ApplyTmdbCandidateOutput, String> {
    let item = &input.item;
    let candidate = &input.candidate;

    let result = match candidate.media_type {
        TmdbSearchMediaType::Movie => apply_movie_candidate(item, &candidate.title, candidate.release_year, candidate.tmdb_id),
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
                blocking_reasons: if has_blocking { vec!["single item safety check failed".to_string()] } else { vec![] },
            };
            Ok(ApplyTmdbCandidateOutput { result: Some(apply_result), error: None, safety: Some(safety) })
        }
        Err(e) => Ok(ApplyTmdbCandidateOutput { result: None, error: Some(e.to_string()), safety: None }),
    }
}

#[tauri::command]
pub fn get_safety_summary(input: SafetySummaryInput) -> Result<SafetyReport, String> {
    Ok(safety_checker::check_all(&input.previews))
}
