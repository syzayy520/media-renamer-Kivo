// src-tauri/src/tmdb_search_contract/rate_limit.rs
// TMDb 速率限制状态
// 职责：定义 TmdbRateLimitState 结构体

use serde::{Deserialize, Serialize};

/// TMDb API 速率限制状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbRateLimitState {
    /// 剩余请求次数
    pub remaining: u32,

    /// 限制重置时间戳（Unix 秒）
    pub reset_at: u64,

    /// 总限制次数（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
