// src-tauri/src/tmdb_search/tests/error_mapper_tests.rs
// Error Mapper Tests
// 职责：测试错误映射器功能

use crate::tmdb_search::mapping::error_mapper::{
    api_key_missing_error, map_http_error, network_connection_error, timeout_error,
    HttpErrorResponse,
};
use crate::tmdb_search_contract::TmdbSearchErrorCode;

#[test]
fn test_401_maps_to_api_key_invalid() {
    let response = HttpErrorResponse {
        status_code: 401,
        message: "Unauthorized".to_string(),
        retry_after: None,
    };
    let error = map_http_error(&response);
    assert_eq!(error.code, TmdbSearchErrorCode::ApiKeyInvalid);
    assert!(!error.retryable);
}

#[test]
fn test_429_maps_to_rate_limited() {
    let response = HttpErrorResponse {
        status_code: 429,
        message: "Too Many Requests".to_string(),
        retry_after: Some(30),
    };
    let error = map_http_error(&response);
    assert_eq!(error.code, TmdbSearchErrorCode::RateLimited);
    assert!(error.retryable);
    assert_eq!(error.retry_after, Some(30));
}

#[test]
fn test_500_maps_to_network_error() {
    let response = HttpErrorResponse {
        status_code: 500,
        message: "Internal Server Error".to_string(),
        retry_after: None,
    };
    let error = map_http_error(&response);
    assert_eq!(error.code, TmdbSearchErrorCode::NetworkError);
    assert!(error.retryable);
}

#[test]
fn test_502_maps_to_network_error() {
    let response = HttpErrorResponse {
        status_code: 502,
        message: "Bad Gateway".to_string(),
        retry_after: None,
    };
    let error = map_http_error(&response);
    assert_eq!(error.code, TmdbSearchErrorCode::NetworkError);
    assert!(error.retryable);
}

#[test]
fn test_503_maps_to_network_error() {
    let response = HttpErrorResponse {
        status_code: 503,
        message: "Service Unavailable".to_string(),
        retry_after: Some(60),
    };
    let error = map_http_error(&response);
    assert_eq!(error.code, TmdbSearchErrorCode::NetworkError);
    assert!(error.retryable);
    assert_eq!(error.retry_after, Some(60));
}

#[test]
fn test_404_maps_to_unknown() {
    let response = HttpErrorResponse {
        status_code: 404,
        message: "Not Found".to_string(),
        retry_after: None,
    };
    let error = map_http_error(&response);
    assert_eq!(error.code, TmdbSearchErrorCode::Unknown);
    assert!(!error.retryable);
}

#[test]
fn test_unknown_status_code() {
    let response = HttpErrorResponse {
        status_code: 418,
        message: "I'm a teapot".to_string(),
        retry_after: None,
    };
    let error = map_http_error(&response);
    assert_eq!(error.code, TmdbSearchErrorCode::Unknown);
    assert!(!error.retryable);
}

#[test]
fn test_api_key_missing_helper() {
    let error = api_key_missing_error();
    assert_eq!(error.code, TmdbSearchErrorCode::ApiKeyMissing);
    assert_eq!(error.message, "API key is missing");
    assert!(!error.retryable);
    assert_eq!(error.retry_after, None);
}

#[test]
fn test_timeout_helper() {
    let error = timeout_error();
    assert_eq!(error.code, TmdbSearchErrorCode::Timeout);
    assert_eq!(error.message, "Request timed out");
    assert!(error.retryable);
}

#[test]
fn test_network_connection_error_helper() {
    let error = network_connection_error("Connection refused");
    assert_eq!(error.code, TmdbSearchErrorCode::NetworkError);
    assert_eq!(error.message, "Connection refused");
    assert!(error.retryable);
}

#[test]
fn test_error_message_preserved() {
    let response = HttpErrorResponse {
        status_code: 429,
        message: "Rate limit exceeded for API key".to_string(),
        retry_after: Some(10),
    };
    let error = map_http_error(&response);
    assert_eq!(error.message, "Rate limit exceeded for API key");
}

#[test]
fn test_no_api_key_in_error_json() {
    let response = HttpErrorResponse {
        status_code: 401,
        message: "Unauthorized".to_string(),
        retry_after: None,
    };
    let error = map_http_error(&response);
    let json = serde_json::to_string(&error).unwrap();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("apiKey"));
}

#[test]
fn test_no_token_in_error_json() {
    let response = HttpErrorResponse {
        status_code: 401,
        message: "Unauthorized".to_string(),
        retry_after: None,
    };
    let error = map_http_error(&response);
    let json = serde_json::to_string(&error).unwrap();
    assert!(!json.contains("token"));
}

#[test]
fn test_no_network_dependency() {
    let response = HttpErrorResponse {
        status_code: 500,
        message: "Error".to_string(),
        retry_after: None,
    };
    let _ = map_http_error(&response);
}

#[test]
fn test_retryable_codes() {
    // 429 and 5xx should be retryable
    for code in [429, 500, 501, 502, 503, 504] {
        let response = HttpErrorResponse {
            status_code: code,
            message: "Error".to_string(),
            retry_after: None,
        };
        let error = map_http_error(&response);
        assert!(error.retryable, "Status code {} should be retryable", code);
    }
}

#[test]
fn test_non_retryable_codes() {
    // 400, 401, 403, 404 should not be retryable
    for code in [400, 401, 403, 404] {
        let response = HttpErrorResponse {
            status_code: code,
            message: "Error".to_string(),
            retry_after: None,
        };
        let error = map_http_error(&response);
        assert!(
            !error.retryable,
            "Status code {} should not be retryable",
            code
        );
    }
}
