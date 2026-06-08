// metadata_settings 模块 - TMDb 设置管理
// 职责：TMDb 相关用户设置的结构定义和 CRUD
// 注意：只做本地配置结构，不联网，不实现 TMDb 请求

use crate::config::secret::api_key_store;
use crate::shared::result_types::AppResult;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// TMDb 服务标识
pub const TMDB_SERVICE: &str = "tmdb";

/// TMDb 用户设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TmdbSettings {
    /// 是否已配置 API Key
    pub configured: bool,
    /// API Key 脱敏显示
    pub api_key_masked: Option<String>,
    /// 语言偏好（如 zh-CN, en-US）
    pub language: String,
    /// 地区偏好
    pub region: String,
}

/// 获取 TMDb 设置
pub fn get_tmdb_settings(config_dir: &Path) -> AppResult<TmdbSettings> {
    let api_key = api_key_store::get_api_key(TMDB_SERVICE, config_dir)?;
    let configured = api_key.is_some();
    let api_key_masked = api_key.map(|k| api_key_store::mask_api_key(&k));

    Ok(TmdbSettings {
        configured,
        api_key_masked,
        language: "zh-CN".to_string(),
        region: "CN".to_string(),
    })
}

/// 保存 TMDb API Key
pub fn save_tmdb_api_key(api_key: &str, config_dir: &Path) -> AppResult<()> {
    api_key_store::save_api_key(TMDB_SERVICE, api_key, config_dir)
}

/// 删除 TMDb API Key
pub fn delete_tmdb_api_key(config_dir: &Path) -> AppResult<()> {
    api_key_store::delete_api_key(TMDB_SERVICE, config_dir)
}

/// 检查 TMDb 是否已配置
pub fn is_tmdb_configured(config_dir: &Path) -> AppResult<bool> {
    let key = api_key_store::get_api_key(TMDB_SERVICE, config_dir)?;
    Ok(key.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_dir() -> PathBuf {
        tempfile::tempdir().unwrap().keep()
    }

    #[test]
    fn test_default_tmdb_settings_not_configured() {
        let dir = temp_dir();
        let settings = get_tmdb_settings(&dir).unwrap();
        assert!(!settings.configured);
        assert!(settings.api_key_masked.is_none());
        assert_eq!(settings.language, "zh-CN");
        assert_eq!(settings.region, "CN");
    }

    #[test]
    fn test_save_and_get_tmdb_settings() {
        let dir = temp_dir();
        save_tmdb_api_key("sk_test_key_12345678", &dir).unwrap();
        let settings = get_tmdb_settings(&dir).unwrap();
        assert!(settings.configured);
        assert!(settings.api_key_masked.is_some());
        let masked = settings.api_key_masked.unwrap();
        assert!(masked.contains('*'));
        assert!(!masked.contains("12345678"));
    }

    #[test]
    fn test_delete_tmdb_api_key() {
        let dir = temp_dir();
        save_tmdb_api_key("test_key", &dir).unwrap();
        assert!(is_tmdb_configured(&dir).unwrap());
        delete_tmdb_api_key(&dir).unwrap();
        assert!(!is_tmdb_configured(&dir).unwrap());
    }

    #[test]
    fn test_is_tmdb_configured_false() {
        let dir = temp_dir();
        assert!(!is_tmdb_configured(&dir).unwrap());
    }
}
