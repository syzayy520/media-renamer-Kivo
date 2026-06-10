// config_commands 模块 - 配置相关 Tauri 命令
// 职责：将 config 子模块暴露为 Tauri 命令
// 参数转换：String ↔ MediaType，AppError → String

use std::path::PathBuf;

use tauri::Manager;

use crate::config::config_loader::{self, AppConfig};
use crate::config::secret::api_key_store;
use crate::config::template_manager::{self, RenameRule};
use crate::config::threshold;
use crate::parse::movie_parser::MediaType;
use crate::tmdb_search::command::TmdbSearchState;

/// 获取配置目录路径
fn config_dir(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir).ok();
    app_dir
}

/// 获取完整应用配置
#[tauri::command]
pub fn get_app_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    let dir = config_dir(&app_handle);
    config_loader::load_config(&dir).map_err(|e| e.to_string())
}

/// 获取所有重命名模板
#[tauri::command]
pub fn get_all_templates(app_handle: tauri::AppHandle) -> Result<Vec<RenameRule>, String> {
    let dir = config_dir(&app_handle);
    template_manager::get_all_templates(&dir).map_err(|e| e.to_string())
}

/// 设置指定媒体类型的模板
#[tauri::command]
pub fn set_template(
    app_handle: tauri::AppHandle,
    media_type: String,
    template: String,
) -> Result<(), String> {
    let dir = config_dir(&app_handle);
    let mt = parse_media_type(&media_type)?;
    template_manager::set_template(&mt, &template, &dir).map_err(|e| e.to_string())
}

/// 获取置信度阈值
#[tauri::command]
pub fn get_confidence_threshold(app_handle: tauri::AppHandle) -> Result<u8, String> {
    let dir = config_dir(&app_handle);
    threshold::get_threshold(&dir).map_err(|e| e.to_string())
}

/// 设置置信度阈值
#[tauri::command]
pub fn set_confidence_threshold(app_handle: tauri::AppHandle, value: u8) -> Result<(), String> {
    let dir = config_dir(&app_handle);
    threshold::set_threshold(value, &dir).map_err(|e| e.to_string())
}

