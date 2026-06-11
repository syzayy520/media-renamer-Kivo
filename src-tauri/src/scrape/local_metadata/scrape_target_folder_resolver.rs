use std::path::PathBuf;

use crate::scrape::local_metadata::local_scrape_input::LocalScrapeInput;
use crate::scrape::local_metadata::local_scrape_mode::LocalScrapeMode;

pub fn resolve_target_folder(input: &LocalScrapeInput) -> Result<PathBuf, String> {
    match input.mode.unwrap_or_default() {
        LocalScrapeMode::BesideCurrentMedia => resolve_beside_current_media(input),
        LocalScrapeMode::IntoPreparedFolder => resolve_prepared_folder(input),
    }
}

fn resolve_beside_current_media(input: &LocalScrapeInput) -> Result<PathBuf, String> {
    if let Some(media_path) = input.media_file_path.as_deref() {
        if let Some(parent) = parent_path(media_path) {
            if is_absolute_like(parent) {
                return Ok(PathBuf::from(parent));
            }
        }
    }

    if let Some(scan_root) = input.scan_root.as_deref() {
        if is_absolute_like(scan_root) {
            return Ok(PathBuf::from(scan_root));
        }
    }

    if is_absolute_like(&input.target_folder_path) {
        return Ok(PathBuf::from(&input.target_folder_path));
    }

    Err("cannot resolve an absolute scrape target folder".to_string())
}

fn resolve_prepared_folder(input: &LocalScrapeInput) -> Result<PathBuf, String> {
    if is_absolute_like(&input.target_folder_path) {
        return Ok(PathBuf::from(&input.target_folder_path));
    }

    if let Some(scan_root) = input.scan_root.as_deref() {
        if is_absolute_like(scan_root) {
            return Ok(join_path(scan_root, &input.target_folder_path));
        }
    }

    Err("cannot resolve prepared scrape target folder".to_string())
}

fn parent_path(path: &str) -> Option<&str> {
    let index = path.rfind(|ch| ch == '\\' || ch == '/')?;
    if index == 0 {
        return None;
    }
    Some(&path[..index])
}

fn is_absolute_like(path: &str) -> bool {
    let bytes = path.as_bytes();
    path.starts_with("\\\\")
        || path.starts_with("//")
        || path.starts_with('/')
        || (bytes.len() >= 3
            && bytes[1] == b':'
            && (bytes[2] == b'\\' || bytes[2] == b'/')
            && bytes[0].is_ascii_alphabetic())
}

fn join_path(parent: &str, child: &str) -> PathBuf {
    let separator = if parent.contains('\\') { "\\" } else { "/" };
    let left = parent.trim_end_matches(|ch| ch == '\\' || ch == '/');
    let right = child.trim_start_matches(|ch| ch == '\\' || ch == '/');
    PathBuf::from(format!("{}{}{}", left, separator, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_path_preserves_unc_prefix() {
        assert_eq!(
            parent_path("\\\\nas\\video\\movie.mkv"),
            Some("\\\\nas\\video")
        );
    }

    #[test]
    fn detects_windows_absolute_paths() {
        assert!(is_absolute_like("C:\\Media\\movie.mkv"));
        assert!(is_absolute_like("\\\\nas\\video"));
    }
}
