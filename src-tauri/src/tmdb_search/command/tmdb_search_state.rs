// src-tauri/src/tmdb_search/command/tmdb_search_state.rs
// TMDb Search State
// 职责：管理 TMDb 搜索的 Tauri 状态，包含 transport gate 和 API key provider

use std::sync::Arc;

use crate::tmdb_search::api_key::ConfigApiKeyProvider;
use crate::tmdb_search::transport::TransportGate;

/// TMDb 搜索状态
///
/// 包含 TMDb 搜索所需的依赖项，通过 Tauri 状态管理注入到命令中。
///
/// # 字段
/// * `gate` - Transport gate 控制 HTTP 启用状态
/// * `api_key_provider` - API key 提供者
pub struct TmdbSearchState {
    pub gate: Arc<TransportGate>,
    pub api_key_provider: ConfigApiKeyProvider,
}

impl TmdbSearchState {
    /// 创建新的 TMDb 搜索状态
    ///
    /// # Arguments
    /// * `config_dir` - 配置目录路径
    pub fn new(config_dir: std::path::PathBuf) -> Self {
        Self {
            gate: Arc::new(TransportGate::new()), // disabled by default
            api_key_provider: ConfigApiKeyProvider::new(config_dir),
        }
    }

    /// 创建已启用的 TMDb 搜索状态（用于测试）
    pub fn enabled(config_dir: std::path::PathBuf) -> Self {
        Self {
            gate: Arc::new(TransportGate::enabled()),
            api_key_provider: ConfigApiKeyProvider::new(config_dir),
        }
    }

    /// 启用 transport gate
    pub fn enable_gate(&self) {
        self.gate.enable();
    }

    /// 禁用 transport gate
    pub fn disable_gate(&self) {
        self.gate.disable();
    }

    /// 检查 gate 是否启用
    pub fn is_gate_enabled(&self) -> bool {
        self.gate.is_enabled()
    }
}

// 实现 Debug 但不暴露敏感信息
impl std::fmt::Debug for TmdbSearchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TmdbSearchState")
            .field("gate_enabled", &self.gate.is_enabled())
            .field("config_dir", &self.api_key_provider.config_dir())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_state_default_gate_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let state = TmdbSearchState::new(temp_dir.path().to_path_buf());

        assert!(!state.is_gate_enabled());
    }

    #[test]
    fn test_state_enabled_gate() {
        let temp_dir = TempDir::new().unwrap();
        let state = TmdbSearchState::enabled(temp_dir.path().to_path_buf());

        assert!(state.is_gate_enabled());
    }

    #[test]
    fn test_state_enable_disable_gate() {
        let temp_dir = TempDir::new().unwrap();
        let state = TmdbSearchState::new(temp_dir.path().to_path_buf());

        assert!(!state.is_gate_enabled());

        state.enable_gate();
        assert!(state.is_gate_enabled());

        state.disable_gate();
        assert!(!state.is_gate_enabled());
    }

    #[test]
    fn test_debug_no_sensitive_info() {
        let temp_dir = TempDir::new().unwrap();
        let state = TmdbSearchState::new(temp_dir.path().to_path_buf());

        let debug_output = format!("{:?}", state);
        assert!(!debug_output.contains("api_key"));
        assert!(!debug_output.contains("secret"));
    }
}
