// src-tauri/src/tmdb_search/response/tmdb_search_result.rs
// TMDb Search Result DTO
// 职责：定义单个搜索结果结构

use serde::{Deserialize, Serialize};

/// TMDb 搜索结果
/// 只包含必要字段，不包含 raw JSON passthrough
/// 不包含 api_key/apiKey/token/account/session/auth/authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbSearchResult {
    /// TMDb ID
    pub id: u32,

    /// 标题（电影）或名称（电视剧）
    #[serde(default)]
    pub title: Option<String>,

    /// 名称（电视剧）或标题（电影）
    #[serde(default)]
    pub name: Option<String>,

    /// 原始标题（电影）
    #[serde(default)]
    pub original_title: Option<String>,

    /// 原始名称（电视剧）
    #[serde(default)]
    pub original_name: Option<String>,

    /// 发布日期（电影，格式：YYYY-MM-DD）
    #[serde(default)]
    pub release_date: Option<String>,

    /// 首播日期（电视剧，格式：YYYY-MM-DD）
    #[serde(default)]
    pub first_air_date: Option<String>,

    /// 概述
    #[serde(default)]
    pub overview: Option<String>,

    /// 海报路径
    #[serde(default)]
    pub poster_path: Option<String>,

    /// 背景图路径
    #[serde(default)]
    pub backdrop_path: Option<String>,

    /// 原始语言
    #[serde(default)]
    pub original_language: Option<String>,

    /// 人气值
    #[serde(default)]
    pub popularity: Option<f64>,

    /// 平均评分
    #[serde(default)]
    pub vote_average: Option<f64>,
}
