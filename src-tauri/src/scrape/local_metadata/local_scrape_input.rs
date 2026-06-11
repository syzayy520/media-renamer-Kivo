use crate::scrape::local_metadata::local_scrape_mode::LocalScrapeMode;
use crate::tmdb_search_contract::TmdbCandidate;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LocalScrapeInput {
    pub candidate: TmdbCandidate,
    pub target_folder_path: String,
    pub target_file_name: String,
    pub media_file_path: Option<String>,
    pub scan_root: Option<String>,
    pub mode: Option<LocalScrapeMode>,
    pub include_images: Option<bool>,
}
