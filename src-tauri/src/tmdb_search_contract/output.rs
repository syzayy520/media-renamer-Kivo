// src-tauri/src/tmdb_search_contract/output.rs
// TMDb 搜索输出类型
// 职责：定义 SearchTmdbCandidatesOutput 结构体

use crate::tmdb_search_contract::candidate::{TmdbCandidate, TmdbCandidateSource};
use crate::tmdb_search_contract::error::TmdbSearchError;
use crate::tmdb_search_contract::rate_limit::TmdbRateLimitState;
use serde::{Deserialize, Serialize};

/// TMDb 搜索输出
/// 空结果必须可表示为 candidates = []
/// error 可选，不能把空结果当 error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTmdbCandidatesOutput {
    /// 候选列表
    pub candidates: Vec<TmdbCandidate>,

    /// 候选来源
    pub source: TmdbCandidateSource,

    /// 速率限制状态（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<TmdbRateLimitState>,

    /// 错误（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TmdbSearchError>,
}
