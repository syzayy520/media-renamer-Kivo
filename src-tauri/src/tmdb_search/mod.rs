// src-tauri/src/tmdb_search/mod.rs
// TMDb Search Backend Implementation Core
// 职责：定义 TMDb 搜索后端实现核心，不包含 Tauri command 注册

mod validation;
mod request;
mod response;
mod mapping;
pub mod transport;
pub mod tests;

pub use validation::search_input_validator::{validate_search_input, ValidatedTmdbSearchInput};
pub use request::search_query::{build_search_query, SearchQuery};
pub use request::search_url::build_search_url;
pub use response::tmdb_search_response::TmdbSearchResponse;
pub use response::tmdb_search_result::TmdbSearchResult;
pub use mapping::candidate_mapper::{map_to_candidate, map_to_candidates};
pub use mapping::error_mapper::{map_http_error, HttpErrorResponse, api_key_missing_error, timeout_error, network_connection_error};
pub use mapping::rate_limit_mapper::{map_rate_limit, HttpRateLimitResponse, is_rate_limited, seconds_until_reset};
pub use transport::{TmdbTransport, MockTmdbTransport};
