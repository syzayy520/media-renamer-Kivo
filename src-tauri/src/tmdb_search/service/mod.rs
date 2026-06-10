// src-tauri/src/tmdb_search/service/mod.rs
// TMDb Search Service 模块
// 职责：定义 TMDb 搜索服务子族入口，只声明服务子模块并重导出公开服务

pub mod live_search_service;
pub mod parse_search_response;

pub use live_search_service::LiveSearchService;
