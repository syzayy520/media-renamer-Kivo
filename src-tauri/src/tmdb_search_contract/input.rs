// src-tauri/src/tmdb_search_contract/input.rs
// TMDb 搜索输入类型
// 职责：定义 SearchTmdbCandidatesInput 结构体

use crate::tmdb_search_contract::media_type::TmdbSearchMediaType;
use serde::{Deserialize, Serialize};

/// TMDb 搜索输入
/// 不包含 API key，不包含文件路径，不包含执行 rename 状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTmdbCandidatesInput {
    /// 搜索查询字符串
    pub query: String,

    /// 媒体类型
    pub media_type: TmdbSearchMediaType,

    /// 语言代码（如 "en", "zh-CN"）
    pub language: String,

    /// 发布年份（可选）
    pub year: Option<u16>,

    /// 页码（可选，从 1 开始）
    pub page: Option<u16>,
}
