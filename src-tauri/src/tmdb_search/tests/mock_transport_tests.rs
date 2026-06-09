// src-tauri/src/tmdb_search/tests/mock_transport_tests.rs
// Mock Transport Tests
// 职责：测试 Mock 传输层功能

use crate::tmdb_search::transport::MockTmdbTransport;
use crate::tmdb_search::TmdbTransport;
use crate::tmdb_search_contract::TmdbSearchErrorCode;

#[tokio::test]
async fn test_mock_response_basic() {
    let mock = MockTmdbTransport::new();
    let response_json = r#"{"results": []}"#.to_string();
    mock.mock_response(
        "/search/movie",
        &[
            ("query".to_string(), "test".to_string()),
            ("language".to_string(), "en".to_string()),
        ],
        Ok(response_json.clone()),
    );

    let params = vec![
        ("query".to_string(), "test".to_string()),
        ("language".to_string(), "en".to_string()),
    ];
    let result = mock.send_search_request("/search/movie", &params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), response_json);
}

#[tokio::test]
async fn test_mock_response_simple() {
    let mock = MockTmdbTransport::new();
    let response_json = r#"{"results": []}"#.to_string();
    mock.mock_response_simple("/search/movie", Ok(response_json.clone()));

    let params = vec![("query".to_string(), "test".to_string())];
    let result = mock.send_search_request("/search/movie", &params).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), response_json);
}

#[tokio::test]
async fn test_mock_response_error() {
    let mock = MockTmdbTransport::new();
    mock.mock_response_simple(
        "/search/movie",
        Err(crate::tmdb_search_contract::TmdbSearchError {
            code: TmdbSearchErrorCode::RateLimited,
            message: "Rate limited".to_string(),
            retryable: true,
            retry_after: Some(30),
        }),
    );

    let params = vec![];
    let result = mock.send_search_request("/search/movie", &params).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, TmdbSearchErrorCode::RateLimited);
    assert!(err.retryable);
    assert_eq!(err.retry_after, Some(30));
}

#[tokio::test]
async fn test_no_mock_response_returns_error() {
    let mock = MockTmdbTransport::new();
    let params = vec![];
    let result = mock.send_search_request("/unknown", &params).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, TmdbSearchErrorCode::Unknown);
    assert!(err.message.contains("No mock response"));
}

#[tokio::test]
async fn test_call_history_recorded() {
    let mock = MockTmdbTransport::new();
    mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

    let params1 = vec![("query".to_string(), "test1".to_string())];
    let params2 = vec![("query".to_string(), "test2".to_string())];

    let _ = mock.send_search_request("/search/movie", &params1).await;
    let _ = mock.send_search_request("/search/movie", &params2).await;

    let history = mock.call_history();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "/search/movie?query=test1");
    assert_eq!(history[1], "/search/movie?query=test2");
}

#[tokio::test]
async fn test_call_count() {
    let mock = MockTmdbTransport::new();
    mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

    assert_eq!(mock.call_count(), 0);

    let params = vec![];
    let _ = mock.send_search_request("/search/movie", &params).await;
    assert_eq!(mock.call_count(), 1);

    let _ = mock.send_search_request("/search/movie", &params).await;
    assert_eq!(mock.call_count(), 2);
}

#[tokio::test]
async fn test_clear_history() {
    let mock = MockTmdbTransport::new();
    mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

    let params = vec![];
    let _ = mock.send_search_request("/search/movie", &params).await;
    assert_eq!(mock.call_count(), 1);

    mock.clear_history();
    assert_eq!(mock.call_count(), 0);
}

#[tokio::test]
async fn test_default_trait() {
    let mock = MockTmdbTransport::default();
    assert_eq!(mock.call_count(), 0);
}

#[tokio::test]
async fn test_no_api_key_in_mock() {
    let mock = MockTmdbTransport::new();
    mock.mock_response_simple("/search/movie", Ok(r#"{"results": []}"#.to_string()));

    let params = vec![("query".to_string(), "test".to_string())];
    let _ = mock.send_search_request("/search/movie", &params).await;

    // Verify no api_key in call history
    let history = mock.call_history();
    for call in &history {
        assert!(!call.contains("api_key"));
        assert!(!call.contains("apiKey"));
    }
}

#[tokio::test]
async fn test_no_network_dependency() {
    let mock = MockTmdbTransport::new();
    mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

    let params = vec![];
    let _ = mock.send_search_request("/search/movie", &params).await;
}

#[tokio::test]
async fn test_multiple_paths() {
    let mock = MockTmdbTransport::new();
    mock.mock_response_simple("/search/movie", Ok(r#"{"type": "movie"}"#.to_string()));
    mock.mock_response_simple("/search/tv", Ok(r#"{"type": "tv"}"#.to_string()));

    let params = vec![];

    let movie_result = mock.send_search_request("/search/movie", &params).await.unwrap();
    assert_eq!(movie_result, r#"{"type": "movie"}"#);

    let tv_result = mock.send_search_request("/search/tv", &params).await.unwrap();
    assert_eq!(tv_result, r#"{"type": "tv"}"#);
}

#[tokio::test]
async fn test_multiple_mock_instances() {
    // Test that multiple mock instances work independently
    let mock1 = MockTmdbTransport::new();
    let mock2 = MockTmdbTransport::new();

    mock1.mock_response_simple("/search/movie", Ok("response1".to_string()));
    mock2.mock_response_simple("/search/tv", Ok("response2".to_string()));

    let params = vec![];

    let result1 = mock1.send_search_request("/search/movie", &params).await.unwrap();
    assert_eq!(result1, "response1");

    let result2 = mock2.send_search_request("/search/tv", &params).await.unwrap();
    assert_eq!(result2, "response2");

    // mock1 doesn't have /search/tv
    let result3 = mock1.send_search_request("/search/tv", &params).await;
    assert!(result3.is_err());

    // mock2 doesn't have /search/movie
    let result4 = mock2.send_search_request("/search/movie", &params).await;
    assert!(result4.is_err());
}
