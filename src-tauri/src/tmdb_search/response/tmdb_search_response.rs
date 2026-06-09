// src-tauri/src/tmdb_search/response/tmdb_search_response.rs
// TMDb Search Response DTO
// 职责：定义 TMDb 搜索响应结构

use serde::{Deserialize, Serialize};
use crate::tmdb_search::response::tmdb_search_result::TmdbSearchResult;

/// TMDb 搜索响应
/// 只包含必要字段，不包含 raw JSON passthrough
/// 不包含 api_key/apiKey/token/account/session/auth/authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbSearchResponse {
    /// 当前页码
    pub page: u32,

    /// 总页数
    pub total_pages: u32,

    /// 总结果数
    pub total_results: u32,

    /// 搜索结果列表
    pub results: Vec<TmdbSearchResult>,
}
