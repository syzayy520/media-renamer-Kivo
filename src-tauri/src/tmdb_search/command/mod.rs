// src-tauri/src/tmdb_search/command/mod.rs
// TMDb Search Command Shell 模块
// 职责：定义 TMDb 搜索后端 command shell，不包含 Tauri command 注册

mod search_tmdb_candidates_command;

pub use search_tmdb_candidates_command::search_tmdb_candidates_command_shell;
