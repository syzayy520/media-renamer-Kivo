// src-tauri/src/tmdb_search/mapping/rate_limit_mapper.rs
// Rate Limit Mapper
// 职责：将 HTTP 速率限制头部映射为 TmdbRateLimitState

use crate::tmdb_search_contract::TmdbRateLimitState;

/// HTTP 速率限制响应信息
/// 不包含实际 HTTP 调用，仅用于映射
#[derive(Debug, Clone)]
pub struct HttpRateLimitResponse {
    /// 剩余请求次数
    pub remaining: u32,

    /// 限制重置时间戳（Unix 秒）
    pub reset_at: u64,

    /// 总限制次数（可选）
    pub limit: Option<u32>,
}

/// 将 HTTP 速率限制响应映射为 TmdbRateLimitState
///
/// 简单的值映射，不进行额外计算
pub fn map_rate_limit(response: &HttpRateLimitResponse) -> TmdbRateLimitState {
    TmdbRateLimitState {
        remaining: response.remaining,
        reset_at: response.reset_at,
        limit: response.limit,
    }
}

/// 检查是否达到速率限制
pub fn is_rate_limited(state: &TmdbRateLimitState) -> bool {
    state.remaining == 0
}

/// 计算距离重置的秒数
/// 如果当前时间超过重置时间，返回 0
pub fn seconds_until_reset(state: &TmdbRateLimitState, current_timestamp: u64) -> u64 {
    state.reset_at.saturating_sub(current_timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_rate_limit_basic() {
        let response = HttpRateLimitResponse {
            remaining: 35,
            reset_at: 1234567890,
            limit: Some(40),
        };
        let state = map_rate_limit(&response);
        assert_eq!(state.remaining, 35);
        assert_eq!(state.reset_at, 1234567890);
        assert_eq!(state.limit, Some(40));
    }

    #[test]
    fn test_map_rate_limit_no_limit() {
        let response = HttpRateLimitResponse {
            remaining: 10,
            reset_at: 1234567890,
            limit: None,
        };
        let state = map_rate_limit(&response);
        assert_eq!(state.limit, None);
    }

    #[test]
    fn test_is_rate_limited_true() {
        let state = TmdbRateLimitState {
            remaining: 0,
            reset_at: 1234567890,
            limit: Some(40),
        };
        assert!(is_rate_limited(&state));
    }

    #[test]
    fn test_is_rate_limited_false() {
        let state = TmdbRateLimitState {
            remaining: 1,
            reset_at: 1234567890,
            limit: Some(40),
        };
        assert!(!is_rate_limited(&state));
    }

    #[test]
    fn test_seconds_until_reset_future() {
        let state = TmdbRateLimitState {
            remaining: 10,
            reset_at: 1000,
            limit: None,
        };
        assert_eq!(seconds_until_reset(&state, 900), 100);
    }

    #[test]
    fn test_seconds_until_reset_past() {
        let state = TmdbRateLimitState {
            remaining: 10,
            reset_at: 1000,
            limit: None,
        };
        assert_eq!(seconds_until_reset(&state, 1100), 0);
    }

    #[test]
    fn test_seconds_until_reset_exact() {
        let state = TmdbRateLimitState {
            remaining: 10,
            reset_at: 1000,
            limit: None,
        };
        assert_eq!(seconds_until_reset(&state, 1000), 0);
    }

    #[test]
    fn test_no_api_key_in_rate_limit_json() {
        let response = HttpRateLimitResponse {
            remaining: 10,
            reset_at: 1234567890,
            limit: Some(40),
        };
        let state = map_rate_limit(&response);
        let json = serde_json::to_string(&state).unwrap();
        assert!(!json.contains("api_key"));
        assert!(!json.contains("apiKey"));
        assert!(!json.contains("token"));
    }

    #[test]
    fn test_no_network_dependency() {
        let response = HttpRateLimitResponse {
            remaining: 10,
            reset_at: 1234567890,
            limit: None,
        };
        let _ = map_rate_limit(&response);
    }
}
