// src-tauri/src/tmdb_search_contract/candidate.rs
// TMDb 候选类型
// 职责：定义 TmdbCandidate 和 TmdbCandidateSource

use crate::tmdb_search_contract::media_type::TmdbSearchMediaType;
use serde::{Deserialize, Serialize};

/// TMDb 候选来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TmdbCandidateSource {
    /// 来自 TMDb API
    #[serde(rename = "tmdb")]
    Tmdb,
}

/// TMDb 候选卡片
/// id 是前端候选 ID，不等于 tmdb_id
/// 不透传完整 TMDb 原始 JSON
/// 不包含 API key
/// 不包含原始 HTTP response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbCandidate {
    /// 前端候选 ID（不等于 tmdb_id）
    pub id: String,

    /// TMDb ID
    pub tmdb_id: u64,

    /// 标题
    pub title: String,

    /// 原始标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,

    /// 媒体类型
    pub media_type: TmdbSearchMediaType,

    /// 发布年份（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<u16>,

    /// 概述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,

    /// 海报路径（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,

    /// 背景路径（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<String>,

    /// 语言（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// 热度（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f32>,

    /// 评分（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f32>,

    /// 置信度提示（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_hint: Option<f32>,

    /// 匹配原因
    pub match_reasons: Vec<String>,
}
