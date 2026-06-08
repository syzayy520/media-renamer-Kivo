// session/plan_session 模块 - 重命名计划会话 Tauri 命令
// 职责：将 pipeline::orchestrator::run_dry_run_pipeline 暴露为 Tauri 命令
// 参数转换：String ↔ Path，AppError → String
// 禁止：业务逻辑、直接 I/O

use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;

use crate::audit::db;
use crate::pipeline::orchestrator;
use crate::pipeline::result::PipelineResult;
use crate::shared::result_types::AppError;

/// 数据库连接状态（Tauri managed state）
pub struct DbState {
    pub conn: Mutex<Connection>,
}

impl DbState {
    /// 创建新的数据库状态
    pub fn new(db_path: &Path) -> Result<Self, AppError> {
        let conn = Connection::open(db_path)
            .map_err(|e| AppError::Database(format!("Open database error: {}", e)))?;
        db::init_tables(&conn)
            .map_err(|e| AppError::Database(format!("Init tables error: {}", e)))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// 创建内存数据库（用于测试）
    pub fn new_memory() -> Result<Self, AppError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| AppError::Database(format!("Open memory database error: {}", e)))?;
        db::init_tables(&conn)
            .map_err(|e| AppError::Database(format!("Init tables error: {}", e)))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

/// 启动重命名计划会话（Tauri 命令）
///
/// 流程：扫描目录 → 解析文件名 → 生成预览 → 安全检查 → 写审计记录
///
/// 参数：
/// - directory: 待扫描目录路径
/// - db_state: Tauri managed 数据库状态
///
/// 返回：PipelineResult（序列化为 JSON 传给前端）
#[tauri::command]
pub fn start_rename_session(
    directory: String,
    db_state: tauri::State<'_, DbState>,
) -> Result<PipelineResult, String> {
    let path = Path::new(&directory);

    let conn = db_state
        .conn
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;

    orchestrator::run_dry_run_pipeline(path, &conn).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_db_state_new_memory() {
        let state = DbState::new_memory();
        assert!(state.is_ok());
    }

    #[test]
    fn test_start_rename_session_with_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("The Matrix (1999) [1080p].mkv"), b"fake").unwrap();

        let state = DbState::new_memory().unwrap();
        let conn = state.conn.lock().unwrap();

        let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
        assert!(result.is_ok());

        let pipeline = result.unwrap();
        assert_eq!(pipeline.scan.video_count, 1);
        assert_eq!(pipeline.parsed_count, 1);
        assert!(!pipeline.task_id.is_empty());
    }

    #[test]
    fn test_start_rename_session_empty_dir() {
        let dir = tempdir().unwrap();
        let state = DbState::new_memory().unwrap();
        let conn = state.conn.lock().unwrap();

        let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
        assert!(result.is_ok());

        let pipeline = result.unwrap();
        assert_eq!(pipeline.scan.video_count, 0);
        assert_eq!(pipeline.previews.len(), 0);
    }

    #[test]
    fn test_start_rename_session_nonexistent() {
        let state = DbState::new_memory().unwrap();
        let conn = state.conn.lock().unwrap();

        let result = orchestrator::run_dry_run_pipeline(Path::new("/nonexistent"), &conn);
        assert!(result.is_err());
    }
}
