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
}

/// TMDb API Key 状态响应
#[derive(Debug, Clone, serde::Serialize)]
pub struct TmdbApiKeyStatus {
    pub configured: bool,
}

/// 获取 TMDb API Key 状态（不返回明文）
#[tauri::command]
pub fn get_tmdb_api_key_status(app_handle: tauri::AppHandle) -> Result<TmdbApiKeyStatus, String> {
    let dir = config_dir(&app_handle);
    let key = api_key_store::get_api_key("tmdb", &dir).map_err(|e| e.to_string())?;
    Ok(TmdbApiKeyStatus {
        configured: key.is_some(),
    })
}

/// 设置 TMDb API Key
#[tauri::command]
pub fn set_tmdb_api_key(
    app_handle: tauri::AppHandle,
    api_key: String,
) -> Result<TmdbApiKeyStatus, String> {
    let trimmed = api_key.trim();
    if trimmed.is_empty() {
        return Err("API key cannot be empty".to_string());
    }
    let dir = config_dir(&app_handle);
    api_key_store::save_api_key("tmdb", trimmed, &dir).map_err(|e| e.to_string())?;
    Ok(TmdbApiKeyStatus { configured: true })
}

/// 清除 TMDb API Key
#[tauri::command]
pub fn clear_tmdb_api_key(app_handle: tauri::AppHandle) -> Result<TmdbApiKeyStatus, String> {
    let dir = config_dir(&app_handle);
    api_key_store::delete_api_key("tmdb", &dir).map_err(|e| e.to_string())?;
    Ok(TmdbApiKeyStatus { configured: false })
}
