// 安全门闸检查模块
// 职责：检查安全条件（blockers）
// 不做确认检查，不做执行逻辑

use super::execution_contract::{SafeExecuteInput, SafeExecuteOutput};
use crate::rename::safety_checker;

/// 检查安全门闸
///
/// 检查所有预览项的安全条件
/// 返回 None 表示通过，返回 Some 表示被阻塞
pub fn check_safety_gate(input: &SafeExecuteInput) -> Option<SafeExecuteOutput> {
    let safety_report = safety_checker::check_all(&input.preview_items);
    if !safety_report.can_execute {
        return Some(SafeExecuteOutput {
            allowed: false,
            rejection_reason: Some(format!(
                "安全检查未通过: {}",
                safety_report.blocking_reasons.join("; ")
            )),
            item_results: vec![],
            summary: None,
            rollback_plan: None,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::execution::ExecutionMode;
    use crate::rename::template::{MetadataSource, RenamePreviewItem};
    use crate::scan::MediaItem;

    fn make_safe_item(id: &str) -> RenamePreviewItem {
        RenamePreviewItem {
            id: id.to_string(),
            parsed_info: ParsedMediaInfo {
                media_item: MediaItem {
                    id: format!("media-{}", id),
                    file_path: "/test.mkv".to_string(),
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
            source_path: "/test.mkv".to_string(),
            original_name: "test.mkv".to_string(),
            proposed_name: "Test Movie (2020).mkv".to_string(),
            target_path: "/Test Movie (2020).mkv".to_string(),
            media_type: MediaType::Movie,
            confidence: 90,
            needs_manual_review: false,
            should_skip: false,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    fn make_unsafe_item(id: &str) -> RenamePreviewItem {
        let mut item = make_safe_item(id);
        item.confidence = 30; // 低置信度触发安全检查
        item
    }

    #[test]
    fn test_safe_items_pass() {
        let input = SafeExecuteInput {
            task_id: "test".to_string(),
            preview_items: vec![make_safe_item("1")],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        assert!(check_safety_gate(&input).is_none());
    }

    #[test]
    fn test_unsafe_items_blocked() {
        let input = SafeExecuteInput {
            task_id: "test".to_string(),
            preview_items: vec![make_unsafe_item("1")],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = check_safety_gate(&input).unwrap();
        assert!(!output.allowed);
        assert!(output.rejection_reason.unwrap().contains("安全检查未通过"));
    }

    #[test]
    fn test_empty_items_pass() {
        let input = SafeExecuteInput {
            task_id: "test".to_string(),
            preview_items: vec![],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        assert!(check_safety_gate(&input).is_none());
    }
}
