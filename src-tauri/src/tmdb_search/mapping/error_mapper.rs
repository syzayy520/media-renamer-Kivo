// src-tauri/src/tmdb_search/mapping/error_mapper.rs
// Error Mapper
// 职责：将 HTTP 错误响应映射为 TmdbSearchError

use crate::tmdb_search_contract::{TmdbSearchError, TmdbSearchErrorCode};

/// HTTP 错误响应信息
/// 不包含实际 HTTP 调用，仅用于映射
#[derive(Debug, Clone)]
pub struct HttpErrorResponse {
    /// HTTP 状态码
    pub status_code: u16,

    /// 错误消息
    pub message: String,

    /// Retry-After 头部（秒）
    pub retry_after: Option<u64>,
}

/// 将 HTTP 错误响应映射为 TmdbSearchError
///
/// 映射规则：
/// - 401 -> ApiKeyInvalid
/// - 404 -> Unknown (资源未找到)
/// - 429 -> RateLimited
/// - 500-599 -> NetworkError (服务器错误)
/// - 其他 -> Unknown
pub fn map_http_error(response: &HttpErrorResponse) -> TmdbSearchError {
    let code = match response.status_code {
        401 => TmdbSearchErrorCode::ApiKeyInvalid,
        404 => TmdbSearchErrorCode::Unknown,
        429 => TmdbSearchErrorCode::RateLimited,
        500..=599 => TmdbSearchErrorCode::NetworkError,
        _ => TmdbSearchErrorCode::Unknown,
    };

    let retryable = matches!(response.status_code, 429 | 500..=599);

    TmdbSearchError {
        code,
        message: response.message.clone(),
        retryable,
        retry_after: response.retry_after,
    }
}

/// 创建 API key 缺失错误
pub fn api_key_missing_error() -> TmdbSearchError {
    TmdbSearchError {
        code: TmdbSearchErrorCode::ApiKeyMissing,
        message: "API key is missing".to_string(),
        retryable: false,
        retry_after: None,
    }
}

/// 创建超时错误
pub fn timeout_error() -> TmdbSearchError {
    TmdbSearchError {
        code: TmdbSearchErrorCode::Timeout,
        message: "Request timed out".to_string(),
        retryable: true,
        retry_after: None,
    }
}

/// 创建网络连接错误
pub fn network_connection_error(message: &str) -> TmdbSearchError {
    TmdbSearchError {
        code: TmdbSearchErrorCode::NetworkError,
        message: message.to_string(),
        retryable: true,
        retry_after: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_401_maps_to_api_key_invalid() {
        let response = HttpErrorResponse {
            status_code: 401,
            message: "Unauthorized".to_string(),
            retry_after: None,
        };
        let error = map_http_error(&response);
        assert_eq!(error.code, TmdbSearchErrorCode::ApiKeyInvalid);
        assert!(!error.retryable);
    }

    #[test]
    fn test_429_maps_to_rate_limited() {
        let response = HttpErrorResponse {
            status_code: 429,
            message: "Too Many Requests".to_string(),
            retry_after: Some(30),
        };
        let error = map_http_error(&response);
        assert_eq!(error.code, TmdbSearchErrorCode::RateLimited);
        assert!(error.retryable);
        assert_eq!(error.retry_after, Some(30));
    }

    #[test]
    fn test_500_maps_to_network_error() {
        let response = HttpErrorResponse {
            status_code: 500,
            message: "Internal Server Error".to_string(),
            retry_after: None,
        };
        let error = map_http_error(&response);
        assert_eq!(error.code, TmdbSearchErrorCode::NetworkError);
        assert!(error.retryable);
    }

    #[test]
    fn test_404_maps_to_unknown() {
        let response = HttpErrorResponse {
            status_code: 404,
            message: "Not Found".to_string(),
            retry_after: None,
        };
        let error = map_http_error(&response);
        assert_eq!(error.code, TmdbSearchErrorCode::Unknown);
        assert!(!error.retryable);
    }

    #[test]
    fn test_unknown_status_code() {
        let response = HttpErrorResponse {
            status_code: 418,
            message: "I'm a teapot".to_string(),
            retry_after: None,
        };
        let error = map_http_error(&response);
        assert_eq!(error.code, TmdbSearchErrorCode::Unknown);
        assert!(!error.retryable);
    }

    #[test]
    fn test_api_key_missing_helper() {
        let error = api_key_missing_error();
        assert_eq!(error.code, TmdbSearchErrorCode::ApiKeyMissing);
        assert_eq!(error.message, "API key is missing");
        assert!(!error.retryable);
        assert_eq!(error.retry_after, None);
    }

    #[test]
    fn test_timeout_helper() {
        let error = timeout_error();
        assert_eq!(error.code, TmdbSearchErrorCode::Timeout);
        assert_eq!(error.message, "Request timed out");
        assert!(error.retryable);
    }

    #[test]
    fn test_network_connection_error_helper() {
        let error = network_connection_error("Connection refused");
        assert_eq!(error.code, TmdbSearchErrorCode::NetworkError);
        assert_eq!(error.message, "Connection refused");
        assert!(error.retryable);
    }

    #[test]
    fn test_no_api_key_in_error_json() {
        let response = HttpErrorResponse {
            status_code: 401,
            message: "Unauthorized".to_string(),
            retry_after: None,
        };
        let error = map_http_error(&response);
        let json = serde_json::to_string(&error).unwrap();
        assert!(!json.contains("api_key"));
        assert!(!json.contains("apiKey"));
    }

    #[test]
    fn test_no_network_dependency() {
        let response = HttpErrorResponse {
            status_code: 500,
            message: "Error".to_string(),
            retry_after: None,
        };
        let _ = map_http_error(&response);
    }
}
