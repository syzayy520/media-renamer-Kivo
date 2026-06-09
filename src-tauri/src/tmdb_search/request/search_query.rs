// src-tauri/src/tmdb_search/request/search_query.rs
// Search Query Builder
// 职责：构建 TMDb 搜索查询参数

use crate::tmdb_search::validation::search_input_validator::ValidatedTmdbSearchInput;
use crate::tmdb_search_contract::TmdbSearchMediaType;

/// TMDb 搜索查询参数
/// 不包含 API key，不包含 base URL
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// 搜索路径（如 /search/movie）
    pub path: String,

    /// 查询参数列表
    pub params: Vec<(String, String)>,
}

/// 构建搜索查询参数
///
/// 功能：
/// - Movie -> /search/movie
/// - Tv -> /search/tv
/// - query params: query, language, page, year (如果 Some)
/// - 不包含 api_key/apiKey/token/bearer/authorization/session
/// - 不拼接 base URL
/// - 不出现 http/https
pub fn build_search_query(input: &ValidatedTmdbSearchInput) -> SearchQuery {
    let path = match input.media_type {
        TmdbSearchMediaType::Movie => "/search/movie".to_string(),
        TmdbSearchMediaType::Tv => "/search/tv".to_string(),
    };

    let mut params = vec![
        ("query".to_string(), input.query.clone()),
        ("language".to_string(), input.language.clone()),
        ("page".to_string(), input.page.to_string()),
    ];

    // year 只在 Some 时添加
    if let Some(year) = input.year {
        params.push(("year".to_string(), year.to_string()));
    }

    SearchQuery { path, params }
}
