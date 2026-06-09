// src-tauri/src/tmdb_search/mod.rs
// TMDb Search Backend Implementation Core
// 职责：定义 TMDb 搜索后端实现核心，不包含 Tauri command 注册

mod validation;
mod request;
mod response;
mod mapping;
pub mod tests;

pub use validation::search_input_validator::{validate_search_input, ValidatedTmdbSearchInput};
pub use request::search_query::{build_search_query, SearchQuery};
pub use request::search_url::build_search_url;
pub use response::tmdb_search_response::TmdbSearchResponse;
pub use response::tmdb_search_result::TmdbSearchResult;
pub use mapping::candidate_mapper::{map_to_candidate, map_to_candidates};
