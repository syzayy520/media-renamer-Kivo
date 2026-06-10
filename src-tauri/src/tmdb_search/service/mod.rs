// src-tauri/src/tmdb_search/service/mod.rs
// TMDb Search Service 模块
// 职责：定义 TMDb 搜索服务，包含 LiveSearchService

pub mod live_search_service;

pub use live_search_service::LiveSearchService;
