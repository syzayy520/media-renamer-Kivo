use crate::scrape::local_metadata::image_asset_downloader::download_image_asset_to_file;
use crate::scrape::local_metadata::local_metadata_writer::{ensure_target_folder, write_nfo_file};
use crate::scrape::local_metadata::local_scrape_input::LocalScrapeInput;
use crate::scrape::local_metadata::local_scrape_output::LocalScrapeOutput;
use crate::scrape::local_metadata::nfo_document_builder::build_nfo_document;
use crate::scrape::local_metadata::scrape_file_name_plan::build_file_name_plan;
use crate::scrape::local_metadata::scrape_target_folder_resolver::resolve_target_folder;

pub async fn scrape_to_local(input: LocalScrapeInput) -> LocalScrapeOutput {
    let mut written_files = Vec::new();
    let mut errors = Vec::new();

    let folder_path = match resolve_target_folder(&input) {
        Ok(path) => path,
        Err(error) => {
            return LocalScrapeOutput {
                success: false,
                target_folder_path: input.target_folder_path,
                written_files,
                errors: vec![error],
            };
        }
    };

    let folder = match ensure_target_folder(&folder_path.to_string_lossy()) {
        Ok(value) => value,
        Err(error) => {
            return LocalScrapeOutput {
                success: false,
                target_folder_path: folder_path.to_string_lossy().to_string(),
                written_files,
                errors: vec![error],
            };
        }
    };

    let file_plan = build_file_name_plan(&input, &folder);
    let nfo_document = build_nfo_document(&input.candidate);

    match write_nfo_file(&folder, &file_plan.nfo_file_name, &nfo_document) {
        Ok(file) => written_files.push(file),
        Err(error) => errors.push(error),
    }

    if input.include_images.unwrap_or(true) {
        match download_image_asset_to_file(
            input.candidate.poster_path.as_deref(),
            &folder,
            &file_plan.poster_file_name,
        )
        .await
        {
            Ok(Some(file)) => written_files.push(file),
            Ok(None) => {}
            Err(error) => errors.push(error),
        }

        match download_image_asset_to_file(
            input.candidate.backdrop_path.as_deref(),
            &folder,
            &file_plan.fanart_file_name,
        )
        .await
        {
            Ok(Some(file)) => written_files.push(file),
            Ok(None) => {}
            Err(error) => errors.push(error),
        }
    }

    LocalScrapeOutput {
        success: !written_files.is_empty(),
        target_folder_path: folder.to_string_lossy().to_string(),
        written_files,
        errors,
    }
}
