use std::fs;
use std::path::{Path, PathBuf};

use crate::scrape::local_metadata::local_scrape_output::LocalScrapeWrittenFile;

pub fn ensure_target_folder(target_folder_path: &str) -> Result<PathBuf, String> {
    let folder = PathBuf::from(target_folder_path);
    fs::create_dir_all(&folder)
        .map_err(|error| format!("create metadata folder failed: {}", error))?;
    Ok(folder)
}

pub fn write_nfo_file(
    target_folder: &Path,
    file_name: &str,
    nfo_document: &str,
) -> Result<LocalScrapeWrittenFile, String> {
    let file_path = target_folder.join(file_name);

    fs::write(&file_path, nfo_document)
        .map_err(|error| format!("write {} failed: {}", file_name, error))?;

    Ok(LocalScrapeWrittenFile {
        role: file_name.to_string(),
        file_path: file_path.to_string_lossy().to_string(),
    })
}

pub fn write_binary_asset(
    target_folder: &Path,
    file_name: &str,
    bytes: &[u8],
) -> Result<LocalScrapeWrittenFile, String> {
    let file_path = target_folder.join(file_name);

    fs::write(&file_path, bytes)
        .map_err(|error| format!("write {} failed: {}", file_name, error))?;

    Ok(LocalScrapeWrittenFile {
        role: file_name.to_string(),
        file_path: file_path.to_string_lossy().to_string(),
    })
}
