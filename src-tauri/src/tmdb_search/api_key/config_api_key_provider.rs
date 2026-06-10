// src-tauri/src/tmdb_search/api_key/config_api_key_provider.rs
// Config-based API Key Provider
// 职责：从配置存储读取真实 API key，实现 TmdbApiKeyProvider trait

use std::path::{Path, PathBuf};

use crate::config::secret::api_key_store;
use crate::tmdb_search::api_key::api_key_state::ApiKeyState;
use crate::tmdb_search::api_key::TmdbApiKeyProvider;

/// 基于配置存储的 API Key 提供者
///
/// 从本地配置文件读取 TMDb API key。
/// 实现 TmdbApiKeyProvider trait，提供安全的 key 访问接口。
///
/// # 安全约束
/// - 不将 key 值放入 Debug 输出
/// - 不将 key 值日志打印
/// - 不将 key 值放入 DTO/序列化结构
#[derive(Clone)]
pub struct ConfigApiKeyProvider {
    config_dir: PathBuf,
}

impl ConfigApiKeyProvider {
    /// 创建新的配置 API Key 提供者
    ///
    /// # Arguments
    /// * `config_dir` - 配置目录路径
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    /// 获取配置目录引用
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
}

impl TmdbApiKeyProvider for ConfigApiKeyProvider {
    #[allow(refining_impl_trait)]
    fn get_api_key(&self) -> impl std::future::Future<Output = Option<String>> + Send {
        let config_dir = self.config_dir.clone();
        async move {
            api_key_store::get_api_key("tmdb", &config_dir)
                .ok()
                .flatten()
        }
    }

    #[allow(refining_impl_trait)]
    fn key_state(&self) -> impl std::future::Future<Output = ApiKeyState> + Send {
        let config_dir = self.config_dir.clone();
        async move {
            match api_key_store::get_api_key("tmdb", &config_dir) {
                Ok(Some(_)) => ApiKeyState::Available,
                Ok(None) => ApiKeyState::Missing,
                Err(_) => ApiKeyState::Invalid,
            }
        }
    }
}

// 实现 Debug 但不暴露 key
impl std::fmt::Debug for ConfigApiKeyProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigApiKeyProvider")
            .field("config_dir", &self.config_dir)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_config_provider_missing_key() {
        let temp_dir = TempDir::new().unwrap();
        let provider = ConfigApiKeyProvider::new(temp_dir.path().to_path_buf());

        let state = provider.key_state().await;
        assert_eq!(state, ApiKeyState::Missing);

        let key = provider.get_api_key().await;
        assert!(key.is_none());
    }

    #[tokio::test]
    async fn test_config_provider_available_key() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();

        // Save a test key
        api_key_store::save_api_key("tmdb", "test_key_12345", &config_dir).unwrap();

        let provider = ConfigApiKeyProvider::new(config_dir);
        let state = provider.key_state().await;
        assert_eq!(state, ApiKeyState::Available);

        let key = provider.get_api_key().await;
        assert_eq!(key.unwrap(), "test_key_12345");
    }

    #[tokio::test]
    async fn test_config_provider_multiple_calls() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();

        api_key_store::save_api_key("tmdb", "test_key_67890", &config_dir).unwrap();

        let provider = ConfigApiKeyProvider::new(config_dir);

        // Multiple calls should work
        let key1 = provider.get_api_key().await;
        let key2 = provider.get_api_key().await;
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_debug_no_key_leak() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();

        api_key_store::save_api_key("tmdb", "super_secret_key", &config_dir).unwrap();

        let provider = ConfigApiKeyProvider::new(config_dir);
        let debug_output = format!("{:?}", provider);

        // Debug output should not contain the key
        assert!(!debug_output.contains("super_secret_key"));
        assert!(debug_output.contains("ConfigApiKeyProvider"));
    }
}
