// pipeline/orchestrator 模块 - 端到端流水线编排
// 职责：串联 Scan → Parse → Preview → Safety Check → Audit
// 委托：scan_directory / classify_and_parse / generate / check_all / task_repository / AuditLogger
// 禁止：直接 I/O、模板渲染、冲突检测 — 全部委托子模块

use std::path::Path;

use rusqlite::Connection;

use crate::audit::db::{self, task_repository};
use crate::audit::logger::AuditLogger;
use crate::parse::classifier;
use crate::rename::preview_generator;
use crate::rename::safety_checker;
use crate::scan;
use crate::shared::result_types::{AppError, AppResult};

use super::result::{PipelineResult, ScanSummary};

/// 去掉文件扩展名（如 "Movie.mkv" → "Movie"）
fn strip_extension(file_name: &str) -> String {
    match file_name.rfind('.') {
        Some(pos) if pos > 0 => file_name[..pos].to_string(),
        _ => file_name.to_string(),
    }
}

/// 运行 dry-run 流水线
///
/// 完整流程：
/// 1. scan — 扫描目录获取 MediaItem 列表
/// 2. parse — 逐文件分类 + 解析
/// 3. preview — 模板渲染 + 冲突检测
/// 4. safety — 安全检查
/// 5. audit — 创建任务 + 写审计日志
///
/// 参数：
/// - scan_path: 待扫描目录
/// - conn: SQLite 连接（用于写审计记录）
pub fn run_dry_run_pipeline(scan_path: &Path, conn: &Connection) -> AppResult<PipelineResult> {
    // ── Step 1: Scan ──
    let (report, media_items) = scan::scan_directory(scan_path)?;

    // ── Step 2: Parse ──
    let mut parsed_items = Vec::new();
    let mut unknown_count: u32 = 0;

    for item in &media_items {
        // 去掉扩展名再解析（parser 正则不处理扩展名）
        let stem = strip_extension(&item.file_name);
        match classifier::classify_and_parse(&stem) {
            Some(info) => parsed_items.push(info),
            None => {
                unknown_count += 1;
            }
        }
    }

    // ── Step 3: Preview ──
    // 使用第一个文件的媒体类型选择默认模板
    let template_str = if let Some(first) = parsed_items.first() {
        crate::rename::template::get_default_template(&first.media_type).to_string()
    } else {
        "{Title}.{ext}".to_string()
    };

    let previews = preview_generator::generate(&parsed_items, &template_str);

    // ── Step 4: Safety Check ──
    let safety = safety_checker::check_all(&previews);

    // ── Step 5: Audit ──
    // 初始化数据库表（幂等操作）
    db::init_tables(conn).map_err(|e| AppError::Database(e.to_string()))?;

    // 创建任务记录
    let task = task_repository::create_task(&template_str, report.video_count);
    task_repository::insert_task(conn, &task).map_err(|e| AppError::Database(e.to_string()))?;

    // 写审计日志
    let logger = AuditLogger::new(conn);
    logger
        .log_task_created(&task.id, &template_str)
        .map_err(|e| AppError::Database(e.to_string()))?;
    logger
        .log_preview(&task.id, report.video_count)
        .map_err(|e| AppError::Database(e.to_string()))?;

    // ── Assemble Result ──
    let parsed_count = parsed_items.len() as u32;

    Ok(PipelineResult {
        task_id: task.id,
        scan: ScanSummary {
            scan_path: report.scan_path,
            video_count: report.video_count,
            companion_count: report.companion_count,
            permission_error_count: report.permission_errors.len() as u32,
            scan_duration_ms: report.scan_duration_ms,
        },
        previews,
        safety,
        parsed_count,
        unknown_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        db::init_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_pipeline_empty_directory() {
        let dir = tempdir().unwrap();
        let conn = setup_db();

        let result = run_dry_run_pipeline(dir.path(), &conn);
        assert!(result.is_ok());

        let pipeline = result.unwrap();
        assert_eq!(pipeline.scan.video_count, 0);
        assert_eq!(pipeline.previews.len(), 0);
        assert_eq!(pipeline.parsed_count, 0);
        assert_eq!(pipeline.unknown_count, 0);
        // 任务 ID 应已写入数据库
        let task = db::get_task(&conn, &pipeline.task_id).unwrap();
        assert!(task.is_some());
    }

    #[test]
    fn test_pipeline_with_video_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("The Matrix (1999) [1080p].mkv"), b"fake").unwrap();
        fs::write(dir.path().join("random_file.mkv"), b"fake").unwrap();

        let conn = setup_db();
        let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

        assert_eq!(result.scan.video_count, 2);
        // "The Matrix (1999)" 应被解析为电影
        assert!(result.parsed_count >= 1);
        // "random_file.mkv" 应被视为未识别
        assert!(result.unknown_count >= 1);
        // 预览项数 = parsed_count
        assert_eq!(result.previews.len() as u32, result.parsed_count);
        // 安全报告应已生成
        assert!(result.safety.dry_run);
        // 审计任务应存在
        assert!(!result.task_id.is_empty());
        let task = db::get_task(&conn, &result.task_id).unwrap();
        assert!(task.is_some());
        assert_eq!(task.unwrap().status, db::TaskStatus::Previewing);
    }

    #[test]
    fn test_pipeline_nonexistent_path() {
        let conn = setup_db();
        let result = run_dry_run_pipeline(Path::new("/nonexistent/path"), &conn);
        assert!(result.is_err());
    }

    #[test]
    fn test_pipeline_audit_logs_written() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("Movie (2020).mkv"), b"fake").unwrap();

        let conn = setup_db();
        let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

        // 验证审计日志已写入
        let logs = db::get_logs_by_task(&conn, &result.task_id).unwrap();
        assert!(logs.len() >= 2); // task_created + preview

        let event_types: Vec<&str> = logs.iter().map(|e| e.event_type.as_str()).collect();
        assert!(event_types.contains(&"task_created"));
        assert!(event_types.contains(&"preview"));
    }

    #[test]
    fn test_pipeline_series_file_parsed() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("Breaking Bad - S01E01 - Pilot [1080p].mkv"),
            b"fake",
        )
        .unwrap();

        let conn = setup_db();
        let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

        assert_eq!(result.parsed_count, 1);
        assert_eq!(result.previews.len(), 1);
        assert_eq!(
            result.previews[0].media_type,
            crate::parse::movie_parser::MediaType::Series
        );
        assert!(result.previews[0].proposed_name.contains("Breaking Bad"));
    }
}
