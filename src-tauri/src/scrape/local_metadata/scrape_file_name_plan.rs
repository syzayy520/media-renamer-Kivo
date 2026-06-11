use std::path::Path;

use crate::scrape::local_metadata::local_scrape_input::LocalScrapeInput;
use crate::scrape::local_metadata::local_scrape_mode::LocalScrapeMode;
use crate::tmdb_search_contract::TmdbSearchMediaType;

pub struct ScrapeFileNamePlan {
    pub nfo_file_name: String,
    pub poster_file_name: String,
    pub fanart_file_name: String,
}

pub fn build_file_name_plan(input: &LocalScrapeInput, target_folder: &Path) -> ScrapeFileNamePlan {
    if should_use_folder_layout(input, target_folder) {
        return folder_layout(input);
    }

    sidecar_layout(&input.target_file_name)
}

fn should_use_folder_layout(input: &LocalScrapeInput, target_folder: &Path) -> bool {
    if matches!(input.mode, Some(LocalScrapeMode::IntoPreparedFolder)) {
        return true;
    }

    let media_stem = file_stem(&input.target_file_name);
    let folder_name = target_folder
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();

    normalize_name(folder_name) == normalize_name(&media_stem)
}

fn folder_layout(input: &LocalScrapeInput) -> ScrapeFileNamePlan {
    let nfo_file_name = match input.candidate.media_type {
        TmdbSearchMediaType::Tv => "tvshow.nfo".to_string(),
        TmdbSearchMediaType::Movie => "movie.nfo".to_string(),
    };

    ScrapeFileNamePlan {
        nfo_file_name,
        poster_file_name: "poster.jpg".to_string(),
        fanart_file_name: "fanart.jpg".to_string(),
    }
}

fn sidecar_layout(target_file_name: &str) -> ScrapeFileNamePlan {
    let stem = file_stem(target_file_name);

    ScrapeFileNamePlan {
        nfo_file_name: format!("{}.nfo", stem),
        poster_file_name: format!("{}-poster.jpg", stem),
        fanart_file_name: format!("{}-fanart.jpg", stem),
    }
}

fn file_stem(file_name: &str) -> String {
    let clean = file_name.trim();
    let slash = clean.rfind(|ch| ch == '\\' || ch == '/').map(|value| value + 1).unwrap_or(0);
    let name = &clean[slash..];
    let dot = name.rfind('.').unwrap_or(name.len());
    let stem = &name[..dot];

    if stem.trim().is_empty() {
        return "metadata".to_string();
    }

    stem.trim().to_string()
}

fn normalize_name(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !matches!(ch, '·' | ' ' | '.' | '_' | '-' | '(' | ')' | '[' | ']'))
        .collect::<String>()
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidecar_uses_media_stem() {
        let plan = sidecar_layout("第一滴血 (1982).mkv");
        assert_eq!(plan.nfo_file_name, "第一滴血 (1982).nfo");
        assert_eq!(plan.poster_file_name, "第一滴血 (1982)-poster.jpg");
    }
}
