// DryRun 执行器模块
// 职责：模拟执行（不修改文件系统）
// 不做真实重命名，不做安全检查

use super::execution_contract::ExecutionItemResult;
use crate::rename::template::RenamePreviewItem;

/// 执行 DryRun
///
/// 对每个非阻塞项返回模拟成功结果
/// 不执行任何文件系统操作
pub fn execute_dry_run(items: &[&RenamePreviewItem]) -> Vec<ExecutionItemResult> {
    items
        .iter()
        .map(|item| ExecutionItemResult::success(&item.source_path, &item.target_path))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::template::MetadataSource;
    use crate::rename::execution::ExecutionItemStatus;
    use crate::scan::MediaItem;

    fn make_item(id: &str, source: &str, target: &str) -> RenamePreviewItem {
        RenamePreviewItem {
            id: id.to_string(),
            parsed_info: ParsedMediaInfo {
                media_item: MediaItem {
                    id: format!("media-{}", id),
                    file_path: source.to_string(),
                    file_name: "test.mkv".to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: MediaType::Movie,
                title: "Test Movie".to_string(),
                year: Some(2020),
                season: None,
                episode: None,
                episode_end: None,
                episode_title: None,
                resolution: None,
                source: None,
                video_codec: None,
                audio_codec: None,
                group: None,
                confidence: 90,
                rule_sources: vec![],
                special_type: None,
                extra_type: None,
                extra_number: None,
            },
            source_path: source.to_string(),
            original_name: "test.mkv".to_string(),
            proposed_name: "Test Movie (2020).mkv".to_string(),
            target_path: target.to_string(),
            media_type: MediaType::Movie,
            confidence: 90,
            needs_manual_review: false,
            should_skip: false,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    #[test]
    fn test_dry_run_returns_success() {
        let item = make_item("1", "/s1.mkv", "/t1.mkv");
        let items = vec![&item];
        let results = execute_dry_run(&items);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, ExecutionItemStatus::Success);
    }

    #[test]
    fn test_dry_run_multiple_items() {
        let item1 = make_item("1", "/s1.mkv", "/t1.mkv");
        let item2 = make_item("2", "/s2.mkv", "/t2.mkv");
        let items = vec![&item1, &item2];
        let results = execute_dry_run(&items);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.status == ExecutionItemStatus::Success));
    }

    #[test]
    fn test_dry_run_empty() {
        let empty: Vec<&RenamePreviewItem> = vec![];
        let results = execute_dry_run(&empty);
        assert_eq!(results.len(), 0);
    }
}
