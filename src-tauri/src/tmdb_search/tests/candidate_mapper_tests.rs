// src-tauri/src/tmdb_search/tests/candidate_mapper_tests.rs
// Candidate Mapper Tests
// 职责：测试候选映射器功能

use crate::tmdb_search::mapping::candidate_mapper::{map_to_candidate, map_to_candidates};
use crate::tmdb_search::response::tmdb_search_result::TmdbSearchResult;
use crate::tmdb_search_contract::TmdbSearchMediaType;

fn make_movie_result() -> TmdbSearchResult {
    TmdbSearchResult {
        id: 123,
        title: Some("Inception".to_string()),
        name: None,
        original_title: Some("Inception".to_string()),
        original_name: None,
        release_date: Some("2010-07-16".to_string()),
        first_air_date: None,
        overview: Some("A thief...".to_string()),
        poster_path: Some("/path.jpg".to_string()),
        backdrop_path: Some("/backdrop.jpg".to_string()),
        original_language: Some("en".to_string()),
        popularity: Some(50.5),
        vote_average: Some(8.5),
    }
}

fn make_tv_result() -> TmdbSearchResult {
    TmdbSearchResult {
        id: 456,
        title: None,
        name: Some("Breaking Bad".to_string()),
        original_title: None,
        original_name: Some("Breaking Bad".to_string()),
        release_date: None,
        first_air_date: Some("2008-01-20".to_string()),
        overview: Some("A chemistry teacher...".to_string()),
        poster_path: Some("/path.jpg".to_string()),
        backdrop_path: Some("/backdrop.jpg".to_string()),
        original_language: Some("en".to_string()),
        popularity: Some(75.2),
        vote_average: Some(9.0),
    }
}

#[test]
fn test_movie_candidate_mapping() {
    let result = make_movie_result();
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.id, "tmdb-movie-123");
    assert_eq!(candidate.tmdb_id, 123);
    assert_eq!(candidate.title, "Inception");
    assert_eq!(candidate.original_title, Some("Inception".to_string()));
    assert_eq!(candidate.media_type, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.release_year, Some(2010));
}

#[test]
fn test_tv_candidate_mapping() {
    let result = make_tv_result();
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Tv);
    assert_eq!(candidate.id, "tmdb-tv-456");
    assert_eq!(candidate.tmdb_id, 456);
    assert_eq!(candidate.title, "Breaking Bad");
    assert_eq!(candidate.original_title, Some("Breaking Bad".to_string()));
    assert_eq!(candidate.media_type, TmdbSearchMediaType::Tv);
    assert_eq!(candidate.release_year, Some(2008));
}

#[test]
fn test_movie_fallback_to_name() {
    let mut result = make_movie_result();
    result.title = None;
    result.name = Some("Fallback Title".to_string());
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.title, "Fallback Title");
}

#[test]
fn test_tv_fallback_to_title() {
    let mut result = make_tv_result();
    result.name = None;
    result.title = Some("Fallback Title".to_string());
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Tv);
    assert_eq!(candidate.title, "Fallback Title");
}

#[test]
fn test_empty_title_fallback() {
    let result = TmdbSearchResult {
        id: 1,
        title: None,
        name: None,
        original_title: None,
        original_name: None,
        release_date: None,
        first_air_date: None,
        overview: None,
        poster_path: None,
        backdrop_path: None,
        original_language: None,
        popularity: None,
        vote_average: None,
    };
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.title, "");
}

#[test]
fn test_release_year_none_when_no_date() {
    let mut result = make_movie_result();
    result.release_date = None;
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.release_year, None);
}

#[test]
fn test_release_year_none_when_invalid_date() {
    let mut result = make_movie_result();
    result.release_date = Some("invalid".to_string());
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.release_year, None);
}

#[test]
fn test_map_to_candidates_multiple() {
    let results = vec![make_movie_result(), make_tv_result()];
    let candidates = map_to_candidates(&results, TmdbSearchMediaType::Movie);
    assert_eq!(candidates.len(), 2);
}

#[test]
fn test_map_to_candidates_empty() {
    let results: Vec<TmdbSearchResult> = vec![];
    let candidates = map_to_candidates(&results, TmdbSearchMediaType::Movie);
    assert!(candidates.is_empty());
}

#[test]
fn test_no_api_key_in_candidate() {
    let result = make_movie_result();
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    let json = serde_json::to_string(&candidate).unwrap();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("apiKey"));
    assert!(!json.contains("token"));
    assert!(!json.contains("account"));
    assert!(!json.contains("session"));
}

#[test]
fn test_no_network_dependency() {
    let result = make_movie_result();
    let _ = map_to_candidate(&result, TmdbSearchMediaType::Movie);
}

#[test]
fn test_match_reasons_default() {
    let result = make_movie_result();
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.match_reasons, vec!["tmdb_search"]);
}

#[test]
fn test_confidence_hint_none() {
    let result = make_movie_result();
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.confidence_hint, None);
}

#[test]
fn test_optional_fields_preserved() {
    let result = make_movie_result();
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.overview, Some("A thief...".to_string()));
    assert_eq!(candidate.poster_path, Some("/path.jpg".to_string()));
    assert_eq!(candidate.backdrop_path, Some("/backdrop.jpg".to_string()));
    assert_eq!(candidate.language, Some("en".to_string()));
    assert_eq!(candidate.popularity, Some(50.5));
    assert_eq!(candidate.vote_average, Some(8.5));
}

#[test]
fn test_optional_fields_none() {
    let result = TmdbSearchResult {
        id: 1,
        title: Some("Test".to_string()),
        name: None,
        original_title: None,
        original_name: None,
        release_date: None,
        first_air_date: None,
        overview: None,
        poster_path: None,
        backdrop_path: None,
        original_language: None,
        popularity: None,
        vote_average: None,
    };
    let candidate = map_to_candidate(&result, TmdbSearchMediaType::Movie);
    assert_eq!(candidate.overview, None);
    assert_eq!(candidate.poster_path, None);
    assert_eq!(candidate.backdrop_path, None);
    assert_eq!(candidate.language, None);
    assert_eq!(candidate.popularity, None);
    assert_eq!(candidate.vote_average, None);
}
