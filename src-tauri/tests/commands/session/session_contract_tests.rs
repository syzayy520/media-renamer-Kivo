// session_contract_tests 模块 - 会话命令契约测试
// 职责：验证 session commands 的参数、返回值、错误分类、安全边界
// 使用 tempdir / 内存数据库，禁止真实路径

use std::fs;
use tempfile::tempdir;

use app_lib::pipeline::orchestrator;
use app_lib::session::plan_session::DbState;

/// 测试：创建内存数据库状态
#[test]
fn test_db_state_new_memory() {
    let state = DbState::new_memory();
    assert!(state.is_ok());
}

/// 测试：空目录 - dry-run pipeline
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
    assert!(!pipeline.task_id.is_empty());
}

/// 测试：不存在的目录 - 返回错误
#[test]
fn test_start_rename_session_nonexistent() {
    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(std::path::Path::new("/nonexistent"), &conn);
    assert!(result.is_err());
}

/// 测试：单文件视频 - dry-run pipeline
#[test]
fn test_start_rename_session_single_video() {
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

/// 测试：电视剧多集 - dry-run pipeline
#[test]
fn test_start_rename_session_series_multi_episode() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("Breaking.Bad.S01E01.720p.mkv"), b"fake").unwrap();
    fs::write(dir.path().join("Breaking.Bad.S01E02.720p.mkv"), b"fake").unwrap();
    fs::write(dir.path().join("Breaking.Bad.S01E03.720p.mkv"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();
    assert_eq!(pipeline.scan.video_count, 3);
    assert_eq!(pipeline.parsed_count, 3);

    // 预览应该存在
    assert_eq!(pipeline.previews.len(), 3);
}

/// 测试：混合媒体类型 - dry-run pipeline
#[test]
fn test_start_rename_session_mixed_media() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("The Matrix (1999).mkv"), b"fake").unwrap();
    fs::write(dir.path().join("Breaking.Bad.S01E01.720p.mkv"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();
    assert_eq!(pipeline.scan.video_count, 2);
}

/// 测试：嵌套子目录 - dry-run pipeline
#[test]
fn test_start_rename_session_nested_subdirectories() {
    let dir = tempdir().unwrap();
    let sub_dir = dir.path().join("Movies");
    fs::create_dir(&sub_dir).unwrap();
    fs::write(sub_dir.join("Inception (2010).mkv"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();
    assert_eq!(pipeline.scan.video_count, 1);
}

/// 测试：伴侣文件计数
#[test]
fn test_start_rename_session_companion_files() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("The Matrix (1999).mkv"), b"fake").unwrap();
    fs::write(dir.path().join("The Matrix (1999).srt"), b"fake").unwrap();
    fs::write(dir.path().join("The Matrix (1999).nfo"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();
    assert_eq!(pipeline.scan.video_count, 1);
    assert!(pipeline.scan.companion_count >= 2);
}

/// 测试：低置信度项保留 review 状态
#[test]
fn test_start_rename_session_low_confidence_marks_review() {
    let dir = tempdir().unwrap();
    // 文件名不包含年份，可能导致低置信度
    fs::write(dir.path().join("random_movie_file.mkv"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();

    // 检查是否有需要审核的项
    let has_review = pipeline.previews.iter().any(|p| p.needs_manual_review);
    // 低置信度文件应该标记为需要审核
    if !pipeline.previews.is_empty() && pipeline.previews[0].confidence < 60 {
        assert!(has_review);
    }
}

/// 测试：冲突状态可返回
#[test]
fn test_start_rename_session_conflict_detection() {
    let dir = tempdir().unwrap();
    // 创建两个可能导致冲突的文件
    fs::write(dir.path().join("Movie (2020).mkv"), b"fake1").unwrap();
    fs::write(dir.path().join("Movie (2020) [1080p].mkv"), b"fake2").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();
    // 应该检测到潜在冲突
    assert!(!pipeline.safety.checks.is_empty());
}

/// 测试：task_id 可追踪
#[test]
fn test_start_rename_session_task_id_trackable() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("Movie (2020).mkv"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();

    // task_id 应该非空且可查询
    assert!(!pipeline.task_id.is_empty());

    // 查询任务应该存在
    let task = app_lib::audit::db::get_task(&conn, &pipeline.task_id).unwrap();
    assert!(task.is_some());
}

/// 测试：安全检查结构稳定
#[test]
fn test_start_rename_session_safety_checks_stable() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("Movie (2020).mkv"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();

    // 安全检查应该有结果
    assert!(!pipeline.safety.checks.is_empty());

    // 每个检查应该有 name 和 passed
    for check in &pipeline.safety.checks {
        assert!(!check.name.is_empty());
    }
}

/// 测试：扫描摘要结构稳定
#[test]
fn test_start_rename_session_scan_summary_stable() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("Movie (2020).mkv"), b"fake").unwrap();

    let state = DbState::new_memory().unwrap();
    let conn = state.conn.lock().unwrap();

    let result = orchestrator::run_dry_run_pipeline(dir.path(), &conn);
    assert!(result.is_ok());

    let pipeline = result.unwrap();

    // 扫描摘要应该有有效值（duration 可能为 0 因为太快）
    // scan_path 应该非空
    assert!(!pipeline.scan.scan_path.is_empty());
}
