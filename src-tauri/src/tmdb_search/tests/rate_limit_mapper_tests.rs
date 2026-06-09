// src-tauri/src/tmdb_search/tests/rate_limit_mapper_tests.rs
// Rate Limit Mapper Tests
// 职责：测试速率限制映射器功能

use crate::tmdb_search::mapping::rate_limit_mapper::{
    is_rate_limited, map_rate_limit, seconds_until_reset, HttpRateLimitResponse,
};
use crate::tmdb_search_contract::TmdbRateLimitState;

#[test]
fn test_map_rate_limit_basic() {
    let response = HttpRateLimitResponse {
        remaining: 35,
        reset_at: 1234567890,
        limit: Some(40),
    };
    let state = map_rate_limit(&response);
    assert_eq!(state.remaining, 35);
    assert_eq!(state.reset_at, 1234567890);
    assert_eq!(state.limit, Some(40));
}

#[test]
fn test_map_rate_limit_no_limit() {
    let response = HttpRateLimitResponse {
        remaining: 10,
        reset_at: 1234567890,
        limit: None,
    };
    let state = map_rate_limit(&response);
    assert_eq!(state.limit, None);
}

#[test]
fn test_map_rate_limit_zero_remaining() {
    let response = HttpRateLimitResponse {
        remaining: 0,
        reset_at: 1234567890,
        limit: Some(40),
    };
    let state = map_rate_limit(&response);
    assert_eq!(state.remaining, 0);
}

#[test]
fn test_is_rate_limited_true() {
    let state = TmdbRateLimitState {
        remaining: 0,
        reset_at: 1234567890,
        limit: Some(40),
    };
    assert!(is_rate_limited(&state));
}

#[test]
fn test_is_rate_limited_false() {
    let state = TmdbRateLimitState {
        remaining: 1,
        reset_at: 1234567890,
        limit: Some(40),
    };
    assert!(!is_rate_limited(&state));
}

#[test]
fn test_is_rate_limited_false_many_remaining() {
    let state = TmdbRateLimitState {
        remaining: 35,
        reset_at: 1234567890,
        limit: Some(40),
    };
    assert!(!is_rate_limited(&state));
}

#[test]
fn test_seconds_until_reset_future() {
    let state = TmdbRateLimitState {
        remaining: 10,
        reset_at: 1000,
        limit: None,
    };
    assert_eq!(seconds_until_reset(&state, 900), 100);
}

#[test]
fn test_seconds_until_reset_past() {
    let state = TmdbRateLimitState {
        remaining: 10,
        reset_at: 1000,
        limit: None,
    };
    assert_eq!(seconds_until_reset(&state, 1100), 0);
}

#[test]
fn test_seconds_until_reset_exact() {
    let state = TmdbRateLimitState {
        remaining: 10,
        reset_at: 1000,
        limit: None,
    };
    assert_eq!(seconds_until_reset(&state, 1000), 0);
}

#[test]
fn test_seconds_until_reset_large_difference() {
    let state = TmdbRateLimitState {
        remaining: 10,
        reset_at: 2000000000,
        limit: None,
    };
    assert_eq!(seconds_until_reset(&state, 1000000000), 1000000000);
}

#[test]
fn test_no_api_key_in_rate_limit_json() {
    let response = HttpRateLimitResponse {
        remaining: 10,
        reset_at: 1234567890,
        limit: Some(40),
    };
    let state = map_rate_limit(&response);
    let json = serde_json::to_string(&state).unwrap();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("apiKey"));
    assert!(!json.contains("token"));
}

#[test]
fn test_no_network_dependency() {
    let response = HttpRateLimitResponse {
        remaining: 10,
        reset_at: 1234567890,
        limit: None,
    };
    let _ = map_rate_limit(&response);
}

#[test]
fn test_rate_limit_state_serialization() {
    let state = TmdbRateLimitState {
        remaining: 35,
        reset_at: 1234567890,
        limit: Some(40),
    };
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: TmdbRateLimitState = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.remaining, 35);
    assert_eq!(deserialized.reset_at, 1234567890);
    assert_eq!(deserialized.limit, Some(40));
}

#[test]
fn test_rate_limit_state_serialization_no_limit() {
    let state = TmdbRateLimitState {
        remaining: 10,
        reset_at: 1234567890,
        limit: None,
    };
    let json = serde_json::to_string(&state).unwrap();
    assert!(!json.contains("limit"));
}
