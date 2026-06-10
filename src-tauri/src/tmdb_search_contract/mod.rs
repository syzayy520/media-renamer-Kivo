// src-tauri/src/tmdb_search_contract/mod.rs
// TMDb Search Backend Contract 模块
// 职责：定义 TMDb 搜索后端契约类型，不包含实现逻辑

mod candidate;
mod error;
mod input;
mod media_type;
mod output;
mod rate_limit;

pub mod tests;

pub use candidate::{TmdbCandidate, TmdbCandidateSource};
pub use error::{TmdbSearchError, TmdbSearchErrorCode};
pub use input::SearchTmdbCandidatesInput;
pub use media_type::TmdbSearchMediaType;
pub use output::SearchTmdbCandidatesOutput;
pub use rate_limit::TmdbRateLimitState;
