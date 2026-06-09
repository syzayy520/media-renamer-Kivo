// src-tauri/src/tmdb_search_contract/tests/serialization_contract_tests.rs
// TMDb Search Backend Contract 序列化测试
// 职责：验证 contract 类型的序列化/反序列化行为

use crate::tmdb_search_contract::candidate::{TmdbCandidate, TmdbCandidateSource};
use crate::tmdb_search_contract::error::{TmdbSearchError, TmdbSearchErrorCode};
use crate::tmdb_search_contract::input::SearchTmdbCandidatesInput;
use crate::tmdb_search_contract::media_type::TmdbSearchMediaType;
use crate::tmdb_search_contract::output::SearchTmdbCandidatesOutput;
use crate::tmdb_search_contract::rate_limit::TmdbRateLimitState;
use serde_json;

#[cfg(test)]
mod tests {
    use super::*;

    // 测试 1: input serialization
    #[test]
    fn test_input_serialization() {
        let input = SearchTmdbCandidatesInput {
            query: "Fight Club".to_string(),
            media_type: TmdbSearchMediaType::Movie,
            language: "en".to_string(),
            year: Some(1999),
            page: Some(1),
        };

        let json = serde_json::to_string(&input).unwrap();
        let deserialized: SearchTmdbCandidatesInput = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.query, "Fight Club");
        assert!(matches!(
            deserialized.media_type,
            TmdbSearchMediaType::Movie
        ));
        assert_eq!(deserialized.language, "en");
        assert_eq!(deserialized.year, Some(1999));
        assert_eq!(deserialized.page, Some(1));

        // 验证 JSON 中不出现 api_key
        assert!(!json.contains("api_key"));
        assert!(!json.contains("apiKey"));
        assert!(!json.contains("token"));
    }

    // 测试 2: output empty candidates serialization
    #[test]
    fn test_output_empty_candidates_serialization() {
        let output = SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: None,
        };

        let json = serde_json::to_string(&output).unwrap();
        let deserialized: SearchTmdbCandidatesOutput = serde_json::from_str(&json).unwrap();

        assert!(deserialized.candidates.is_empty());
        assert!(matches!(deserialized.source, TmdbCandidateSource::Tmdb));
        assert!(deserialized.error.is_none());

        // 空 candidates 不等于 error
        assert!(deserialized.error.is_none());
    }

    // 测试 3: candidate serialization does not include API key fields
    #[test]
    fn test_candidate_serialization_no_api_key() {
        let candidate = TmdbCandidate {
            id: "candidate-1".to_string(),
            tmdb_id: 550,
            title: "Fight Club".to_string(),
            original_title: Some("Fight Club".to_string()),
            media_type: TmdbSearchMediaType::Movie,
            release_year: Some(1999),
            overview: Some("A ticking-Loss-Loss-Loss bomb insomniac...".to_string()),
            poster_path: Some("/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg".to_string()),
            backdrop_path: Some("/hZkgoQYus5dXo3H8T7Uef6DNknx.jpg".to_string()),
            language: Some("en".to_string()),
            popularity: Some(61.4),
            vote_average: Some(8.4),
            confidence_hint: Some(0.95),
            match_reasons: vec!["Title match".to_string(), "Year match".to_string()],
        };

        let json = serde_json::to_string(&candidate).unwrap();

        // 验证 JSON 中不出现 API key 相关字段
        assert!(!json.contains("api_key"));
        assert!(!json.contains("apiKey"));
        assert!(!json.contains("token"));
        assert!(!json.contains("secret"));
    }

    // 测试 4: error code serialization
    #[test]
    fn test_error_code_serialization() {
        let error = TmdbSearchError {
            code: TmdbSearchErrorCode::RateLimited,
            message: "Rate limit exceeded".to_string(),
            retryable: true,
            retry_after: Some(60),
        };

        let json = serde_json::to_string(&error).unwrap();
        let deserialized: TmdbSearchError = serde_json::from_str(&json).unwrap();

        assert!(matches!(
            deserialized.code,
            TmdbSearchErrorCode::RateLimited
        ));
        assert!(deserialized.retryable);
        assert_eq!(deserialized.retry_after, Some(60));
    }

    // 测试 5: rate limit serialization
    #[test]
    fn test_rate_limit_serialization() {
        let rate_limit = TmdbRateLimitState {
            remaining: 40,
            reset_at: 1717958400,
            limit: Some(50),
        };

        let json = serde_json::to_string(&rate_limit).unwrap();
        let deserialized: TmdbRateLimitState = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.remaining, 40);
        assert_eq!(deserialized.reset_at, 1717958400);
        assert_eq!(deserialized.limit, Some(50));
    }

    // 测试 6: media type serialization
    #[test]
    fn test_media_type_serialization() {
        let movie = TmdbSearchMediaType::Movie;
        let tv = TmdbSearchMediaType::Tv;

        let movie_json = serde_json::to_string(&movie).unwrap();
        let tv_json = serde_json::to_string(&tv).unwrap();

        assert_eq!(movie_json, "\"movie\"");
        assert_eq!(tv_json, "\"tv\"");

        let deserialized_movie: TmdbSearchMediaType = serde_json::from_str(&movie_json).unwrap();
        let deserialized_tv: TmdbSearchMediaType = serde_json::from_str(&tv_json).unwrap();

        assert!(matches!(deserialized_movie, TmdbSearchMediaType::Movie));
        assert!(matches!(deserialized_tv, TmdbSearchMediaType::Tv));
    }

    // 测试 7: source serialization
    #[test]
    fn test_source_serialization() {
        let source = TmdbCandidateSource::Tmdb;

        let json = serde_json::to_string(&source).unwrap();
        assert_eq!(json, "\"tmdb\"");

        let deserialized: TmdbCandidateSource = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, TmdbCandidateSource::Tmdb));
    }

    // 测试 8: error retryable / retry_after contract
    #[test]
    fn test_error_retryable_contract() {
        let retryable_error = TmdbSearchError {
            code: TmdbSearchErrorCode::RateLimited,
            message: "Rate limited".to_string(),
            retryable: true,
            retry_after: Some(30),
        };

        let non_retryable_error = TmdbSearchError {
            code: TmdbSearchErrorCode::ApiKeyInvalid,
            message: "Invalid API key".to_string(),
            retryable: false,
            retry_after: None,
        };

        assert!(retryable_error.retryable);
        assert_eq!(retryable_error.retry_after, Some(30));

        assert!(!non_retryable_error.retryable);
        assert_eq!(non_retryable_error.retry_after, None);
    }
}
