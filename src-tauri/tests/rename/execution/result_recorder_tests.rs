// ResultRecorder 测试
// 职责：测试结果记录

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
fn test_audit_result_recorded() {
    let tmp = TempDir::new().unwrap();
    let source = tmp.path().join("file.mkv");
    let target = tmp.path().join("renamed.mkv");
    fs::write(&source, b"content").unwrap();

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

    // 写入数据库
    for r in &results {
        db::insert_result(&conn, r).unwrap();
    }

    let stored = db::get_results_by_task(&conn, &task.id).unwrap();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].status, TaskStatus::Completed);
}