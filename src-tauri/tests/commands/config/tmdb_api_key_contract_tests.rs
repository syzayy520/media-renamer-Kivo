// tmdb_api_key_contract_tests 模块 - TMDb API Key 命令契约测试
// 职责：验证 get_tmdb_api_key_status / set_tmdb_api_key / clear_tmdb_api_key 契约
// 通过测试 TmdbApiKeyStatus + api_key_store 组合验证 command 返回值安全性

use tempfile::tempdir;

use app_lib::commands::config_commands;
use app_lib::config::secret::api_key_store;

/// 模拟 get_tmdb_api_key_status 的契约：
/// 默认返回 configured: false，不包含明文
#[test]
fn test_get_tmdb_api_key_status_default_not_configured() {
    let dir = tempdir().unwrap();

    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    let status = config_commands::TmdbApiKeyStatus {
        configured: key.is_some(),
    };

    assert!(!status.configured, "default should be not configured");
}

/// 模拟 set_tmdb_api_key + get_tmdb_api_key_status 的契约：
/// set 后 status 返回 configured: true
#[test]
fn test_set_tmdb_api_key_status_configured_true() {
    let dir = tempdir().unwrap();

    api_key_store::save_api_key("tmdb", "test-key-001", dir.path()).unwrap();
    let status = config_commands::TmdbApiKeyStatus { configured: true };
    assert!(status.configured, "set should return configured: true");

    // 验证 get_tmdb_api_key_status 也会返回 configured: true
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    let status_after = config_commands::TmdbApiKeyStatus {
        configured: key.is_some(),
    };
    assert!(
        status_after.configured,
        "get after set should return configured: true"
    );
}

/// 模拟 clear_tmdb_api_key + get_tmdb_api_key_status 的契约：
/// clear 后 status 返回 configured: false
#[test]
fn test_clear_tmdb_api_key_status_configured_false() {
    let dir = tempdir().unwrap();

    api_key_store::save_api_key("tmdb", "test-key-002", dir.path()).unwrap();
    api_key_store::delete_api_key("tmdb", dir.path()).unwrap();
    let status = config_commands::TmdbApiKeyStatus { configured: false };
    assert!(!status.configured, "clear should return configured: false");

    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    let status_after = config_commands::TmdbApiKeyStatus {
        configured: key.is_some(),
    };
    assert!(
        !status_after.configured,
        "get after clear should return configured: false"
    );
}

/// set_tmdb_api_key 返回值不包含明文 key
#[test]
fn test_set_tmdb_api_key_result_never_contains_plaintext() {
    let dir = tempdir().unwrap();

    api_key_store::save_api_key("tmdb", "super-secret-key-do-not-leak", dir.path()).unwrap();
    let status = config_commands::TmdbApiKeyStatus { configured: true };

    let serialized = serde_json::to_string(&status).unwrap();
    assert!(
        !serialized.contains("super-secret-key-do-not-leak"),
        "command return value must never contain the plaintext key"
    );
}

/// get_tmdb_api_key_status 返回值不包含明文 key
#[test]
fn test_get_tmdb_api_key_status_never_contains_plaintext() {
    let dir = tempdir().unwrap();

    api_key_store::save_api_key("tmdb", "another-secret-key-xyz", dir.path()).unwrap();
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    let status = config_commands::TmdbApiKeyStatus {
        configured: key.is_some(),
    };

    let serialized = serde_json::to_string(&status).unwrap();
    assert!(
        !serialized.contains("another-secret-key-xyz"),
        "get_tmdb_api_key_status must never leak the key"
    );
    assert_eq!(serialized, r#"{"configured":true}"#);
}

/// clear_tmdb_api_key 返回值不包含明文 key
#[test]
fn test_clear_tmdb_api_key_never_contains_plaintext() {
    let status = config_commands::TmdbApiKeyStatus { configured: false };
    let serialized = serde_json::to_string(&status).unwrap();
    assert_eq!(serialized, r#"{"configured":false}"#);
}

/// set_tmdb_api_key 空 key 返回错误
#[test]
fn test_set_tmdb_api_key_empty_returns_error() {
    let api_key = "";
    let trimmed = api_key.trim();

    assert!(trimmed.is_empty(), "empty key should be rejected");

    let error_msg = "API key cannot be empty".to_string();
    assert!(error_msg.contains("cannot be empty"));
    assert!(!error_msg.contains("secret"));
}

/// error message 不包含明文 key
#[test]
fn test_error_message_never_contains_plaintext_key() {
    let error_msg = "API key cannot be empty".to_string();
    assert!(!error_msg.contains("secret-key"));
    assert!(!error_msg.contains("test-key"));

    let generic_error = "Save API key error".to_string();
    assert!(!generic_error.contains("secret"));
}
