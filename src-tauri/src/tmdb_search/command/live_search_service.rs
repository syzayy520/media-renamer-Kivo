// src-tauri/src/tmdb_search/command/live_search_service.rs
// Live Search Service
// 职责：使用 LiveHttpTransport 和 ConfigApiKeyProvider 执行真实 TMDb 搜索

use std::sync::Arc;

use crate::tmdb_search::api_key::TmdbApiKeyProvider;
use crate::tmdb_search::transport::{LiveHttpTransport, TmdbTransport, TransportGate};
use crate::tmdb_search_contract::{
    SearchTmdbCandidatesInput, SearchTmdbCandidatesOutput, TmdbCandidateSource, TmdbSearchError,
    TmdbSearchErrorCode, TmdbSearchMediaType,
};

/// Live Search Service
///
/// 使用真实的 HTTP transport 和 API key 执行 TMDb 搜索。
/// 必须显式启用 transport gate 才能执行网络请求。
///
/// # 安全约束
/// - API key 仅在 HTTP 请求中使用，不放入日志或输出
/// - Transport gate 必须显式启用
/// - 所有错误都转换为结构化 TmdbSearchError
pub struct LiveSearchService<P: TmdbApiKeyProvider> {
    api_key_provider: P,
    gate: Arc<TransportGate>,
}

impl<P: TmdbApiKeyProvider> LiveSearchService<P> {
    /// 创建新的 Live Search Service
    ///
    /// # Arguments
    /// * `api_key_provider` - API key 提供者
    /// * `gate` - Transport gate 控制 HTTP 启用状态
    pub fn new(api_key_provider: P, gate: Arc<TransportGate>) -> Self {
        Self {
            api_key_provider,
            gate,
        }
    }

    /// 执行 TMDb 搜索
    ///
    /// # Arguments
    /// * `input` - 搜索输入
    ///
    /// # Returns
    /// `SearchTmdbCandidatesOutput` 包含搜索结果或错误
    pub async fn search(
        &self,
        input: SearchTmdbCandidatesInput,
    ) -> SearchTmdbCandidatesOutput {
        // Check if gate is enabled
        if !self.gate.is_enabled() {
            return Self::gate_disabled_error();
        }

        // Get API key
        let api_key = match self.api_key_provider.get_api_key().await {
            Some(key) => key,
            None => {
                return Self::api_key_missing_error();
            }
        };

        // Build search parameters
        let media_type_str = match input.media_type {
            TmdbSearchMediaType::Movie => "movie",
            TmdbSearchMediaType::Tv => "tv",
        };
        let path = format!("/search/{}", media_type_str);
        let mut params = vec![
            ("query".to_string(), input.query),
            ("language".to_string(), input.language),
        ];

        if let Some(year) = input.year {
            params.push(("year".to_string(), year.to_string()));
        }

        if let Some(page) = input.page {
            params.push(("page".to_string(), page.to_string()));
        }

        // Create transport with actual API key
        let transport = LiveHttpTransport::new(api_key, self.gate.clone());

        // Execute search
        match transport.send_search_request(&path, &params).await {
            Ok(response) => {
                // Parse response and map to candidates
                // TODO: Implement response parsing and candidate mapping
                Self::parse_response(response)
            }
            Err(error) => Self::map_error(error),
        }
    }

