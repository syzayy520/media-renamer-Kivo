// src-tauri/src/tmdb_search/transport/live_http.rs
// Live HTTP Transport
// 职责：实现真实的 TMDb API HTTP 调用

use crate::tmdb_search::transport::transport_gate::TransportGate;
use crate::tmdb_search::transport::TmdbTransport;
use crate::tmdb_search_contract::{TmdbSearchError, TmdbSearchErrorCode};
use reqwest::Client;
use std::future::Future;
use std::sync::Arc;

/// TMDb API 基础 URL
const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

/// Live HTTP Transport 实现
///
/// 使用 reqwest 进行真实的 TMDb API 调用
/// 必须通过 TransportGate 控制启用状态
pub struct LiveHttpTransport {
    /// HTTP 客户端
    client: Client,
    /// API key
    api_key: String,
    /// Transport gate
    gate: Arc<TransportGate>,
}

impl LiveHttpTransport {
    /// 创建新的 Live HTTP Transport
    ///
    /// # Arguments
    /// * `api_key` - TMDb API key
    /// * `gate` - Transport gate 控制启用状态
    pub fn new(api_key: String, gate: Arc<TransportGate>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            gate,
        }
    }

    /// 构建完整的请求 URL（用于测试和调试）
    ///
    /// # Arguments
    /// * `path` - API 路径
    /// * `params` - 查询参数
    ///
    /// # Returns
    /// 完整的 URL 字符串（api_key 被隐藏）
    fn build_url_for_debug(&self, path: &str, params: &[(String, String)]) -> String {
        let mut url = format!("{}{}", TMDB_BASE_URL, path);
        url.push_str("?api_key=***");
        for (key, value) in params {
            url.push('&');
            url.push_str(key);
            url.push('=');
            url.push_str(value);
        }
        url
    }
}

impl TmdbTransport for LiveHttpTransport {
    fn send_search_request(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> impl Future<Output = Result<String, TmdbSearchError>> + Send {
        let gate_enabled = self.gate.is_enabled();
        let url = format!("{}{}", TMDB_BASE_URL, path);
        let api_key = self.api_key.clone();
        let params: Vec<(String, String)> = params.to_vec();
        let client = self.client.clone();

        async move {
            // 检查 gate 是否启用
            if !gate_enabled {
                return Err(TmdbSearchError {
                    code: TmdbSearchErrorCode::Unknown,
                    message: "Live HTTP transport is disabled. Enable transport gate first."
                        .to_string(),
                    retryable: false,
                    retry_after: None,
                });
            }

            let mut request = client.get(&url).query(&[("api_key", &api_key)]);
            for (key, value) in &params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }

            let response = request.send().await.map_err(|e| TmdbSearchError {
                code: TmdbSearchErrorCode::NetworkError,
                message: format!("HTTP request failed: {}", e),
                retryable: true,
                retry_after: None,
            })?;

            let status = response.status();
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let retry_after = response
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());
                return Err(TmdbSearchError {
                    code: TmdbSearchErrorCode::RateLimited,
                    message: "Rate limited by TMDb API".to_string(),
                    retryable: true,
                    retry_after,
                });
            }

            if !status.is_success() {
                return Err(TmdbSearchError {
                    code: TmdbSearchErrorCode::NetworkError,
                    message: format!("HTTP error: {}", status),
                    retryable: false,
                    retry_after: None,
                });
            }

            response.text().await.map_err(|e| TmdbSearchError {
                code: TmdbSearchErrorCode::NetworkError,
                message: format!("Failed to read response: {}", e),
                retryable: true,
                retry_after: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gate_disabled_returns_error() {
        let gate = Arc::new(TransportGate::new());
        let transport = LiveHttpTransport::new("test_key".to_string(), gate);

        let params = vec![("query".to_string(), "test".to_string())];
        let result = transport
            .send_search_request("/search/movie", &params)
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, TmdbSearchErrorCode::Unknown);
        assert!(err.message.contains("disabled"));
    }

    #[tokio::test]
    #[ignore] // 需要真实网络和有效 API key
    async fn test_live_search_with_valid_key() {
        // 这个测试需要设置环境变量 TMDB_API_KEY
        let api_key = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY not set");
        let gate = Arc::new(TransportGate::enabled());
        let transport = LiveHttpTransport::new(api_key, gate);

        let params = vec![
            ("query".to_string(), "Inception".to_string()),
            ("language".to_string(), "en".to_string()),
        ];
        let result = transport
            .send_search_request("/search/movie", &params)
            .await;
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("results"));
    }

    #[test]
    fn test_build_url_for_debug() {
        let gate = Arc::new(TransportGate::new());
        let transport = LiveHttpTransport::new("test_key".to_string(), gate);

        let params = vec![
            ("query".to_string(), "test movie".to_string()),
            ("language".to_string(), "en".to_string()),
        ];
        let url = transport.build_url_for_debug("/search/movie", &params);
        assert!(url.contains("api_key=***"));
        assert!(url.contains("query=test movie"));
        assert!(url.contains("language=en"));
    }

    #[test]
    fn test_api_key_hidden_in_debug() {
        let gate = Arc::new(TransportGate::new());
        let transport = LiveHttpTransport::new("secret_key_123".to_string(), gate);

        let params = vec![("query".to_string(), "test".to_string())];
        let url = transport.build_url_for_debug("/search/movie", &params);

        // 验证真实 key 不在 debug URL 中
        assert!(!url.contains("secret_key_123"));
        assert!(url.contains("api_key=***"));
    }
}
