// src-tauri/src/tmdb_search_contract/media_type.rs
// TMDb 搜索媒体类型枚举
// 职责：定义 TmdbSearchMediaType 枚举

use serde::{Deserialize, Serialize};

/// TMDb 搜索媒体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TmdbSearchMediaType {
    /// 电影
    #[serde(rename = "movie")]
    Movie,

    /// 电视剧
    #[serde(rename = "tv")]
    Tv,
}
