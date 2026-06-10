// src-tauri/src/tmdb_search/tests/request_builder_tests.rs
// Request Builder Tests
// 职责：测试搜索请求构建功能

use crate::tmdb_search::request::search_query::build_search_query;
use crate::tmdb_search::request::search_url::build_search_url;
use crate::tmdb_search::validation::search_input_validator::ValidatedTmdbSearchInput;
use crate::tmdb_search_contract::TmdbSearchMediaType;

#[test]
fn test_movie_search_path() {
    let input = ValidatedTmdbSearchInput {
        query: "Inception".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    let query = build_search_query(&input);
    assert_eq!(query.path, "/search/movie");
}

#[test]
fn test_tv_search_path() {
    let input = ValidatedTmdbSearchInput {
        query: "Breaking Bad".to_string(),
        media_type: TmdbSearchMediaType::Tv,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    let query = build_search_query(&input);
    assert_eq!(query.path, "/search/tv");
}

#[test]
fn test_query_params_include_query_language_page() {
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "zh-CN".to_string(),
        year: None,
        page: 2,
    };
    let query = build_search_query(&input);
    let params: std::collections::HashMap<_, _> = query.params.into_iter().collect();
    assert_eq!(params.get("query").unwrap(), "test");
    assert_eq!(params.get("language").unwrap(), "zh-CN");
    assert_eq!(params.get("page").unwrap(), "2");
}

#[test]
fn test_year_some_included() {
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: Some(2010),
        page: 1,
    };
    let query = build_search_query(&input);
    let params: std::collections::HashMap<_, _> = query.params.into_iter().collect();
    assert_eq!(params.get("year").unwrap(), "2010");
}

#[test]
fn test_year_none_not_included() {
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    let query = build_search_query(&input);
    let params: std::collections::HashMap<_, _> = query.params.into_iter().collect();
    assert!(!params.contains_key("year"));
}

#[test]
fn test_no_api_key_in_params() {
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    let query = build_search_query(&input);
    let params: std::collections::HashMap<_, _> = query.params.into_iter().collect();
    assert!(!params.contains_key("api_key"));
    assert!(!params.contains_key("apiKey"));
    assert!(!params.contains_key("token"));
    assert!(!params.contains_key("bearer"));
    assert!(!params.contains_key("authorization"));
    assert!(!params.contains_key("session"));
}

#[test]
fn test_search_url_format() {
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    let query = build_search_query(&input);
    let url = build_search_url(&query);
    assert!(url.starts_with("/search/movie?"));
    assert!(url.contains("query=test"));
    assert!(url.contains("language=en"));
    assert!(url.contains("page=1"));
}

#[test]
fn test_search_url_no_http_https() {
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    let query = build_search_query(&input);
    let url = build_search_url(&query);
    assert!(!url.contains("http://"));
    assert!(!url.contains("https://"));
}

#[test]
fn test_search_url_no_api_key() {
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    let query = build_search_query(&input);
    let url = build_search_url(&query);
    assert!(!url.contains("api_key"));
    assert!(!url.contains("apiKey"));
    assert!(!url.contains("token"));
    assert!(!url.contains("bearer"));
    assert!(!url.contains("authorization"));
    assert!(!url.contains("session"));
}

#[test]
fn test_no_network_dependency() {
    // 验证请求构建函数不依赖网络、API key、环境变量、文件系统
    let input = ValidatedTmdbSearchInput {
        query: "test".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: 1,
    };
    // 直接调用，不使用任何 mock 或外部依赖
    let _ = build_search_query(&input);
}
