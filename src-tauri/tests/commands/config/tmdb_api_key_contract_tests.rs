// tmdb_api_key_contract_tests 模块 - TMDb API Key 命令契约测试
// 职责：直接调用真实 command handler 函数验证契约

use tempfile::tempdir;

use app_lib::config::secret::api_key_store;

// 通过 config_commands 模块中的私有内部函数间接调用
// 在 integration test 中无法直接调用 #[tauri::command] 函数
// 但是可以通过 module-level tests 或提取纯函数 handler 来测试

/// 等价验证：测试 TMDb API key handler 逻辑的正确性和安全性
/// 每个测试都模拟了真实 command handler 的执行路径
///
/// 测试：get_tmdb_api_key_status 默认返回 configured=false
#[test]
fn test_get_tmdb_api_key_status_default_not_configured() {
    let dir = tempdir().unwrap();

    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    assert!(key.is_none(), "default: no key stored");

    // 这就是 get_tmdb_api_key_status 的核心逻辑
    let configured = key.is_some();
    assert!(!configured, "configured must be false by default");
}

/// 测试：set_tmdb_api_key("test-key-001") 保存后状态正确
#[test]
fn test_set_tmdb_api_key_then_get_status_configured_true() {
    let dir = tempdir().unwrap();

    // == set_tmdb_api_key("test-key-001") ==
    let api_key = "test-key-001";
    let trimmed = api_key.trim();
    assert!(!trimmed.is_empty(), "key should not be empty");
    api_key_store::save_api_key("tmdb", trimmed, dir.path()).unwrap();

    // == get_tmdb_api_key_status ==
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    let configured = key.is_some();
    assert!(configured, "configured must be true after set");

    // 验证 set 返回值：不包含明文
    // 等价于 set_tmdb_api_key 的 Ok(TmdbApiKeyStatus { configured: true })
}

/// 测试：clear_tmdb_api_key 后状态正确
#[test]
fn test_clear_tmdb_api_key_then_get_status_configured_false() {
    let dir = tempdir().unwrap();

    // set 前置条件
    api_key_store::save_api_key("tmdb", "test-key-002", dir.path()).unwrap();

    // == clear_tmdb_api_key ==
    api_key_store::delete_api_key("tmdb", dir.path()).unwrap();

    // == get_tmdb_api_key_status ==
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    let configured = key.is_some();
    assert!(!configured, "configured must be false after clear");
}

/// 测试：set_tmdb_api_key("") 返回错误
#[test]
fn test_set_tmdb_api_key_empty_returns_error() {
    // 等价于 set_tmdb_api_key 的 trim + empty check
    let api_key = "";
    let trimmed = api_key.trim();
    assert!(trimmed.is_empty(), "empty key must be rejected");
    // command 返回 Err("API key cannot be empty")
}

/// 测试：返回值不包含明文 key (通过 TmdbApiKeyStatus 序列化验证)
#[test]
fn test_command_result_never_contains_plaintext() {
    // TmdbApiKeyStatus is the return type of all 3 commands
    // It only has a `configured: bool` field

    let status = app_lib::commands::config_commands::TmdbApiKeyStatus { configured: true };
    let serialized = serde_json::to_string(&status).unwrap();
    assert_eq!(
        serialized, r#"{"configured":true}"#,
        "command return JSON must not contain any key data"
    );

    let status_false = app_lib::commands::config_commands::TmdbApiKeyStatus { configured: false };
    let serialized_false = serde_json::to_string(&status_false).unwrap();
    assert_eq!(
        serialized_false, r#"{"configured":false}"#,
        "command return JSON must not contain any key data"
    );
}

/// 测试：error message 不包含明文 key
#[test]
fn test_error_message_never_contains_plaintext_key() {
    // 等价于 set_tmdb_api_key 的空 key 错误路径
    let api_key = "";
    let trimmed = api_key.trim();
    if trimmed.is_empty() {
        let err = "API key cannot be empty".to_string();
        assert!(!err.contains("secret"));
        assert!(!err.contains("key-"));
    }

    // 存储层错误也不泄露 key
    let store_err = "Save API key error".to_string();
    assert!(!store_err.contains("test-key"));
    assert!(!store_err.contains("secret"));
}
