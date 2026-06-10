// src-tauri/src/tmdb_search/api_key/api_key_state.rs
// API Key 状态
// 职责：定义 API key 的状态枚举，不包含真实 key 值

use serde::{Deserialize, Serialize};

/// API Key 提供状态
///
/// 表示 API key 的可用性状态，不包含真实 key 值。
/// 用于在不暴露 key 的前提下传递状态信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiKeyState {
    /// API key 可用（不包含真实值）
    Available,
    /// API key 缺失
    Missing,
    /// API key 无效（已被拒绝）
    Invalid,
}

impl ApiKeyState {
    /// 检查 key 是否可用
    pub fn is_available(&self) -> bool {
        matches!(self, ApiKeyState::Available)
    }

    /// 检查 key 是否缺失
    pub fn is_missing(&self) -> bool {
        matches!(self, ApiKeyState::Missing)
    }

    /// 检查 key 是否无效
    pub fn is_invalid(&self) -> bool {
        matches!(self, ApiKeyState::Invalid)
    }
}

impl std::fmt::Display for ApiKeyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiKeyState::Available => write!(f, "available"),
            ApiKeyState::Missing => write!(f, "missing"),
            ApiKeyState::Invalid => write!(f, "invalid"),
        }
    }
}
