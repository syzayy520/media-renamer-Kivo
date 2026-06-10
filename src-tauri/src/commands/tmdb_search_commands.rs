// src-tauri/src/commands/tmdb_search_commands.rs
// TMDb Search Tauri Commands
// 职责：将 tmdb_search command shell 注册为 Tauri 命令
// 当前状态：gated execution — 使用 LiveSearchService 执行真实搜索

use crate::tmdb_search::command::search_tmdb_candidates_command_shell;
use crate::tmdb_search::command::TmdbSearchState;
use crate::tmdb_search_contract::{SearchTmdbCandidatesInput, SearchTmdbCandidatesOutput};

/// TMDb 搜索候选 command
///
/// 使用 LiveSearchService 执行真实 TMDb 搜索。
/// 必须显式启用 transport gate 才能执行网络请求。
///
/// # Arguments
/// * `input` - 搜索输入（复用 P2-011 contract）
/// * `state` - Tauri 状态，包含 transport gate 和 API key provider
///
/// # Returns
/// `Result<SearchTmdbCandidatesOutput, String>` 包含搜索结果或错误
#[tauri::command]
pub async fn search_tmdb_candidates(
    input: SearchTmdbCandidatesInput,
    state: tauri::State<'_, TmdbSearchState>,
) -> Result<SearchTmdbCandidatesOutput, String> {
    // Check if gate is enabled
    if !state.is_gate_enabled() {
        return Ok(search_tmdb_candidates_command_shell(input));
    }

    // Create live search service
    let service = crate::tmdb_search::service::LiveSearchService::new(
        state.api_key_provider.clone(),
        state.gate.clone(),
    );

    // Execute search
    Ok(service.search(input).await)
}
