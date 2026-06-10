// src-tauri/src/scrape/local_metadata/image_asset_downloader.rs
// 职责：下载 TMDb 图片资源为二进制字节

use crate::scrape::local_metadata::nfo_document_builder::tmdb_image_url;

pub async fn download_image_asset(path: Option<&str>) -> Result<Option<Vec<u8>>, String> {
    let Some(path) = path else {
        return Ok(None);
    };

    let url = tmdb_image_url(path);
    let response = reqwest::get(&url)
        .await
        .map_err(|error| format!("download image request failed: {}", error))?;

    if !response.status().is_success() {
        return Err(format!("download image failed with status: {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("read image body failed: {}", error))?;

    Ok(Some(bytes.to_vec()))
}
