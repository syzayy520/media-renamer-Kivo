// src-tauri/src/tmdb_search_contract/tests/error_contract_tests.rs
// TMDb Search Backend Contract 错误契约测试
// 职责：验证错误类型的契约行为

use crate::tmdb_search_contract::candidate::{TmdbCandidate, TmdbCandidateSource};
use crate::tmdb_search_contract::error::{TmdbSearchError, TmdbSearchErrorCode};
use crate::tmdb_search_contract::output::SearchTmdbCandidatesOutput;
use serde_json;

#[cfg(test)]
mod tests {
    use super::*;

    // 测试 1: 空 candidates 不等于 error
    #[test]
    fn test_empty_candidates_not_error() {
        let output = SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: None,
        };

        // 空结果不应该有 error
        assert!(output.error.is_none());
        assert!(output.candidates.is_empty());
    }

    // 测试 2: 有 candidates 时 error 可以为 Some
    #[test]
    fn test_candidates_with_error() {
        let output = SearchTmdbCandidatesOutput {
            candidates: vec![TmdbCandidate {
                id: "candidate-1".to_string(),
                tmdb_id: 550,
                title: "Fight Club".to_string(),
                original_title: Some("Fight Club".to_string()),
                media_type: crate::tmdb_search_contract::media_type::TmdbSearchMediaType::Movie,
                release_year: Some(1999),
                overview: None,
                poster_path: None,
                backdrop_path: None,
                language: None,
                popularity: None,
                vote_average: None,
                confidence_hint: None,
                match_reasons: vec![],
            }],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: Some(TmdbSearchError {
                code: TmdbSearchErrorCode::RateLimited,
                message: "Rate limited but returned partial results".to_string(),
                retryable: true,
                retry_after: Some(60),
            }),
        };

        assert!(!output.candidates.is_empty());
        assert!(output.error.is_some());
    }

    // 测试 3: API key missing error
    #[test]
    fn test_api_key_missing_error() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::ApiKeyMissing,
            message: "TMDb API key is not configured".to_string(),
            retryable: false,
            retry_after: None,
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("apikey_missing"));
        assert!(!error.retryable);
    }

    // 测试 4: API key invalid error
    #[test]
    fn test_api_key_invalid_error() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::ApiKeyInvalid,
            message: "TMDb API key is invalid".to_string(),
            retryable: false,
            retry_after: None,
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("apikey_invalid"));
        assert!(!error.retryable);
    }

    // 测试 5: rate limited error
    #[test]
    fn test_rate_limited_error() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::RateLimited,
            message: "Rate limit exceeded".to_string(),
            retryable: true,
            retry_after: Some(60),
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("rate_limited"));
        assert!(error.retryable);
        assert_eq!(error.retry_after, Some(60));
    }

    // 测试 6: timeout error
    #[test]
    fn test_timeout_error() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::Timeout,
            message: "Request timeout".to_string(),
            retryable: true,
            retry_after: Some(30),
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("timeout"));
        assert!(error.retryable);
    }

    // 测试 7: network error
    #[test]
    fn test_network_error() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::NetworkError,
            message: "Network error".to_string(),
            retryable: true,
            retry_after: Some(10),
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("network_error"));
        assert!(error.retryable);
    }

    // 测试 8: unknown error
    #[test]
    fn test_unknown_error() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::Unknown,
            message: "Unknown error".to_string(),
            retryable: false,
            retry_after: None,
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("unknown"));
        assert!(!error.retryable);
    }

    // 测试 9: JSON 中不包含 API key
    #[test]
    fn test_json_no_api_key() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::ApiKeyMissing,
            message: "API key missing".to_string(),
            retryable: false,
            retry_after: None,
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(!json.contains("api_key"));
        assert!(!json.contains("apiKey"));
        assert!(!json.contains("token"));
    }

    // 测试 10: 不需要网络
    // （此测试不执行网络操作，仅验证类型定义）
    #[test]
    fn test_no_network_required() {
        // 此测试仅验证我们可以创建类型而不需要网络
        let _input = crate::tmdb_search_contract::input::SearchTmdbCandidatesInput {
            query: "test".to_string(),
            media_type: crate::tmdb_search_contract::media_type::TmdbSearchMediaType::Movie,
            language: "en".to_string(),
            year: None,
            page: None,
        };

        // 如果编译通过，说明类型定义正确
        assert!(true);
    }
}
