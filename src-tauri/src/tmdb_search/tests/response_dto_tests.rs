// src-tauri/src/tmdb_search/tests/response_dto_tests.rs
// Response DTO Tests
// 职责：测试 TMDb 搜索响应 DTO 的序列化/反序列化

use crate::tmdb_search::response::tmdb_search_response::TmdbSearchResponse;
use crate::tmdb_search::response::tmdb_search_result::TmdbSearchResult;

#[test]
fn test_movie_result_deserialize() {
    let json = r#"{
        "id": 123,
        "title": "Inception",
        "original_title": "Inception",
        "release_date": "2010-07-16",
        "overview": "A thief who steals corporate secrets...",
        "poster_path": "/path.jpg",
        "backdrop_path": "/backdrop.jpg",
        "original_language": "en",
        "popularity": 50.5,
        "vote_average": 8.5
    }"#;

    let result: TmdbSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.id, 123);
    assert_eq!(result.title, Some("Inception".to_string()));
    assert_eq!(result.release_date, Some("2010-07-16".to_string()));
    assert_eq!(result.vote_average, Some(8.5));
}

#[test]
fn test_tv_result_deserialize() {
    let json = r#"{
        "id": 456,
        "name": "Breaking Bad",
        "original_name": "Breaking Bad",
        "first_air_date": "2008-01-20",
        "overview": "A high school chemistry teacher...",
        "poster_path": "/path.jpg",
        "backdrop_path": "/backdrop.jpg",
        "original_language": "en",
        "popularity": 75.2,
        "vote_average": 9.0
    }"#;

    let result: TmdbSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.id, 456);
    assert_eq!(result.name, Some("Breaking Bad".to_string()));
    assert_eq!(result.first_air_date, Some("2008-01-20".to_string()));
    assert_eq!(result.vote_average, Some(9.0));
}

#[test]
fn test_empty_results_deserialize() {
    let json = r#"{
        "page": 1,
        "total_pages": 0,
        "total_results": 0,
        "results": []
    }"#;

    let response: TmdbSearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.page, 1);
    assert_eq!(response.total_pages, 0);
    assert_eq!(response.total_results, 0);
    assert!(response.results.is_empty());
}

#[test]
fn test_missing_optional_fields_ok() {
    let json = r#"{
        "id": 789,
        "title": "Test Movie"
    }"#;

    let result: TmdbSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.id, 789);
    assert_eq!(result.title, Some("Test Movie".to_string()));
    assert_eq!(result.name, None);
    assert_eq!(result.release_date, None);
    assert_eq!(result.overview, None);
    assert_eq!(result.poster_path, None);
    assert_eq!(result.backdrop_path, None);
    assert_eq!(result.original_language, None);
    assert_eq!(result.popularity, None);
    assert_eq!(result.vote_average, None);
}

#[test]
fn test_no_key_token_fields_in_serialized_dto() {
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

    let json = serde_json::to_string(&result).unwrap();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("apiKey"));
    assert!(!json.contains("token"));
    assert!(!json.contains("account"));
    assert!(!json.contains("session"));
    assert!(!json.contains("auth"));
    assert!(!json.contains("authorization"));
}

#[test]
fn test_no_raw_passthrough_field() {
    let json = r#"{
        "id": 1,
        "title": "Test",
        "raw_response": {"some": "data"}
    }"#;

    // 应该能够反序列化，但 raw_response 字段会被忽略
    let result: TmdbSearchResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.id, 1);
    assert_eq!(result.title, Some("Test".to_string()));
}

#[test]
fn test_no_network_dependency() {
    // 验证 DTO 不依赖网络、API key、环境变量、文件系统
    let response = TmdbSearchResponse {
        page: 1,
        total_pages: 1,
        total_results: 1,
        results: vec![TmdbSearchResult {
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
        }],
    };

    // 直接序列化/反序列化，不使用任何 mock 或外部依赖
    let json = serde_json::to_string(&response).unwrap();
    let _: TmdbSearchResponse = serde_json::from_str(&json).unwrap();
}
