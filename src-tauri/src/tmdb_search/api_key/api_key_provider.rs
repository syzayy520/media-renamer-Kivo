// src-tauri/src/tmdb_search/api_key/api_key_provider.rs
// API Key Provider 抽象
// 职责：定义 API key 提供者特征，不包含真实 key 读取逻辑

use std::future::Future;

use super::api_key_state::ApiKeyState;

/// API Key 提供者特征
///
/// 定义获取 TMDb API key 的抽象接口。
/// 实现者可以是配置存储、环境变量读取器或 Mock 实现。
///
/// # 安全约束
/// - 实现者不得将 key 值放入 Debug 输出
/// - 实现者不得将 key 值日志打印
/// - 实现者不得将 key 值放入 DTO/序列化结构
pub trait TmdbApiKeyProvider: Send + Sync {
    /// 获取 API key
    ///
    /// 返回 API key 字符串。如果 key 不可用，应返回 None。
    /// 返回的 key 值仅用于构建 HTTP 请求，不得持久化或日志记录。
    fn get_api_key(&self) -> impl Future<Output = Option<String>> + Send;

    /// 查询 API key 状态
    ///
    /// 返回当前 key 的可用性状态，不包含真实 key 值。
    /// 用于在不暴露 key 的前提下检查 key 是否可用。
    fn key_state(&self) -> impl Future<Output = ApiKeyState> + Send;
}
