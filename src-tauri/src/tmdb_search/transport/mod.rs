// src-tauri/src/tmdb_search/transport/mod.rs
// Transport 模块
// 职责：定义 TMDb API 传输层抽象

pub mod live_http;
pub mod mock;
pub mod transport_gate;

pub use live_http::LiveHttpTransport;
pub use mock::MockTmdbTransport;
pub use transport_gate::TransportGate;

use crate::tmdb_search_contract::TmdbSearchError;
use std::future::Future;

/// TMDb API 传输层特征
/// 定义与 TMDb API 通信的抽象接口
/// 实现者可以是真实 HTTP 客户端或 Mock 实现
pub trait TmdbTransport {
    /// 发送搜索请求并返回响应 JSON 字符串
    ///
    /// # Arguments
    /// * `path` - API 路径（如 "/search/movie"）
    /// * `params` - 查询参数列表
    ///
    /// # Returns
    /// 成功时返回 JSON 响应字符串，失败时返回 TmdbSearchError
    fn send_search_request(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> impl Future<Output = Result<String, TmdbSearchError>> + Send;
}
