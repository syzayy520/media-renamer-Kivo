// src-tauri/src/commands/scrape_commands.rs
// 职责：把 scrape/local_metadata 子家族暴露为 Tauri 命令，不承载业务逻辑

use crate::scrape::local_metadata::{scrape_to_local, LocalScrapeInput, LocalScrapeOutput};

#[tauri::command]
pub async fn scrape_to_local_metadata(input: LocalScrapeInput) -> Result<LocalScrapeOutput, String> {
    Ok(scrape_to_local(input).await)
}