/// 解析媒体类型字符串
fn parse_media_type(s: &str) -> Result<MediaType, String> {
    match s.to_lowercase().as_str() {
        "movie" => Ok(MediaType::Movie),
        "series" => Ok(MediaType::Series),
        "anime" => Ok(MediaType::Anime),
        "special" => Ok(MediaType::Special),
        "ova" => Ok(MediaType::Ova),
        "extras" => Ok(MediaType::Extras),
        "unknown" => Ok(MediaType::Unknown),
        _ => Err(format!("Unknown media type: {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_media_type_valid() {
        assert_eq!(parse_media_type("movie").unwrap(), MediaType::Movie);
        assert_eq!(parse_media_type("Series").unwrap(), MediaType::Series);
        assert_eq!(parse_media_type("ANIME").unwrap(), MediaType::Anime);
    }

    #[test]
    fn test_parse_media_type_invalid() {
        assert!(parse_media_type("invalid").is_err());
    }

    // === TMDb API Key command handler unit tests ===

    #[test]
    fn test_do_get_tmdb_api_key_status_default_not_configured() {
        let dir = tempfile::tempdir().unwrap();
        let status = do_get_tmdb_api_key_status(dir.path()).unwrap();
        assert!(!status.configured);
    }

    #[test]
    fn test_do_set_tmdb_api_key_status_configured_true() {
        let dir = tempfile::tempdir().unwrap();
        let status = do_set_tmdb_api_key(dir.path(), "test-key-u8-001").unwrap();
        assert!(status.configured);
    }

    #[test]
    fn test_do_get_after_set_configured_true() {
        let dir = tempfile::tempdir().unwrap();
        do_set_tmdb_api_key(dir.path(), "test-key-u8-002").unwrap();
        let status = do_get_tmdb_api_key_status(dir.path()).unwrap();
        assert!(status.configured);
    }

    #[test]
    fn test_do_clear_tmdb_api_key_status_configured_false() {
        let dir = tempfile::tempdir().unwrap();
        do_set_tmdb_api_key(dir.path(), "test-key-u8-003").unwrap();
        let status = do_clear_tmdb_api_key(dir.path()).unwrap();
        assert!(!status.configured);
    }

    #[test]
    fn test_do_get_after_clear_configured_false() {
        let dir = tempfile::tempdir().unwrap();
        do_set_tmdb_api_key(dir.path(), "test-key-u8-004").unwrap();
        do_clear_tmdb_api_key(dir.path()).unwrap();
        let status = do_get_tmdb_api_key_status(dir.path()).unwrap();
        assert!(!status.configured);
    }

    #[test]
    fn test_do_set_tmdb_api_key_empty_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let result = do_set_tmdb_api_key(dir.path(), "");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("cannot be empty"));
    }

    #[test]
    fn test_do_set_tmdb_api_key_whitespace_only_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let result = do_set_tmdb_api_key(dir.path(), "   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_tmdb_api_key_status_serialization_no_plaintext() {
        let status = TmdbApiKeyStatus { configured: true };
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#"{"configured":true}"#);

        let status_false = TmdbApiKeyStatus { configured: false };
        let json_false = serde_json::to_string(&status_false).unwrap();
        assert_eq!(json_false, r#"{"configured":false}"#);
    }

    // === TMDb Config Status tests ===

    #[test]
    fn test_do_get_tmdb_config_status_not_configured() {
        let dir = tempfile::tempdir().unwrap();
        let status = do_get_tmdb_config_status(dir.path(), false).unwrap();
        assert!(!status.api_key_configured);
        assert!(!status.gate_enabled);
        assert_eq!(status.status, TmdbStatusLevel::NotConfigured);
    }

    #[test]
    fn test_do_get_tmdb_config_status_gate_disabled() {
        let dir = tempfile::tempdir().unwrap();
        do_set_tmdb_api_key(dir.path(), "test-key").unwrap();
        let status = do_get_tmdb_config_status(dir.path(), false).unwrap();
        assert!(status.api_key_configured);
        assert!(!status.gate_enabled);
        assert_eq!(status.status, TmdbStatusLevel::GateDisabled);
    }

    #[test]
    fn test_do_get_tmdb_config_status_ready() {
        let dir = tempfile::tempdir().unwrap();
        do_set_tmdb_api_key(dir.path(), "test-key").unwrap();
        let status = do_get_tmdb_config_status(dir.path(), true).unwrap();
        assert!(status.api_key_configured);
        assert!(status.gate_enabled);
        assert_eq!(status.status, TmdbStatusLevel::Ready);
    }

    #[test]
    fn test_tmdb_config_status_serialization_no_sensitive_data() {
        let status = TmdbConfigStatus {
            api_key_configured: true,
            gate_enabled: true,
            status: TmdbStatusLevel::Ready,
            message: "Ready".to_string(),
        };
        let json = serde_json::to_string(&status).unwrap();
        // Verify no actual API key values are present (long alphanumeric strings)
        // Field names like "api_key_configured" are acceptable - only values matter
        assert!(!json.contains("sk-"));
        assert!(!json.contains("Bearer "));
        assert!(!json.contains("tmdb_key="));
        // Verify the status structure is complete and correct
        assert!(json.contains("api_key_configured"));
        assert!(json.contains("gate_enabled"));
        assert!(json.contains("Ready"));
    }

    #[test]
    fn test_tmdb_status_level_serialization() {
        let levels = vec![
            TmdbStatusLevel::NotConfigured,
            TmdbStatusLevel::GateDisabled,
            TmdbStatusLevel::Ready,
            TmdbStatusLevel::Connected,
            TmdbStatusLevel::ConnectionFailed,
        ];
        for level in levels {
            let json = serde_json::to_string(&level).unwrap();
            assert!(!json.is_empty());
        }
    }

    // === API Key Save/Clear Closed-Loop Proof Tests ===

    #[test]
    fn save_key_does_not_leak_full_key() {
        let dir = tempfile::tempdir().unwrap();
        let full_key = "tmdb_abcdef1234567890xyz";

        // Save key
        let status = do_set_tmdb_api_key(dir.path(), full_key).unwrap();
        assert!(status.configured);

        // Verify: returned status contains NO key content
        let json = serde_json::to_string(&status).unwrap();
        assert!(!json.contains(full_key));
        assert!(!json.contains("abcdef1234567890"));
        assert_eq!(json, r#"{"configured":true}"#);

        // Verify: config status also contains NO key content
        let config_status = do_get_tmdb_config_status(dir.path(), true).unwrap();
        let config_json = serde_json::to_string(&config_status).unwrap();
        assert!(!config_json.contains(full_key));
        assert!(!config_json.contains("abcdef1234567890"));
        // Only boolean flag, never the actual key
        assert!(config_json.contains("api_key_configured"));
        assert!(config_json.contains("true"));
    }

    #[test]
    fn clear_key_removes_configured_status() {
        let dir = tempfile::tempdir().unwrap();

        // Save then clear
        do_set_tmdb_api_key(dir.path(), "test-key-12345").unwrap();
        let clear_status = do_clear_tmdb_api_key(dir.path()).unwrap();
        assert!(!clear_status.configured);

        // Verify: get status also shows not configured
        let get_status = do_get_tmdb_api_key_status(dir.path()).unwrap();
        assert!(!get_status.configured);

        // Verify: config status shows NotConfigured
        let config_status = do_get_tmdb_config_status(dir.path(), true).unwrap();
        assert!(!config_status.api_key_configured);
        assert_eq!(config_status.status, TmdbStatusLevel::NotConfigured);
    }

    #[test]
    fn status_json_only_exposes_configured_boolean() {
        let dir = tempfile::tempdir().unwrap();
        do_set_tmdb_api_key(dir.path(), "super_secret_key_12345").unwrap();

        // TmdbApiKeyStatus: only {configured: true}
        let api_status = do_get_tmdb_api_key_status(dir.path()).unwrap();
        let api_json = serde_json::to_string(&api_status).unwrap();
        assert_eq!(api_json, r#"{"configured":true}"#);
        assert!(!api_json.contains("super_secret"));
        assert!(!api_json.contains("key_12345"));

        // TmdbConfigStatus: api_key_configured is boolean, no key content
        let config_status = do_get_tmdb_config_status(dir.path(), true).unwrap();
        let config_json = serde_json::to_string(&config_status).unwrap();
        assert!(!config_json.contains("super_secret"));
        assert!(!config_json.contains("key_12345"));
        // Contains the boolean field name but NOT the key value
        assert!(config_json.contains(r#""api_key_configured":true"#));
    }
}

/// TMDb API Key 状态响应
#[derive(Debug, Clone, serde::Serialize)]
pub struct TmdbApiKeyStatus {
    pub configured: bool,
}

// === TMDb API Key internal handlers (testable without AppHandle) ===

fn do_get_tmdb_api_key_status(config_dir: &std::path::Path) -> Result<TmdbApiKeyStatus, String> {
    let key = api_key_store::get_api_key("tmdb", config_dir).map_err(|e| e.to_string())?;
    Ok(TmdbApiKeyStatus {
        configured: key.is_some(),
    })
}

fn do_set_tmdb_api_key(
    config_dir: &std::path::Path,
    api_key: &str,
) -> Result<TmdbApiKeyStatus, String> {
    let trimmed = api_key.trim();
    if trimmed.is_empty() {
        return Err("API key cannot be empty".to_string());
    }
    api_key_store::save_api_key("tmdb", trimmed, config_dir).map_err(|e| e.to_string())?;
    Ok(TmdbApiKeyStatus { configured: true })
}

fn do_clear_tmdb_api_key(config_dir: &std::path::Path) -> Result<TmdbApiKeyStatus, String> {
    api_key_store::delete_api_key("tmdb", config_dir).map_err(|e| e.to_string())?;
    Ok(TmdbApiKeyStatus { configured: false })
}

/// 获取 TMDb API Key 状态（不返回明文）
#[tauri::command]
pub fn get_tmdb_api_key_status(app_handle: tauri::AppHandle) -> Result<TmdbApiKeyStatus, String> {
    do_get_tmdb_api_key_status(&config_dir(&app_handle))
}

/// 设置 TMDb API Key
#[tauri::command]
pub fn set_tmdb_api_key(
    app_handle: tauri::AppHandle,
    api_key: String,
) -> Result<TmdbApiKeyStatus, String> {
    do_set_tmdb_api_key(&config_dir(&app_handle), &api_key)
}

/// 清除 TMDb API Key
#[tauri::command]
pub fn clear_tmdb_api_key(app_handle: tauri::AppHandle) -> Result<TmdbApiKeyStatus, String> {
    do_clear_tmdb_api_key(&config_dir(&app_handle))
}

// === TMDb Gate Control Commands ===

/// TMDb 配置状态响应
///
/// 包含 API key 状态和 gate 状态的综合信息
/// 不包含任何敏感数据（API key 明文等）
#[derive(Debug, Clone, serde::Serialize)]
pub struct TmdbConfigStatus {
    /// API key 是否已配置
    pub api_key_configured: bool,
    /// Gate 是否启用
    pub gate_enabled: bool,
    /// 综合状态描述
    pub status: TmdbStatusLevel,
    /// 用户可读的状态消息
    pub message: String,
}

/// TMDb 状态级别
#[derive(Debug, Clone, serde::Serialize, PartialEq)]
pub enum TmdbStatusLevel {
    /// 未配置 API key
    NotConfigured,
    /// 已配置但 gate 关闭
    GateDisabled,
    /// 已配置且 gate 开启，待测试
    Ready,
    /// 连接测试成功
    Connected,
    /// 连接测试失败
    ConnectionFailed,
}

/// 获取 TMDb 综合配置状态
#[tauri::command]
pub fn get_tmdb_config_status(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, TmdbSearchState>,
) -> Result<TmdbConfigStatus, String> {
    let dir = config_dir(&app_handle);
    let api_key_status = do_get_tmdb_api_key_status(&dir)?;
    let gate_enabled = state.is_gate_enabled();

    let (status, message) = if !api_key_status.configured {
        (
            TmdbStatusLevel::NotConfigured,
            "TMDb API key 未配置。请在设置中添加您的 API key。".to_string(),
        )
    } else if !gate_enabled {
        (
            TmdbStatusLevel::GateDisabled,
            "TMDb live search 已禁用。请在设置中启用。".to_string(),
        )
    } else {
        (
            TmdbStatusLevel::Ready,
            "TMDb 已就绪。可以进行搜索。".to_string(),
        )
    };

    Ok(TmdbConfigStatus {
        api_key_configured: api_key_status.configured,
        gate_enabled,
        status,
        message,
    })
}

/// 获取 TMDb gate 状态
#[tauri::command]
pub fn get_tmdb_gate_status(
    state: tauri::State<'_, TmdbSearchState>,
) -> Result<bool, String> {
    Ok(state.is_gate_enabled())
}

/// 设置 TMDb gate 启用/禁用
#[tauri::command]
pub fn set_tmdb_gate_enabled(
    enabled: bool,
    state: tauri::State<'_, TmdbSearchState>,
) -> Result<bool, String> {
    if enabled {
        state.enable_gate();
    } else {
        state.disable_gate();
    }
    Ok(state.is_gate_enabled())
}

/// 测试 TMDb 连接
///
/// 仅在用户主动调用时执行，不会自动联网
/// 测试结果不包含任何敏感信息
#[tauri::command]
pub async fn test_tmdb_connection(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, TmdbSearchState>,
) -> Result<TmdbConfigStatus, String> {
    let dir = config_dir(&app_handle);
    let api_key_status = do_get_tmdb_api_key_status(&dir)?;
    let gate_enabled = state.is_gate_enabled();

    // Pre-checks
    if !api_key_status.configured {
        return Ok(TmdbConfigStatus {
            api_key_configured: false,
            gate_enabled,
            status: TmdbStatusLevel::NotConfigured,
            message: "无法测试：TMDb API key 未配置。".to_string(),
        });
    }

    if !gate_enabled {
        return Ok(TmdbConfigStatus {
            api_key_configured: true,
            gate_enabled: false,
            status: TmdbStatusLevel::GateDisabled,
            message: "无法测试：TMDb live search 已禁用。".to_string(),
        });
    }

    // Execute test search with minimal query
    let service = crate::tmdb_search::service::LiveSearchService::new(
        state.api_key_provider.clone(),
        state.gate.clone(),
    );

    let test_input = crate::tmdb_search_contract::SearchTmdbCandidatesInput {
        query: "test".to_string(),
        media_type: crate::tmdb_search_contract::TmdbSearchMediaType::Movie,
        language: "en".to_string(),
        year: None,
        page: Some(1),
    };

    let output = service.search(test_input).await;

    if let Some(error) = output.error {
        let message = match error.code {
            crate::tmdb_search_contract::TmdbSearchErrorCode::ApiKeyInvalid => {
                "API key 无效。请检查您的 TMDb API key。".to_string()
            }
            crate::tmdb_search_contract::TmdbSearchErrorCode::RateLimited => {
                "TMDb 请求频率限制。请稍后再试。".to_string()
            }
            crate::tmdb_search_contract::TmdbSearchErrorCode::NetworkError => {
                "网络连接失败。请检查您的网络连接。".to_string()
            }
            _ => {
                format!("连接测试失败：{}", error.message)
            }
        };

        Ok(TmdbConfigStatus {
            api_key_configured: true,
            gate_enabled: true,
            status: TmdbStatusLevel::ConnectionFailed,
            message,
        })
    } else {
        Ok(TmdbConfigStatus {
            api_key_configured: true,
            gate_enabled: true,
            status: TmdbStatusLevel::Connected,
            message: "TMDb 连接测试成功！".to_string(),
        })
    }
}

// === Internal helpers (testable without AppHandle) ===

/// 内部：获取 TMDb 配置状态（用于测试）
#[allow(dead_code)]
fn do_get_tmdb_config_status(
    config_dir: &std::path::Path,
    gate_enabled: bool,
) -> Result<TmdbConfigStatus, String> {
    let api_key_status = do_get_tmdb_api_key_status(config_dir)?;

    let (status, message) = if !api_key_status.configured {
        (
            TmdbStatusLevel::NotConfigured,
            "TMDb API key 未配置。".to_string(),
        )
    } else if !gate_enabled {
        (
            TmdbStatusLevel::GateDisabled,
            "TMDb live search 已禁用。".to_string(),
        )
    } else {
        (
            TmdbStatusLevel::Ready,
            "TMDb 已就绪。".to_string(),
        )
    };

    Ok(TmdbConfigStatus {
        api_key_configured: api_key_status.configured,
        gate_enabled,
        status,
        message,
    })
}
