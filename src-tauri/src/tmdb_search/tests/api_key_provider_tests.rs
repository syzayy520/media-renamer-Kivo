// src-tauri/src/tmdb_search/tests/api_key_provider_tests.rs
// API Key Provider 测试
// 职责：验证 API key provider 抽象的正确性和安全性

use crate::tmdb_search::api_key::{ApiKeyState, TmdbApiKeyProvider};
use std::sync::{Arc, Mutex};

// ============ Mock Implementation ============

/// 测试用 Mock API Key Provider
///
/// 用于测试目的，返回预设的 key 值和状态。
/// 不读取真实环境变量、文件系统或配置。
#[derive(Clone)]
struct TestApiKeyProvider {
    key: Option<String>,
    state: ApiKeyState,
    call_count: Arc<Mutex<usize>>,
}

impl TestApiKeyProvider {
    fn available(key: &str) -> Self {
        Self {
            key: Some(key.to_string()),
            state: ApiKeyState::Available,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    fn missing() -> Self {
        Self {
            key: None,
            state: ApiKeyState::Missing,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    fn invalid() -> Self {
        Self {
            key: None,
            state: ApiKeyState::Invalid,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }
}

impl TmdbApiKeyProvider for TestApiKeyProvider {
    #[allow(refining_impl_trait)]
    fn get_api_key(&self) -> impl std::future::Future<Output = Option<String>> + Send {
        let key = self.key.clone();
        let count = self.call_count.clone();
        async move {
            *count.lock().unwrap() += 1;
            key
        }
    }

    #[allow(refining_impl_trait)]
    fn key_state(&self) -> impl std::future::Future<Output = ApiKeyState> + Send {
        let state = self.state.clone();
        async move { state }
    }
}

// ============ ApiKeyState Tests ============

#[test]
fn test_api_key_state_available() {
    let state = ApiKeyState::Available;
    assert!(state.is_available());
    assert!(!state.is_missing());
    assert!(!state.is_invalid());
}

#[test]
fn test_api_key_state_missing() {
    let state = ApiKeyState::Missing;
    assert!(!state.is_available());
    assert!(state.is_missing());
    assert!(!state.is_invalid());
}

#[test]
fn test_api_key_state_invalid() {
    let state = ApiKeyState::Invalid;
    assert!(!state.is_available());
    assert!(!state.is_missing());
    assert!(state.is_invalid());
}

#[test]
fn test_api_key_state_display() {
    assert_eq!(ApiKeyState::Available.to_string(), "available");
    assert_eq!(ApiKeyState::Missing.to_string(), "missing");
    assert_eq!(ApiKeyState::Invalid.to_string(), "invalid");
}

#[test]
fn test_api_key_state_serialization_no_real_key() {
    let state = ApiKeyState::Available;
    let json = serde_json::to_string(&state).unwrap();
    assert_eq!(json, "\"Available\"");
    assert!(!json.contains("api_key"));
    assert!(!json.contains("apiKey"));
    assert!(!json.contains("token"));
}

#[test]
fn test_api_key_state_clone_eq() {
    let state = ApiKeyState::Available;
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

// ============ Provider Tests ============

#[tokio::test]
async fn test_provider_returns_key_when_available() {
    let provider = TestApiKeyProvider::available("test-key-123");
    let key = provider.get_api_key().await;
    assert_eq!(key, Some("test-key-123".to_string()));
}

#[tokio::test]
async fn test_provider_returns_none_when_missing() {
    let provider = TestApiKeyProvider::missing();
    let key = provider.get_api_key().await;
    assert_eq!(key, None);
}

#[tokio::test]
async fn test_provider_returns_state_available() {
    let provider = TestApiKeyProvider::available("test-key");
    let state = provider.key_state().await;
    assert_eq!(state, ApiKeyState::Available);
}

#[tokio::test]
async fn test_provider_returns_state_missing() {
    let provider = TestApiKeyProvider::missing();
    let state = provider.key_state().await;
    assert_eq!(state, ApiKeyState::Missing);
}

#[tokio::test]
async fn test_provider_returns_state_invalid() {
    let provider = TestApiKeyProvider::invalid();
    let state = provider.key_state().await;
    assert_eq!(state, ApiKeyState::Invalid);
}

#[tokio::test]
async fn test_provider_tracks_call_count() {
    let provider = TestApiKeyProvider::available("key");
    assert_eq!(provider.call_count(), 0);

    let _ = provider.get_api_key().await;
    assert_eq!(provider.call_count(), 1);

    let _ = provider.get_api_key().await;
    assert_eq!(provider.call_count(), 2);
}

// ============ Safety Tests ============

#[test]
fn test_api_key_state_debug_contains_no_secrets() {
    // ApiKeyState debug output must not contain any key material
    let state_debug = format!("{:?}", ApiKeyState::Available);
    assert!(!state_debug.contains("super-secret-key"));
    assert!(!state_debug.contains("api_key"));
    assert!(!state_debug.contains("token"));
}

#[test]
fn test_api_key_state_json_contains_no_secrets() {
    let state_json = serde_json::to_string(&ApiKeyState::Available).unwrap();
    assert!(!state_json.contains("super-secret-key"));
    assert!(!state_json.contains("api_key"));
    assert!(!state_json.contains("token"));
}

#[test]
fn test_key_state_serialization_contains_no_secrets() {
    let state = ApiKeyState::Available;
    let json = serde_json::to_string(&state).unwrap();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("apiKey"));
    assert!(!json.contains("token"));
    assert!(!json.contains("bearer"));
    assert!(!json.contains("authorization"));
    assert!(!json.contains("session"));
}

#[tokio::test]
async fn test_mock_provider_key_not_in_state() {
    let provider = TestApiKeyProvider::available("my-secret-api-key");
    let state = provider.key_state().await;
    let state_json = serde_json::to_string(&state).unwrap();
    assert!(!state_json.contains("my-secret-api-key"));
}

// ============ Send + Sync Tests ============

#[test]
fn test_provider_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TestApiKeyProvider>();
}

#[tokio::test]
async fn test_provider_works_across_threads() {
    let provider = TestApiKeyProvider::available("cross-thread-key");
    let provider_clone = provider.clone();

    let handle = tokio::spawn(async move {
        let key = provider_clone.get_api_key().await;
        key
    });

    let result = handle.await.unwrap();
    assert_eq!(result, Some("cross-thread-key".to_string()));
}
