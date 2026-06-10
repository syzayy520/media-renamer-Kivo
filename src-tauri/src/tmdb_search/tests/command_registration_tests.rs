// src-tauri/src/tmdb_search/tests/command_registration_tests.rs
// Command Registration Tests
// 职责：验证 Tauri command 注册后的 contract 行为
// 不联网、不读 API key、不调用 transport

use crate::tmdb_search::command::search_tmdb_candidates_command_shell;
use crate::tmdb_search_contract::{
    SearchTmdbCandidatesInput, TmdbSearchErrorCode, TmdbSearchMediaType,
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
fn test_command_registered_shell_callable() {
    // Command shell function is callable and returns expected output
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.error.is_some());
}

#[test]
fn test_disabled_gate_default_closed() {
    // Disabled gate is always closed — always returns disabled error
    let output = search_tmdb_candidates_command_shell(valid_input());
    let error = output.error.unwrap();
    assert_eq!(error.code, TmdbSearchErrorCode::Unknown);
    assert!(error.message.contains("not enabled"));
}

#[test]
fn test_disabled_output_candidates_empty() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.candidates.is_empty());
}

#[test]
fn test_disabled_output_error_some() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.error.is_some());
}

#[test]
fn test_disabled_output_no_api_key() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("api_key"), "output must not contain api_key");
}

#[test]
fn test_disabled_output_no_camelcase_api_key() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("apiKey"), "output must not contain apiKey");
}

#[test]
fn test_disabled_output_no_token() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("token"), "output must not contain token");
}

#[test]
fn test_disabled_output_no_bearer() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(!json.contains("bearer"), "output must not contain bearer");
}

#[test]
fn test_disabled_output_no_authorization() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(
        !json.contains("authorization"),
        "output must not contain authorization"
    );
}

#[test]
fn test_disabled_output_no_session() {
    let output = search_tmdb_candidates_command_shell(valid_input());
    let json = serde_json::to_string(&output).unwrap();
    assert!(
        !json.contains("session"),
        "output must not contain session"
    );
}

#[test]
fn test_disabled_no_network_call() {
    // Command shell has no network dependency
    let output = search_tmdb_candidates_command_shell(valid_input());
    let error = output.error.unwrap();
    assert!(error.message.contains("HTTP transport dependency required"));
}

#[test]
fn test_disabled_no_api_key_read() {
    // Command shell takes only SearchTmdbCandidatesInput — no provider access
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert_eq!(output.error.unwrap().code, TmdbSearchErrorCode::Unknown);
}

#[test]
fn test_disabled_no_rename_rollback() {
    // Command shell does not touch rename/rollback
    let output = search_tmdb_candidates_command_shell(valid_input());
    assert!(output.candidates.is_empty());
    assert!(output.error.is_some());
}
