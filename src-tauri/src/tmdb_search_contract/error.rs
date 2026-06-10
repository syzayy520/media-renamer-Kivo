// src-tauri/src/tmdb_search_contract/error.rs
// TMDb 搜索错误类型
// 职责：定义 TmdbSearchError 和 TmdbSearchErrorCode

use serde::{Deserialize, Serialize};

/// TMDb 搜索错误代码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TmdbSearchErrorCode {
    /// API key 缺失
    #[serde(rename = "apikey_missing")]
    ApiKeyMissing,
    
    /// API key 无效
    #[serde(rename = "apikey_invalid")]
    ApiKeyInvalid,
    
    /// 速率限制
    #[serde(rename = "rate_limited")]
    RateLimited,
    
    /// 超时
    #[serde(rename = "timeout")]
    Timeout,
    
    /// 网络错误
    #[serde(rename = "network_error")]
    NetworkError,
    
    /// 未知错误
    #[serde(rename = "unknown")]
    Unknown,
}

/// TMDb 搜索错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbSearchError {
    /// 错误代码
    pub code: TmdbSearchErrorCode,

    /// 错误消息
    pub message: String,

    /// 是否可重试
    pub retryable: bool,

    /// 重试等待时间（秒，可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}
