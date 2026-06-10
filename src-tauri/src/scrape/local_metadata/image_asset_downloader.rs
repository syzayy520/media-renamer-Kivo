// src-tauri/src/scrape/local_metadata/image_asset_downloader.rs
// 职责：下载 TMDb 图片资源为二进制字节，所有失败必须返回错误，不允许影响主进程稳定性

use std::time::Duration;

use crate::scrape::local_metadata::nfo_document_builder::tmdb_image_url;

const IMAGE_DOWNLOAD_TIMEOUT_SECS: u64 = 20;
const MAX_IMAGE_BYTES: u64 = 25 * 1024 * 1024;

pub async fn download_image_asset(path: Option<&str>) -> Result<Option<Vec<u8>>, String> {
    let Some(path) = path else {
        return Ok(None);
    };

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

    Ok(Some(bytes.to_vec()))
}
