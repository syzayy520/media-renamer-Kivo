// 回滚计划模块
// 职责：生成回滚计划，记录每个重命名操作的 source/target/rollback_source/rollback_target
// 不做回滚执行，不做审计写入

use chrono::Utc;
use serde::Serialize;

use crate::rename::template::RenamePreviewItem;

/// 回滚计划
#[derive(Debug, Clone, Serialize)]
pub struct RollbackPlan {
    /// 任务 ID
    pub task_id: String,
    /// 回滚条目列表
    pub entries: Vec<RollbackPlanEntry>,
    /// 生成时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 回滚计划条目
#[derive(Debug, Clone, Serialize)]
pub struct RollbackPlanEntry {
    /// 源文件路径（重命名前）
    pub source_path: String,
    /// 目标文件路径（重命名后）
    pub target_path: String,
    /// 回滚源路径（回滚时的源，即 target_path）
    pub rollback_source: String,
    /// 回滚目标路径（回滚时的目标，即 source_path）
    pub rollback_target: String,
    /// 是否已执行
    pub executed: bool,
}

impl RollbackPlan {
    /// 从预览项列表生成回滚计划
    pub fn from_preview_items(task_id: &str, items: &[RenamePreviewItem]) -> Self {
        let entries = items
            .iter()
            .map(|item| RollbackPlanEntry {
                source_path: item.source_path.clone(),
                target_path: item.target_path.clone(),
                rollback_source: item.target_path.clone(),
                rollback_target: item.source_path.clone(),
                executed: false,
            })
            .collect();

        Self {
            task_id: task_id.to_string(),
            entries,
            created_at: Utc::now(),
        }
    }

    /// 标记条目为已执行
    pub fn mark_executed(&mut self, source: &str, target: &str) {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|e| e.source_path == source && e.target_path == target)
        {
            entry.executed = true;
        }
    }

    /// 获取已执行条目数量
    pub fn executed_count(&self) -> usize {
        self.entries.iter().filter(|e| e.executed).count()
    }

    /// 获取未执行条目数量
    pub fn pending_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.executed).count()
    }

    /// 检查是否有可回滚的条目
    pub fn has_rollbackable_entries(&self) -> bool {
        self.entries.iter().any(|e| e.executed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::template::MetadataSource;
    use crate::scan::MediaItem;

    fn make_preview_item(source: &str, target: &str) -> RenamePreviewItem {
        RenamePreviewItem {
            id: "test-1".to_string(),
            parsed_info: ParsedMediaInfo {
                media_item: MediaItem {
                    id: "media-1".to_string(),
                    file_path: source.to_string(),
                    file_name: "test.mkv".to_string(),
                    extension: "mkv".to_string(),
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: MediaType::Movie,
                title: "Test".to_string(),
                year: None,
                season: None,
                episode: None,
                episode_end: None,
                episode_title: None,
                resolution: None,
                source: None,
                video_codec: None,
                audio_codec: None,
                group: None,
                confidence: 80,
                rule_sources: vec![],
                special_type: None,
                extra_type: None,
                extra_number: None,
            },
            source_path: source.to_string(),
            original_name: "test.mkv".to_string(),
            proposed_name: "Test.mkv".to_string(),
            target_path: target.to_string(),
            media_type: MediaType::Movie,
            confidence: 80,
            needs_manual_review: false,
            should_skip: false,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    #[test]
    fn test_rollback_plan_from_preview_items() {
        let items = vec![
            make_preview_item("/source1.mkv", "/target1.mkv"),
            make_preview_item("/source2.mkv", "/target2.mkv"),
        ];
        let plan = RollbackPlan::from_preview_items("task-1", &items);

        assert_eq!(plan.task_id, "task-1");
        assert_eq!(plan.entries.len(), 2);
        assert_eq!(plan.entries[0].source_path, "/source1.mkv");
        assert_eq!(plan.entries[0].target_path, "/target1.mkv");
        assert_eq!(plan.entries[0].rollback_source, "/target1.mkv");
        assert_eq!(plan.entries[0].rollback_target, "/source1.mkv");
        assert!(!plan.entries[0].executed);
    }

    #[test]
    fn test_rollback_plan_mark_executed() {
        let items = vec![make_preview_item("/source.mkv", "/target.mkv")];
        let mut plan = RollbackPlan::from_preview_items("task-1", &items);

        assert!(!plan.has_rollbackable_entries());
        plan.mark_executed("/source.mkv", "/target.mkv");
        assert!(plan.has_rollbackable_entries());
        assert_eq!(plan.executed_count(), 1);
        assert_eq!(plan.pending_count(), 0);
    }

    #[test]
    fn test_rollback_plan_mark_executed_not_found() {
        let items = vec![make_preview_item("/source.mkv", "/target.mkv")];
        let mut plan = RollbackPlan::from_preview_items("task-1", &items);

        plan.mark_executed("/nonexistent.mkv", "/nonexistent2.mkv");
        assert!(!plan.has_rollbackable_entries());
    }

    #[test]
    fn test_rollback_plan_empty() {
        let plan = RollbackPlan::from_preview_items("task-1", &[]);
        assert_eq!(plan.entries.len(), 0);
        assert!(!plan.has_rollbackable_entries());
        assert_eq!(plan.executed_count(), 0);
        assert_eq!(plan.pending_count(), 0);
    }
}
