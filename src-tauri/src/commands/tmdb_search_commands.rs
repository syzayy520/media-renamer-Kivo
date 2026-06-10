// src-tauri/src/commands/tmdb_search_commands.rs
// TMDb Search Tauri Commands
// 职责：将 tmdb_search command shell 注册为 Tauri 命令
// 当前状态：disabled gate — 始终返回 disabled 结果，不调用 live network

use crate::tmdb_search::command::search_tmdb_candidates_command_shell;
use crate::tmdb_search_contract::{SearchTmdbCandidatesInput, SearchTmdbCandidatesOutput};

/// TMDb 搜索候选 command
///
/// 当前状态：disabled gate — 始终返回结构化 disabled 错误。
/// 不读取 API key、不联网、不调用 transport。
///
/// # Arguments
/// * `input` - 搜索输入（复用 P2-011 contract）
///
/// # Returns
/// `SearchTmdbCandidatesOutput` 包含空候选列表和 disabled 错误
#[tauri::command]
pub async fn search_tmdb_candidates(
    input: SearchTmdbCandidatesInput,
) -> SearchTmdbCandidatesOutput {
    search_tmdb_candidates_command_shell(input)
}