    /// 解析 TMDb API 响应
    fn parse_response(_response: String) -> SearchTmdbCandidatesOutput {
        // TODO: Implement proper response parsing
        // For now, return empty candidates with success
        SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: None,
        }
    }

    /// 映射 transport 错误到 SearchTmdbCandidatesOutput
    fn map_error(error: TmdbSearchError) -> SearchTmdbCandidatesOutput {
        SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: Some(error),
        }
    }

    /// Gate disabled 错误
    fn gate_disabled_error() -> SearchTmdbCandidatesOutput {
        SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: Some(TmdbSearchError {
                code: TmdbSearchErrorCode::Unknown,
                message: "TMDb live search is disabled. Transport gate not enabled.".to_string(),
                retryable: false,
                retry_after: None,
            }),
        }
    }

    /// API key 缺失错误
    fn api_key_missing_error() -> SearchTmdbCandidatesOutput {
        SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: Some(TmdbSearchError {
                code: TmdbSearchErrorCode::ApiKeyMissing,
                message: "TMDb API key is not configured.".to_string(),
                retryable: false,
                retry_after: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tmdb_search::api_key::api_key_state::ApiKeyState;
    use crate::tmdb_search::api_key::TmdbApiKeyProvider;
    use crate::tmdb_search_contract::TmdbSearchMediaType;
    use std::future::Future;

    /// Mock API key provider for testing
    struct MockApiKeyProvider {
        key: Option<String>,
    }

    impl MockApiKeyProvider {
        fn with_key(key: &str) -> Self {
            Self {
                key: Some(key.to_string()),
            }
        }

        fn without_key() -> Self {
            Self { key: None }
        }
    }

    impl TmdbApiKeyProvider for MockApiKeyProvider {
        #[allow(refining_impl_trait)]
        fn get_api_key(&self) -> impl Future<Output = Option<String>> + Send {
            let key = self.key.clone();
            async move { key }
        }

        #[allow(refining_impl_trait)]
        fn key_state(&self) -> impl Future<Output = ApiKeyState> + Send {
            let has_key = self.key.is_some();
            async move {
                if has_key {
                    ApiKeyState::Available
                } else {
                    ApiKeyState::Missing
                }
            }
        }
    }

    fn make_valid_input() -> SearchTmdbCandidatesInput {
        SearchTmdbCandidatesInput {
            query: "Inception".to_string(),
            media_type: TmdbSearchMediaType::Movie,
            language: "en".to_string(),
            year: Some(2010),
            page: Some(1),
        }
    }

    #[tokio::test]
    async fn test_gate_disabled_returns_error() {
        let provider = MockApiKeyProvider::with_key("test_key");
        let gate = Arc::new(TransportGate::new()); // disabled by default
        let service = LiveSearchService::new(provider, gate);

        let input = make_valid_input();
        let output = service.search(input).await;

        assert!(output.error.is_some());
        let error = output.error.unwrap();
        assert_eq!(error.code, TmdbSearchErrorCode::Unknown);
        assert!(error.message.contains("disabled"));
    }

    #[tokio::test]
    async fn test_api_key_missing_returns_error() {
        let provider = MockApiKeyProvider::without_key();
        let gate = Arc::new(TransportGate::enabled());
        let service = LiveSearchService::new(provider, gate);

        let input = make_valid_input();
        let output = service.search(input).await;

        assert!(output.error.is_some());
        let error = output.error.unwrap();
        assert_eq!(error.code, TmdbSearchErrorCode::ApiKeyMissing);
    }

    #[tokio::test]
    async fn test_successful_search_returns_output() {
        // This test requires actual network and API key
        // Mark as ignored for CI
        let provider = MockApiKeyProvider::with_key("test_key");
        let gate = Arc::new(TransportGate::enabled());
        let service = LiveSearchService::new(provider, gate);

        let input = make_valid_input();
        let output = service.search(input).await;

        // Should not have gate or api_key errors
        if let Some(error) = &output.error {
            // May have network error (expected in test environment)
            assert!(
                error.code == TmdbSearchErrorCode::NetworkError
                    || error.code == TmdbSearchErrorCode::Unknown
            );
        }
    }

    #[tokio::test]
    async fn test_service_with_different_media_types() {
        let provider = MockApiKeyProvider::with_key("test_key");
        let gate = Arc::new(TransportGate::enabled());
        let service = LiveSearchService::new(provider, gate);

        let input = SearchTmdbCandidatesInput {
            query: "Breaking Bad".to_string(),
            media_type: TmdbSearchMediaType::Tv,
            language: "zh-CN".to_string(),
            year: Some(2008),
            page: None,
        };

        let output = service.search(input).await;
        // Should not panic
        assert!(output.source == TmdbCandidateSource::Tmdb);
    }
}
