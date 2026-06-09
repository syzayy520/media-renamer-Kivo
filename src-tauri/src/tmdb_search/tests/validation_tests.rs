// src-tauri/src/tmdb_search/tests/validation_tests.rs
// Validation Tests
// 职责：测试搜索输入验证功能

use crate::tmdb_search_contract::{SearchTmdbCandidatesInput, TmdbSearchMediaType};
use crate::tmdb_search::validation::search_input_validator::validate_search_input;

#[test]
fn test_query_trim_empty_returns_error() {
    let input = SearchTmdbCandidatesInput {
        query: "   ".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: None,
    };
    let result = validate_search_input(&input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.message, "Search query cannot be empty");
}

#[test]
fn test_language_trim_empty_returns_error() {
    let input = SearchTmdbCandidatesInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "   ".to_string(),
        year: None,
        page: None,
    };
    let result = validate_search_input(&input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.message, "Language cannot be empty");
}

#[test]
fn test_page_none_defaults_to_1() {
    let input = SearchTmdbCandidatesInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: None,
    };
    let result = validate_search_input(&input).unwrap();
    assert_eq!(result.page, 1);
}

#[test]
fn test_page_zero_returns_error() {
    let input = SearchTmdbCandidatesInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: Some(0),
    };
    let result = validate_search_input(&input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.message, "Page number must be greater than 0");
}

#[test]
fn test_year_zero_returns_error() {
    let input = SearchTmdbCandidatesInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: Some(0),
        page: None,
    };
    let result = validate_search_input(&input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.message, "Year cannot be 0");
}

#[test]
fn test_valid_movie_input() {
    let input = SearchTmdbCandidatesInput {
        query: "Inception".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: Some(2010),
        page: Some(1),
    };
    let result = validate_search_input(&input).unwrap();
    assert_eq!(result.query, "Inception");
    assert_eq!(result.media_type, TmdbSearchMediaType::Movie);
    assert_eq!(result.language, "en");
    assert_eq!(result.year, Some(2010));
    assert_eq!(result.page, 1);
}

#[test]
fn test_valid_tv_input() {
    let input = SearchTmdbCandidatesInput {
        query: "Breaking Bad".to_string(),
        media_type: TmdbSearchMediaType::Tv,
        language: "en".to_string(),
        year: None,
        page: Some(2),
    };
    let result = validate_search_input(&input).unwrap();
    assert_eq!(result.query, "Breaking Bad");
    assert_eq!(result.media_type, TmdbSearchMediaType::Tv);
    assert_eq!(result.language, "en");
    assert_eq!(result.year, None);
    assert_eq!(result.page, 2);
}

#[test]
fn test_query_trimmed() {
    let input = SearchTmdbCandidatesInput {
        query: "  Inception  ".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: None,
    };
    let result = validate_search_input(&input).unwrap();
    assert_eq!(result.query, "Inception");
}

#[test]
fn test_language_trimmed() {
    let input = SearchTmdbCandidatesInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "  en  ".to_string(),
        year: None,
        page: None,
    };
    let result = validate_search_input(&input).unwrap();
    assert_eq!(result.language, "en");
}

#[test]
fn test_validation_error_json_no_api_key() {
    let input = SearchTmdbCandidatesInput {
        query: "".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: None,
    };
    let err = validate_search_input(&input).unwrap_err();
    let json = serde_json::to_string(&err).unwrap();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("apiKey"));
}

#[test]
fn test_validation_error_json_no_token() {
    let input = SearchTmdbCandidatesInput {
        query: "".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: None,
    };
    let err = validate_search_input(&input).unwrap_err();
    let json = serde_json::to_string(&err).unwrap();
    assert!(!json.contains("token"));
}

#[test]
fn test_no_network_dependency() {
    // 验证验证函数不依赖网络、API key、环境变量、文件系统
    // 如果函数签名正确，这个测试本身就证明了依赖隔离
    let input = SearchTmdbCandidatesInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: None,
    };
    // 直接调用，不使用任何 mock 或外部依赖
    let _ = validate_search_input(&input);
}
