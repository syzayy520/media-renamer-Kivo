// src-tauri/src/tmdb_search/mod.rs
// TMDb Search Backend Implementation Core
// 职责：定义 TMDb 搜索后端实现核心，不包含 Tauri command 注册

pub mod api_key;
pub mod command;
mod mapping;
mod request;
mod response;
pub mod service;
pub mod tests;
pub mod transport;
mod validation;

pub use api_key::{ApiKeyState, ConfigApiKeyProvider, TmdbApiKeyProvider};
pub use command::search_tmdb_candidates_command_shell;
pub use command::TmdbSearchState;
pub use mapping::candidate_mapper::{map_to_candidate, map_to_candidates};
pub use mapping::error_mapper::{
    api_key_missing_error, map_http_error, network_connection_error, timeout_error,
    HttpErrorResponse,
};
pub use mapping::rate_limit_mapper::{
    is_rate_limited, map_rate_limit, seconds_until_reset, HttpRateLimitResponse,
};
pub use request::search_query::{build_search_query, SearchQuery};
pub use request::search_url::build_search_url;
pub use response::tmdb_search_response::TmdbSearchResponse;
pub use response::tmdb_search_result::TmdbSearchResult;
pub use service::LiveSearchService;
pub use transport::{LiveHttpTransport, MockTmdbTransport, TmdbTransport, TransportGate};
pub use validation::search_input_validator::{validate_search_input, ValidatedTmdbSearchInput};
