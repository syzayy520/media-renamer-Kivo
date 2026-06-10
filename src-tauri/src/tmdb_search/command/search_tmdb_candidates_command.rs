// src-tauri/src/tmdb_search/command/search_tmdb_candidates_command.rs
// TMDb Search Command Shell
// 职责：提供 TMDb 搜索 command shell，返回结构化 disabled 错误
// 不联网、不读 API key、不调用 transport、不注册 Tauri command

use crate::tmdb_search_contract::{
    SearchTmdbCandidatesInput, SearchTmdbCandidatesOutput, TmdbCandidateSource, TmdbSearchError,
    TmdbSearchErrorCode,
};

/// TMDb 搜索 command shell
///
/// 始终返回结构化错误，表示 live search 未启用。
/// 不读取 API key、不联网、不调用 transport。
///
/// # Arguments
/// * `input` - 搜索输入（复用 P2-011 contract）
///
/// # Returns
/// `SearchTmdbCandidatesOutput` 包含空候选列表和结构化错误
pub fn search_tmdb_candidates_command_shell(
    _input: SearchTmdbCandidatesInput,
) -> SearchTmdbCandidatesOutput {
    SearchTmdbCandidatesOutput {
        candidates: vec![],
        source: TmdbCandidateSource::Tmdb,
        rate_limit: None,
        error: Some(TmdbSearchError {
            code: TmdbSearchErrorCode::Unknown,
            message: "TMDb live search is not enabled. HTTP transport dependency required. Command shell only.".to_string(),
            retryable: false,
            retry_after: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn make_valid_input() -> SearchTmdbCandidatesInput {
        use crate::tmdb_search_contract::TmdbSearchMediaType;
        SearchTmdbCandidatesInput {
            query: "Inception".to_string(),
            media_type: TmdbSearchMediaType::Movie,
            language: "en".to_string(),
            year: Some(2010),
            page: Some(1),
        }
    }

    #[test]
    fn test_command_shell_accepts_valid_input() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        // Should return without panic
        assert!(output.error.is_some());
    }

    #[test]
    fn test_command_shell_returns_empty_candidates() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        assert!(output.candidates.is_empty());
    }

    #[test]
    fn test_command_shell_returns_error_some() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        assert!(output.error.is_some());
    }

    #[test]
    fn test_command_shell_error_retryable_false() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let error = output.error.unwrap();
        assert!(!error.retryable);
    }

    #[test]
    fn test_command_shell_error_retry_after_none() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let error = output.error.unwrap();
        assert!(error.retry_after.is_none());
    }

    #[test]
    fn test_command_shell_rate_limit_none() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        assert!(output.rate_limit.is_none());
    }

    #[test]
    fn test_command_shell_source_is_tmdb() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        assert_eq!(output.source, TmdbCandidateSource::Tmdb);
    }

    #[test]
    fn test_command_shell_output_json_no_api_key() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let json = serde_json::to_string(&output).unwrap();
        assert!(!json.contains("api_key"), "output JSON must not contain api_key");
    }

    #[test]
    fn test_command_shell_output_json_no_camelcase_api_key() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let json = serde_json::to_string(&output).unwrap();
        assert!(!json.contains("apiKey"), "output JSON must not contain apiKey");
    }

    #[test]
    fn test_command_shell_output_json_no_token() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let json = serde_json::to_string(&output).unwrap();
        assert!(!json.contains("token"), "output JSON must not contain token");
    }

    #[test]
    fn test_command_shell_output_json_no_bearer() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let json = serde_json::to_string(&output).unwrap();
        assert!(!json.contains("bearer"), "output JSON must not contain bearer");
    }

    #[test]
    fn test_command_shell_output_json_no_authorization() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let json = serde_json::to_string(&output).unwrap();
        assert!(
            !json.contains("authorization"),
            "output JSON must not contain authorization"
        );
    }

    #[test]
    fn test_command_shell_output_json_no_session() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let json = serde_json::to_string(&output).unwrap();
        assert!(
            !json.contains("session"),
            "output JSON must not contain session"
        );
    }

    #[test]
    fn test_command_shell_no_api_key_read() {
        // The function signature takes only SearchTmdbCandidatesInput
        // and has no access to TmdbApiKeyProvider — no API key read possible.
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        // Verify error is returned (shell always returns error)
        assert_eq!(output.error.unwrap().code, TmdbSearchErrorCode::Unknown);
    }

    #[test]
    fn test_command_shell_no_network_dependency() {
        // The function body contains no HTTP client calls.
        // This test documents the contract: no network, no transport.
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let error = output.error.unwrap();
        assert!(error.message.contains("HTTP transport dependency required"));
    }

    #[test]
    fn test_command_shell_error_code_is_unknown() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let error = output.error.unwrap();
        assert_eq!(error.code, TmdbSearchErrorCode::Unknown);
    }

    #[test]
    fn test_command_shell_error_message_descriptive() {
        let input = make_valid_input();
        let output = search_tmdb_candidates_command_shell(input);
        let error = output.error.unwrap();
        assert!(error.message.contains("not enabled"));
        assert!(error.message.contains("Command shell only"));
    }

    #[test]
    fn test_command_shell_with_different_media_type() {
        use crate::tmdb_search_contract::TmdbSearchMediaType;
        let input = SearchTmdbCandidatesInput {
            query: "Breaking Bad".to_string(),
            media_type: TmdbSearchMediaType::Tv,
            language: "zh-CN".to_string(),
            year: Some(2008),
            page: None,
        };
        let output = search_tmdb_candidates_command_shell(input);
        assert!(output.candidates.is_empty());
        assert!(output.error.is_some());
        assert_eq!(output.source, TmdbCandidateSource::Tmdb);
    }

    #[test]
    fn test_command_shell_with_minimal_input() {
        use crate::tmdb_search_contract::TmdbSearchMediaType;
        let input = SearchTmdbCandidatesInput {
            query: "test".to_string(),
            media_type: TmdbSearchMediaType::Movie,
            language: "en".to_string(),
            year: None,
            page: None,
        };
        let output = search_tmdb_candidates_command_shell(input);
        assert!(output.candidates.is_empty());
        assert!(output.rate_limit.is_none());
        assert!(output.error.is_some());
    }
}
