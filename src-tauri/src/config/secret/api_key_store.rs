// api_key_store 模块 - API Key 安全存储
// 职责：API Key 的 CRUD 操作（本地结构，不联网，不硬编码 key）
// 存储：使用 JSON 文件持久化（非 config.toml，避免意外提交）

use crate::shared::result_types::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// API Key 存储结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ApiKeyStore {
    keys: HashMap<String, String>,
}

/// 获取指定服务的 API Key
pub fn get_api_key(service: &str, config_dir: &Path) -> AppResult<Option<String>> {
    let store = load_store(config_dir)?;
    Ok(store.keys.get(service).cloned())
}

/// 保存指定服务的 API Key
pub fn save_api_key(service: &str, api_key: &str, config_dir: &Path) -> AppResult<()> {
    let mut store = load_store(config_dir)?;
    store.keys.insert(service.to_string(), api_key.to_string());
    save_store(&store, config_dir)
}

/// 删除指定服务的 API Key
pub fn delete_api_key(service: &str, config_dir: &Path) -> AppResult<()> {
    let mut store = load_store(config_dir)?;
    store.keys.remove(service);
    save_store(&store, config_dir)
}

/// 脱敏显示 API Key（保留前 4 位和后 4 位，中间用 * 替代）
pub fn mask_api_key(api_key: &str) -> String {
    let len = api_key.len();
    if len <= 8 {
        return "*".repeat(len);
    }
    let prefix = &api_key[..4];
    let suffix = &api_key[len - 4..];
    let masked_len = len - 8;
    format!("{}{}{}", prefix, "*".repeat(masked_len), suffix)
}

/// 加载 API Key 存储文件
fn load_store(config_dir: &Path) -> AppResult<ApiKeyStore> {
    let store_path = config_dir.join(".api_keys.json");
    if !store_path.exists() {
        return Ok(ApiKeyStore::default());
    }
    let content = std::fs::read_to_string(&store_path)
        .map_err(|e| AppError::Config(format!("Read API key store error: {}", e)))?;
    serde_json::from_str(&content)
        .map_err(|e| AppError::Config(format!("Parse API key store error: {}", e)))
}

/// 保存 API Key 存储文件
fn save_store(store: &ApiKeyStore, config_dir: &Path) -> AppResult<()> {
    let store_path = config_dir.join(".api_keys.json");
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| AppError::Config(format!("Serialize API key store error: {}", e)))?;
    std::fs::write(&store_path, content)
        .map_err(|e| AppError::Config(format!("Write API key store error: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_dir() -> PathBuf {
        tempfile::tempdir().unwrap().keep()
    }

    #[test]
    fn test_get_nonexistent_key() {
        let dir = temp_dir();
        let result = get_api_key("tmdb", &dir).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_get_key() {
        let dir = temp_dir();
        save_api_key("tmdb", "abc12345defghijk", &dir).unwrap();
        let key = get_api_key("tmdb", &dir).unwrap();
        assert_eq!(key.unwrap(), "abc12345defghijk");
    }

    #[test]
    fn test_delete_key() {
        let dir = temp_dir();
        save_api_key("tmdb", "abc12345defghijk", &dir).unwrap();
        delete_api_key("tmdb", &dir).unwrap();
        let result = get_api_key("tmdb", &dir).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_multiple_services() {
        let dir = temp_dir();
        save_api_key("tmdb", "tmdb_key_1234", &dir).unwrap();
        save_api_key("omdb", "omdb_key_5678", &dir).unwrap();
        assert_eq!(get_api_key("tmdb", &dir).unwrap().unwrap(), "tmdb_key_1234");
        assert_eq!(get_api_key("omdb", &dir).unwrap().unwrap(), "omdb_key_5678");
    }

    #[test]
    fn test_mask_api_key_long() {
        // 16 chars: 4 prefix + 8 masked + 4 suffix
        assert_eq!(mask_api_key("abcdefghijklmnop"), "abcd********mnop");
    }

    #[test]
    fn test_mask_api_key_short() {
        assert_eq!(mask_api_key("abc"), "***");
    }

    #[test]
    fn test_mask_api_key_exactly_eight() {
        assert_eq!(mask_api_key("abcdefgh"), "********");
    }
}
