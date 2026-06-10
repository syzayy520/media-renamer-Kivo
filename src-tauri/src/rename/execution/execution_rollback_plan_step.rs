// 回滚计划步骤模块
// 职责：生成回滚计划
// 不做执行逻辑，不做安全检查

use crate::rename::template::RenamePreviewItem;
use crate::rollback::rollback_plan::RollbackPlan;

/// 生成回滚计划
///
/// 从预览项列表生成回滚计划
/// 回滚计划在执行前生成，用于记录 source/target 映射
pub fn generate_rollback_plan(task_id: &str, preview_items: &[RenamePreviewItem]) -> RollbackPlan {
    RollbackPlan::from_preview_items(task_id, preview_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::template::MetadataSource;
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
    fn test_generate_plan_from_items() {
        let items = vec![
            make_item("1", "/s1.mkv", "/t1.mkv"),
            make_item("2", "/s2.mkv", "/t2.mkv"),
        ];
        let plan = generate_rollback_plan("task-1", &items);
        assert_eq!(plan.entries.len(), 2);
        assert_eq!(plan.entries[0].source_path, "/s1.mkv");
        assert_eq!(plan.entries[0].rollback_target, "/s1.mkv");
    }

    #[test]
    fn test_generate_plan_empty() {
        let plan = generate_rollback_plan("task-1", &[]);
        assert_eq!(plan.entries.len(), 0);
    }
}
