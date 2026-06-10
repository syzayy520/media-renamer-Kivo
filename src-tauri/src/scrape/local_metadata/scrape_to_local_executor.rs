// src-tauri/src/scrape/local_metadata/scrape_to_local_executor.rs
// 职责：编排本地刮削写入流程，不承载 NFO 构建或文件写入细节

use crate::scrape::local_metadata::image_asset_downloader::download_image_asset_safely;
use crate::scrape::local_metadata::local_metadata_writer::{
    ensure_target_folder, write_binary_asset, write_nfo,
};
use crate::scrape::local_metadata::local_scrape_input::LocalScrapeInput;
use crate::scrape::local_metadata::local_scrape_output::LocalScrapeOutput;
use crate::scrape::local_metadata::nfo_document_builder::build_nfo_document;
use crate::tmdb_search_contract::TmdbSearchMediaType;

pub async fn scrape_to_local(input: LocalScrapeInput) -> LocalScrapeOutput {
    let mut written_files = Vec::new();
    let mut errors = Vec::new();

    let target_folder = match ensure_target_folder(&input.target_folder_path) {
        Ok(folder) => folder,
        Err(error) => {
            return LocalScrapeOutput {
                success: false,
                target_folder_path: input.target_folder_path,
                written_files,
                errors: vec![error],
            };
        }
    };

    let media_kind = match input.candidate.media_type {
        TmdbSearchMediaType::Tv => "tv",
        TmdbSearchMediaType::Movie => "movie",
    };
    let nfo_document = build_nfo_document(&input.candidate);

    match write_nfo(&target_folder, media_kind, &nfo_document) {
        Ok(file) => written_files.push(file),
        Err(error) => errors.push(error),
    }

    match download_image_asset_safely(input.candidate.poster_path.as_deref()).await {
        Ok(Some(bytes)) => match write_binary_asset(&target_folder, "poster.jpg", &bytes) {
            Ok(file) => written_files.push(file),
            Err(error) => errors.push(error),
        },
        Ok(None) => {}
        Err(error) => errors.push(error),
    }

    match download_image_asset_safely(input.candidate.backdrop_path.as_deref()).await {
        Ok(Some(bytes)) => match write_binary_asset(&target_folder, "fanart.jpg", &bytes) {
            Ok(file) => written_files.push(file),
            Err(error) => errors.push(error),
        },
        Ok(None) => {}
        Err(error) => errors.push(error),
    }

    LocalScrapeOutput {
        success: !written_files.is_empty(),
        target_folder_path: input.target_folder_path,
        written_files,
        errors,
    }
}
