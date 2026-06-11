// src-tauri/src/scrape/local_metadata/image_asset_downloader.rs
// 职责：把 TMDb 图片资源安全下载到指定文件，不承载刮削编排逻辑

use std::path::Path;
#[cfg(not(target_os = "windows"))]
use std::time::Duration;

use crate::scrape::local_metadata::local_scrape_output::LocalScrapeWrittenFile;
use crate::scrape::local_metadata::nfo_document_builder::tmdb_image_url;

#[cfg(not(target_os = "windows"))]
const IMAGE_DOWNLOAD_TIMEOUT_SECS: u64 = 20;
#[cfg(not(target_os = "windows"))]
const MAX_IMAGE_BYTES: u64 = 25 * 1024 * 1024;

pub async fn download_image_asset_to_file(
    path: Option<&str>,
    target_folder: &Path,
    file_name: &str,
) -> Result<Option<LocalScrapeWrittenFile>, String> {
    let Some(path) = path else {
        return Ok(None);
    };

    let url = tmdb_image_url(path);
    let file_path = target_folder.join(file_name);

    #[cfg(target_os = "windows")]
    {
        download_image_with_powershell(&url, &file_path).await?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let bytes = download_image_asset(path).await?;
        tokio::fs::write(&file_path, &bytes)
            .await
            .map_err(|error| format!("write {} failed: {}", file_name, error))?;
    }

    Ok(Some(LocalScrapeWrittenFile {
        role: file_name.to_string(),
        file_path: file_path.to_string_lossy().to_string(),
    }))
}

#[cfg(target_os = "windows")]
async fn download_image_with_powershell(url: &str, file_path: &Path) -> Result<(), String> {
    let output = tokio::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg("$ProgressPreference='SilentlyContinue'; Invoke-WebRequest -Uri $args[0] -OutFile $args[1] -UseBasicParsing")
        .arg(url)
        .arg(file_path.to_string_lossy().to_string())
        .output()
        .await
        .map_err(|error| format!("start image download process failed: {}", error))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if stderr.is_empty() {
        return Err(format!("image download process failed with status: {}", output.status));
    }

    Err(stderr)
}

#[cfg(not(target_os = "windows"))]
async fn download_image_asset(path: &str) -> Result<Vec<u8>, String> {
    let url = tmdb_image_url(path);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(IMAGE_DOWNLOAD_TIMEOUT_SECS))
        .redirect(reqwest::redirect::Policy::limited(5))
        .user_agent("media-renamer-kivo/0.1 local-metadata-scraper")
        .build()
        .map_err(|error| format!("create image http client failed: {}", error))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|error| format!("download image request failed: {}", error))?;

    if !response.status().is_success() {
        return Err(format!("download image failed with status: {}", response.status()));
    }

    if let Some(length) = response.content_length() {
        if length > MAX_IMAGE_BYTES {
            return Err(format!("image too large: {} bytes", length));
        }
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("read image body failed: {}", error))?;

    if bytes.len() as u64 > MAX_IMAGE_BYTES {
        return Err(format!("image too large after download: {} bytes", bytes.len()));
    }

    Ok(bytes.to_vec())
}
