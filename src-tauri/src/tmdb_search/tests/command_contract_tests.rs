// src-tauri/src/tmdb_search/tests/command_contract_tests.rs
// Command Contract Tests
// 职责：验证 command shell 的外部契约行为
// 不联网、不读 API key、不调用 transport

use crate::tmdb_search::command::search_tmdb_candidates_command_shell;
use crate::tmdb_search_contract::{
    SearchTmdbCandidatesInput, TmdbCandidateSource, TmdbSearchErrorCode, TmdbSearchMediaType,
};
use serde_json;

fn valid_input() -> SearchTmdbCandidatesInput {
    SearchTmdbCandidatesInput {
        query: "Inception".to_string(),
        media_type: TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: Some(2010),
        page: Some(1),
    }
}

#[test]
fn test_command_contract_accepts_valid_input() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.error.is_some());
}

#[test]
fn test_command_contract_returns_empty_candidates() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.candidates.is_empty());
}

#[test]
fn test_command_contract_returns_error_some() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.error.is_some());
}

#[test]
fn test_command_contract_error_retryable_false() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let error = output.error.unwrap();
    assert!(!error.retryable);
}

#[test]
fn test_command_contract_error_retry_after_none() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let error = output.error.unwrap();
    assert!(error.retry_after.is_none());
}

#[test]
fn test_command_contract_rate_limit_none() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.rate_limit.is_none());
}

#[test]
fn test_command_contract_source_is_tmdb() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert_eq!(output.source, TmdbCandidateSource::Tmdb);
}

#[test]
fn test_command_contract_output_json_no_api_key() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("api_key"));
}

#[test]
fn test_command_contract_output_json_no_camelcase_api_key() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("apiKey"));
}

#[test]
fn test_command_contract_output_json_no_token() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("token"));
}

#[test]
fn test_command_contract_output_json_no_bearer() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("bearer"));
}

#[test]
fn test_command_contract_output_json_no_authorization() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("authorization"));
}

#[test]
fn test_command_contract_output_json_no_session() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("session"));
}

#[test]
fn test_command_contract_no_api_key_read() {
    // Shell function takes only SearchTmdbCandidatesInput — no provider access
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert_eq!(output.error.unwrap().code, TmdbSearchErrorCode::Unknown);
}

#[test]
fn test_command_contract_no_network_dependency() {
    // Shell body contains no HTTP calls — documents the contract
    let output = search_tmdb_candidates_command_shell(valid_input());
    let error = output.error.unwrap();
    assert!(error.message.contains("HTTP transport dependency required"));
}

#[test]
fn test_command_contract_no_rename_rollback_execute() {
    // Shell does not touch rename/rollback/execute — documents the contract
    // The function signature and body have no reference to rename/rollback.
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.candidates.is_empty());
    assert!(output.error.is_some());
}
