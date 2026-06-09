// src-tauri/src/tmdb_search/request/search_url.rs
// Search URL Builder
// 职责：构建完整的搜索 URL（仅路径 + 查询参数，不含 base URL）

use crate::tmdb_search::request::search_query::SearchQuery;

/// 构建搜索 URL（路径 + 查询参数）
///
/// 功能：
/// - 将 SearchQuery 转换为 URL 字符串
/// - 只包含路径和查询参数
/// - 不包含 base URL
/// - 不包含 api_key/apiKey/token/bearer/authorization/session
/// - 不出现 http/https
pub fn build_search_url(query: &SearchQuery) -> String {
    let params_str = query
        .params
        .iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    format!("{}?{}", query.path, params_str)
}

/// 简单的 URL 编码
/// 只编码必要的字符：空格、&、=、?、#
fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '&' => "%26".to_string(),
            '=' => "%3D".to_string(),
            '?' => "%3F".to_string(),
            '#' => "%23".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
