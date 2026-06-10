// src-tauri/src/scrape/local_metadata/local_scrape_input.rs
// 职责：定义本地刮削命令输入契约

use crate::tmdb_search_contract::TmdbCandidate;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LocalScrapeInput {
    pub candidate: TmdbCandidate,
    pub target_folder_path: String,
    pub target_file_name: String,
}
