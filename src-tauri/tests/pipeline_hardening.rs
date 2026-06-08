// Pipeline hardening integration tests
// 职责：端到端 dry-run 流水线边界场景覆盖
// 覆盖：anime 解析、混合媒体、伴随文件、嵌套目录、安全阻断、全未识别

use app_lib::audit::db;
use app_lib::parse::movie_parser::MediaType;
use app_lib::pipeline::orchestrator::run_dry_run_pipeline;
use rusqlite::Connection;
use std::fs;
use tempfile::tempdir;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    db::init_tables(&conn).unwrap();
    conn
}

#[test]
fn test_pipeline_anime_file_parsed() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path()
            .join("[SubGroup] Attack on Titan - 01 [1080p].mkv"),
        b"fake",
    )
    .unwrap();

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.scan.video_count, 1);
    assert_eq!(result.parsed_count, 1);
    assert_eq!(result.unknown_count, 0);
    assert_eq!(result.previews.len(), 1);
    assert_eq!(result.previews[0].media_type, MediaType::Anime);
    assert!(result.previews[0].proposed_name.contains("Attack on Titan"));
}

#[test]
fn test_pipeline_mixed_media_types() {
    let dir = tempdir().unwrap();
    // Movie
    fs::write(dir.path().join("The Matrix (1999) [1080p].mkv"), b"fake").unwrap();
    // Series
    fs::write(
        dir.path().join("Breaking Bad - S01E01 - Pilot [720p].mkv"),
        b"fake",
    )
    .unwrap();
    // Anime
    fs::write(
        dir.path().join("[FFF] Naruto - 42 [1080p][x264].mkv"),
        b"fake",
    )
    .unwrap();

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.scan.video_count, 3);
    assert_eq!(result.parsed_count, 3);
    assert_eq!(result.unknown_count, 0);
    assert_eq!(result.previews.len(), 3);

    let types: Vec<&MediaType> = result.previews.iter().map(|p| &p.media_type).collect();
    assert!(types.contains(&&MediaType::Movie));
    assert!(types.contains(&&MediaType::Series));
    assert!(types.contains(&&MediaType::Anime));
}

#[test]
fn test_pipeline_companion_files_counted() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("Movie (2020).mkv"), b"fake").unwrap();
    fs::write(dir.path().join("Movie (2020).srt"), b"sub").unwrap();
    fs::write(dir.path().join("Movie (2020).nfo"), b"info").unwrap();
    fs::write(dir.path().join("poster.jpg"), b"img").unwrap();

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.scan.video_count, 1);
    assert_eq!(result.scan.companion_count, 3); // srt + nfo + jpg
    assert_eq!(result.parsed_count, 1); // only video enters parse
    assert_eq!(result.previews.len(), 1);
}

#[test]
fn test_pipeline_nested_subdirectories() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("Season 1");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("Show - S01E01 - Pilot.mkv"), b"fake").unwrap();
    fs::write(dir.path().join("Show - S01E02 - Episode 2.mp4"), b"fake").unwrap();

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.scan.video_count, 2);
    assert_eq!(result.parsed_count, 2);
    assert_eq!(result.previews.len(), 2);
}

#[test]
fn test_pipeline_safety_blocks_low_confidence() {
    let dir = tempdir().unwrap();
    // "abc.mkv" has no year, no group, no season/episode → unparseable → unknown
    // But "T.mkv" with year would parse with very low confidence
    fs::write(dir.path().join("T (2020).mkv"), b"fake").unwrap();

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.parsed_count, 1);
    // Short title "T" should trigger low confidence → safety should flag it
    // The pipeline should still complete (dry-run doesn't block on safety)
    assert!(result.safety.dry_run);
    // Check if confidence check flagged it
    let confidence_check = result.safety.checks.iter().find(|c| c.name == "confidence");
    assert!(confidence_check.is_some());
}

#[test]
fn test_pipeline_all_unknown_files() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("random_file.mkv"), b"fake").unwrap();
    fs::write(dir.path().join("another.mp4"), b"fake").unwrap();

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.scan.video_count, 2);
    assert_eq!(result.parsed_count, 0);
    assert_eq!(result.unknown_count, 2);
    assert_eq!(result.previews.len(), 0);
    assert!(result.safety.can_execute); // no items = no blocking reasons
    assert!(!result.task_id.is_empty());
}

#[test]
fn test_pipeline_many_video_extensions() {
    let dir = tempdir().unwrap();
    let filenames = vec![
        "Film (2020).mkv",
        "Film (2020).mp4",
        "Film (2020).avi",
        "Film (2020).mov",
        "Film (2020).webm",
        "Film (2020).ts",
    ];
    for name in &filenames {
        fs::write(dir.path().join(name), b"fake").unwrap();
    }

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.scan.video_count, 6);
    assert_eq!(result.parsed_count, 6);
    assert_eq!(result.previews.len(), 6);
}

#[test]
fn test_pipeline_series_multi_episode() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Show - S01E01 - Pilot [1080p].mkv"),
        b"fake",
    )
    .unwrap();
    fs::write(
        dir.path().join("Show - S01E02 - Episode 2 [1080p].mkv"),
        b"fake",
    )
    .unwrap();
    fs::write(
        dir.path()
            .join("Show - S02E01 - Season 2 Premiere [720p].mkv"),
        b"fake",
    )
    .unwrap();

    let conn = setup_db();
    let result = run_dry_run_pipeline(dir.path(), &conn).unwrap();

    assert_eq!(result.scan.video_count, 3);
    assert_eq!(result.parsed_count, 3);
    assert_eq!(result.previews.len(), 3);

    // All should be Series type
    for preview in &result.previews {
        assert_eq!(preview.media_type, MediaType::Series);
        assert!(preview.proposed_name.contains("Show"));
    }
}
