// src-tauri/src/tmdb_search/transport/mock.rs
// Mock TMDb Transport
// 职责：提供 TMDb API 传输层的 Mock 实现

use crate::tmdb_search::transport::TmdbTransport;
use crate::tmdb_search_contract::TmdbSearchError;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};

/// Mock TMDb 传输层实现
/// 用于测试，不进行真实 HTTP 调用
///
/// 使用示例：
/// ```rust
/// use app_lib::tmdb_search::MockTmdbTransport;
/// let mock = MockTmdbTransport::new();
/// mock.mock_response_simple("/search/movie", Ok(r#"{"results": []}"#.to_string()));
/// ```
pub struct MockTmdbTransport {
    /// 存储预设的响应
    /// key: 完整的请求路径+参数（如 "/search/movie?query=test&language=en&page=1"）
    /// value: 预设的响应结果
    responses: Arc<Mutex<HashMap<String, Result<String, TmdbSearchError>>>>,

    /// 记录调用历史
    call_history: Arc<Mutex<Vec<String>>>,
}

impl MockTmdbTransport {
    /// 创建新的 Mock 传输层实例
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            call_history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 添加 mock 响应
    ///
    /// # Arguments
    /// * `path` - 请求路径（如 "/search/movie"）
    /// * `params` - 查询参数
    /// * `result` - 预设的响应结果
    pub fn mock_response(
        &self,
        path: &str,
        params: &[(String, String)],
        result: Result<String, TmdbSearchError>,
    ) {
        let key = Self::build_key(path, params);
        let mut responses = self.responses.lock().unwrap();
        responses.insert(key, result);
    }

    /// 添加简单的 mock 响应（仅匹配路径，忽略参数）
    ///
    /// # Arguments
    /// * `path` - 请求路径（如 "/search/movie"）
    /// * `result` - 预设的响应结果
    pub fn mock_response_simple(&self, path: &str, result: Result<String, TmdbSearchError>) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(path.to_string(), result);
    }

    /// 获取调用历史
    pub fn call_history(&self) -> Vec<String> {
        let history = self.call_history.lock().unwrap();
        history.clone()
    }

    /// 获取调用次数
    pub fn call_count(&self) -> usize {
        let history = self.call_history.lock().unwrap();
        history.len()
    }

    /// 清空调用历史
    pub fn clear_history(&self) {
        let mut history = self.call_history.lock().unwrap();
        history.clear();
    }

    /// 构建请求 key
    fn build_key(path: &str, params: &[(String, String)]) -> String {
        let mut key = path.to_string();
        if !params.is_empty() {
            key.push('?');
            let param_str: Vec<String> =
                params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
            key.push_str(&param_str.join("&"));
        }
        key
    }
}

impl Default for MockTmdbTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl TmdbTransport for MockTmdbTransport {
    fn send_search_request(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> impl Future<Output = Result<String, TmdbSearchError>> + Send {
        // 记录调用
        let key = Self::build_key(path, params);
        {
            let mut history = self.call_history.lock().unwrap();
            history.push(key.clone());
        }

        // 查找预设响应
        let responses = self.responses.lock().unwrap();
        let result = responses
            .get(&key)
            .or_else(|| responses.get(path))
            .cloned()
            .unwrap_or_else(|| {
                Err(TmdbSearchError {
                    code: crate::tmdb_search_contract::TmdbSearchErrorCode::Unknown,
                    message: format!("No mock response for: {}", key),
                    retryable: false,
                    retry_after: None,
                })
            });

        async move { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tmdb_search_contract::TmdbSearchErrorCode;

    #[tokio::test]
    async fn test_mock_response_basic() {
        let mock = MockTmdbTransport::new();
        let response_json = r#"{"results": []}"#.to_string();
        mock.mock_response(
            "/search/movie",
            &[
                ("query".to_string(), "test".to_string()),
                ("language".to_string(), "en".to_string()),
            ],
            Ok(response_json.clone()),
        );

        let params = vec![
            ("query".to_string(), "test".to_string()),
            ("language".to_string(), "en".to_string()),
        ];
        let result = mock.send_search_request("/search/movie", &params).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), response_json);
    }

    #[tokio::test]
    async fn test_mock_response_simple() {
        let mock = MockTmdbTransport::new();
        let response_json = r#"{"results": []}"#.to_string();
        mock.mock_response_simple("/search/movie", Ok(response_json.clone()));

        let params = vec![("query".to_string(), "test".to_string())];
        let result = mock.send_search_request("/search/movie", &params).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), response_json);
    }

    #[tokio::test]
    async fn test_mock_response_error() {
        let mock = MockTmdbTransport::new();
        mock.mock_response_simple(
            "/search/movie",
            Err(TmdbSearchError {
                code: TmdbSearchErrorCode::RateLimited,
                message: "Rate limited".to_string(),
                retryable: true,
                retry_after: Some(30),
            }),
        );

        let params = vec![];
        let result = mock.send_search_request("/search/movie", &params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, TmdbSearchErrorCode::RateLimited);
    }

    #[tokio::test]
    async fn test_no_mock_response_returns_error() {
        let mock = MockTmdbTransport::new();
        let params = vec![];
        let result = mock.send_search_request("/unknown", &params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, TmdbSearchErrorCode::Unknown);
    }

    #[tokio::test]
    async fn test_call_history_recorded() {
        let mock = MockTmdbTransport::new();
        mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

        let params1 = vec![("query".to_string(), "test1".to_string())];
        let params2 = vec![("query".to_string(), "test2".to_string())];

        let _ = mock.send_search_request("/search/movie", &params1).await;
        let _ = mock.send_search_request("/search/movie", &params2).await;

        let history = mock.call_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "/search/movie?query=test1");
        assert_eq!(history[1], "/search/movie?query=test2");
    }

    #[tokio::test]
    async fn test_call_count() {
        let mock = MockTmdbTransport::new();
        mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

        assert_eq!(mock.call_count(), 0);

        let params = vec![];
        let _ = mock.send_search_request("/search/movie", &params).await;
        assert_eq!(mock.call_count(), 1);

        let _ = mock.send_search_request("/search/movie", &params).await;
        assert_eq!(mock.call_count(), 2);
    }

    #[tokio::test]
    async fn test_clear_history() {
        let mock = MockTmdbTransport::new();
        mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

        let params = vec![];
        let _ = mock.send_search_request("/search/movie", &params).await;
        assert_eq!(mock.call_count(), 1);

        mock.clear_history();
        assert_eq!(mock.call_count(), 0);
    }

    #[tokio::test]
    async fn test_default_trait() {
        let mock = MockTmdbTransport::default();
        assert_eq!(mock.call_count(), 0);
    }

    #[tokio::test]
    async fn test_no_api_key_in_mock() {
        let mock = MockTmdbTransport::new();
        mock.mock_response_simple("/search/movie", Ok(r#"{"results": []}"#.to_string()));

        let params = vec![("query".to_string(), "test".to_string())];
        let _ = mock.send_search_request("/search/movie", &params).await;

        // Verify no api_key in call history
        let history = mock.call_history();
        for call in &history {
            assert!(!call.contains("api_key"));
            assert!(!call.contains("apiKey"));
        }
    }

    #[tokio::test]
    async fn test_no_network_dependency() {
        let mock = MockTmdbTransport::new();
        mock.mock_response_simple("/search/movie", Ok("{}".to_string()));

        let params = vec![];
        let _ = mock.send_search_request("/search/movie", &params).await;
    }
}
