// ExecutorCore 测试
// 职责：测试执行器核心编排

use app_lib::rename::execution::{execute, ExecutionMode};
use app_lib::audit::db::{self, init_tables, TaskStatus};
use app_lib::rename::template::{RenamePreviewItem, MetadataSource};
use app_lib::parse::movie_parser::{MediaType, ParsedMediaInfo};
use app_lib::scan::MediaItem;
use rusqlite::Connection;
use tempfile::TempDir;
use uuid::Uuid;
use std::fs;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    init_tables(&conn).unwrap();
    conn
}

fn make_preview_item(
    source: &str,
    target: &str,
    confidence: u8,
    needs_review: bool,
) -> RenamePreviewItem {
    RenamePreviewItem {
        id: Uuid::new_v4().to_string(),
        parsed_info: ParsedMediaInfo {
            media_item: MediaItem {
                id: "test".to_string(),
                file_path: source.to_string(),
                file_name: std::path::Path::new(source)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                extension: "mkv".to_string(),
                file_size: 0,
                is_video: true,
                is_companion: false,
            },
            media_type: MediaType::Movie,
            title: "Test".to_string(),
            year: Some(2020),
            season: None,
            episode: None,
            episode_end: None,
            episode_title: None,
            resolution: Some("1080p".to_string()),
            source: None,
            video_codec: None,
            audio_codec: None,
            group: None,
            confidence,
            rule_sources: vec![],
            special_type: None,
            extra_type: None,
            extra_number: None,
        },
        source_path: source.to_string(),
        original_name: std::path::Path::new(source)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        proposed_name: std::path::Path::new(target)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        target_path: target.to_string(),
        media_type: MediaType::Movie,
        confidence,
        needs_manual_review: needs_review,
        should_skip: needs_review,
        conflicts: vec![],
        evidence: vec![],
        metadata_source: MetadataSource::LocalRule,
    }
}

#[test]
fn test_dry_run_does_not_modify_files() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("original.mkv");
    let target = tmp.path().join("renamed.mkv");
    fs::write(&source, b"test content").unwrap();

    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();

    let item = make_preview_item(
        source.to_string_lossy().to_string().as_str(),
        target.to_string_lossy().to_string().as_str(),
        85,
        false,
    );

    let results = execute(&conn, &task.id, &[item], ExecutionMode::DryRun).unwrap();

    // 文件不应被修改
    assert!(source.exists());
    assert!(!target.exists());
    // 结果应为 Pending（dry-run 模拟成功）
    assert_eq!(results[0].status, TaskStatus::Pending);
}

#[test]
fn test_confirmed_rename_success() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("original.mkv");
    let target = tmp.path().join("renamed.mkv");
    fs::write(&source, b"test content").unwrap();

    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();

    let item = make_preview_item(
        source.to_string_lossy().to_string().as_str(),
        target.to_string_lossy().to_string().as_str(),
        85,
        false,
    );

    let results = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed).unwrap();

    assert!(!source.exists());
    assert!(target.exists());
    assert_eq!(results[0].status, TaskStatus::Completed);
}

#[test]
fn test_source_not_found_fails() {
    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();

    let item = make_preview_item(
        "C:\\nonexistent\\file.mkv",
        "C:\\nonexistent\\renamed.mkv",
        85,
        false,
    );

    let results = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed).unwrap();

    assert_eq!(results[0].status, TaskStatus::Failed);
    assert!(results[0]
        .error_message
        .as_ref()
        .unwrap()
        .contains("not found"));
}

#[test]
fn test_target_already_exists_fails() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("source.mkv");
    let target = tmp.path().join("existing.mkv");
    fs::write(&source, b"source").unwrap();
    fs::write(&target, b"existing").unwrap();

    let conn = setup_db();
    let task = db::create_task("{Title}", 1);
    db::insert_task(&conn, &task).unwrap();

    let item = make_preview_item(
        source.to_string_lossy().to_string().as_str(),
        target.to_string_lossy().to_string().as_str(),
        85,
        false,
    );

    let results = execute(&conn, &task.id, &[item], ExecutionMode::Confirmed).unwrap();

    assert_eq!(results[0].status, TaskStatus::Failed);
    assert!(results[0]
        .error_message
        .as_ref()
        .unwrap()
        .contains("already exists"));
}

#[test]
fn test_partial_failure_aggregation() {
    let tmp = TempDir::new().unwrap();
    let source_ok = tmp.path().join("ok.mkv");
    let target_ok = tmp.path().join("renamed_ok.mkv");
    fs::write(&source_ok, b"ok").unwrap();

    let conn = setup_db();
    let task = db::create_task("{Title}", 2);
    db::insert_task(&conn, &task).unwrap();

    let item_ok = make_preview_item(
        source_ok.to_string_lossy().to_string().as_str(),
        target_ok.to_string_lossy().to_string().as_str(),
        85,
        false,
    );

    // 第二个文件源不存在
    let item_bad = make_preview_item(
        "C:\\nonexistent\\bad.mkv",
        "C:\\nonexistent\\bad_renamed.mkv",
        85,
        false,
    );

    let results = execute(
        &conn,
        &task.id,
        &[item_ok, item_bad],
        ExecutionMode::Confirmed,
    )
    .unwrap();

    assert_eq!(results.len(), 2);
    let summary = app_lib::rename::execution::summarize(&results, 2);
    assert_eq!(summary.success, 1);
    assert_eq!(summary.failed, 1);
}