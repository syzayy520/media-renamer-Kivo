// src-tauri/src/tmdb_search/mod.rs
// TMDb Search Backend Implementation Core
// 职责：定义 TMDb 搜索后端实现核心，不包含 Tauri command 注册

mod validation;
pub mod tests;

pub use validation::search_input_validator::{validate_search_input, ValidatedTmdbSearchInput};
