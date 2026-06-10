// 模块声明
pub mod audit;
pub mod commands;
pub mod config;
pub mod parse;
pub mod pipeline;
pub mod rename;
pub mod rollback;
pub mod scan;
pub mod session;
pub mod shared;
pub mod tmdb_search;
pub mod tmdb_search_contract;

use commands::{
    clear_tmdb_api_key, get_all_tasks, get_all_templates, get_app_config, get_audit_logs,
    get_confidence_threshold, get_task, get_tmdb_api_key_status, search_tmdb_candidates,
    set_confidence_threshold, set_template, set_tmdb_api_key,
};
use session::plan_session::{start_rename_session, DbState};
use tauri::Manager;
use tmdb_search::command::TmdbSearchState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 初始化数据库（默认存放在 app data 目录）
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).ok();
            let db_path = app_dir.join("media-renamer.db");
            let db_state = DbState::new(&db_path).expect("Failed to initialize database");
            app.manage(db_state);

            // 初始化 TMDb 搜索状态
            let tmdb_search_state = TmdbSearchState::new(app_dir.clone());
            app.manage(tmdb_search_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_rename_session,
            get_app_config,
            get_all_templates,
            set_template,
            get_confidence_threshold,
            set_confidence_threshold,
            get_tmdb_api_key_status,
            set_tmdb_api_key,
            clear_tmdb_api_key,
            get_task,
            get_all_tasks,
            get_audit_logs,
            search_tmdb_candidates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
