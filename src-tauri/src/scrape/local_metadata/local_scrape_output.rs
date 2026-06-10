// src-tauri/src/scrape/local_metadata/local_scrape_output.rs
// 职责：定义本地刮削命令输出契约

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LocalScrapeWrittenFile {
    pub role: String,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocalScrapeOutput {
    pub success: bool,
    pub target_folder_path: String,
    pub written_files: Vec<LocalScrapeWrittenFile>,
    pub errors: Vec<String>,
}
