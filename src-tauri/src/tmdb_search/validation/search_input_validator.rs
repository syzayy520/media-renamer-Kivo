// src-tauri/src/tmdb_search/validation/search_input_validator.rs
// Search Input Validator
// 职责：验证 TMDb 搜索输入，返回验证后的结构

use crate::tmdb_search_contract::{
    SearchTmdbCandidatesInput, TmdbSearchError, TmdbSearchErrorCode, TmdbSearchMediaType,
};

/// 验证后的 TMDb 搜索输入
/// 所有字段已验证，page 和 year 已规范化
#[derive(Debug, Clone)]
pub struct ValidatedTmdbSearchInput {
    /// 搜索查询字符串（已 trim）
    pub query: String,

    /// 媒体类型
    pub media_type: TmdbSearchMediaType,

    /// 语言代码（已 trim）
    pub language: String,

    /// 发布年份（已验证非零）
    pub year: Option<u16>,

    /// 页码（从 1 开始，默认 1）
    pub page: u16,
}

/// 验证 TMDb 搜索输入
///
/// 验证规则：
/// 1. query trim 后不能为空
/// 2. language trim 后不能为空
/// 3. page 为 None 时默认 1
/// 4. page 为 Some(0) 返回结构化错误
/// 5. year 为 Some(0) 返回结构化错误
/// 6. 不验证 API key
/// 7. 不读取环境变量
/// 8. 不读取文件系统
/// 9. 不调用网络
pub fn validate_search_input(
    input: &SearchTmdbCandidatesInput,
) -> Result<ValidatedTmdbSearchInput, TmdbSearchError> {
    // 验证 query
    let query = input.query.trim();
    if query.is_empty() {
        return Err(TmdbSearchError {
            code: TmdbSearchErrorCode::Unknown,
            message: "Search query cannot be empty".to_string(),
            retryable: false,
            retry_after: None,
        });
    }

    // 验证 language
    let language = input.language.trim();
    if language.is_empty() {
        return Err(TmdbSearchError {
            code: TmdbSearchErrorCode::Unknown,
            message: "Language cannot be empty".to_string(),
            retryable: false,
            retry_after: None,
        });
    }

    // 验证 page
    let page = match input.page {
        Some(0) => {
            return Err(TmdbSearchError {
                code: TmdbSearchErrorCode::Unknown,
                message: "Page number must be greater than 0".to_string(),
                retryable: false,
                retry_after: None,
            });
        }
        Some(p) => p,
        None => 1,
    };

    // 验证 year
    if let Some(0) = input.year {
        return Err(TmdbSearchError {
            code: TmdbSearchErrorCode::Unknown,
            message: "Year cannot be 0".to_string(),
            retryable: false,
            retry_after: None,
        });
    }

    Ok(ValidatedTmdbSearchInput {
        query: query.to_string(),
        media_type: input.media_type,
        language: language.to_string(),
        year: input.year,
        page,
    })
}
