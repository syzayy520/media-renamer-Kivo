// execution_commands 模块 - 安全执行相关 Tauri 命令
// 职责：将 rename::execution::safe_executor 暴露为 Tauri 命令
// 不做业务逻辑，只做参数转换 + 错误映射

use crate::rename::execution::safe_executor::safe_execute;
use crate::rename::execution::{SafeExecuteInput, SafeExecuteOutput};

/// 安全执行重命名（Tauri 命令）
///
/// 主流程：
/// 1. 检查用户确认（Confirmed 模式）
/// 2. 检查安全门闸（blockers）
/// 3. 生成回滚计划
/// 4. 执行重命名（DryRun 或 Confirmed）
/// 5. 返回结果
///
/// # Arguments
/// * `input` - 包含任务 ID、预览项列表、执行模式、用户确认状态
///
/// # Returns
/// `SafeExecuteOutput` 包含执行结果、摘要、回滚计划
#[tauri::command]
pub fn safe_execute_rename(input: SafeExecuteInput) -> Result<SafeExecuteOutput, String> {
    Ok(safe_execute(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
    use crate::rename::execution::ExecutionMode;
    use crate::rename::template::MetadataSource;
    use crate::scan::MediaItem;

    fn make_safe_item(
        id: &str,
        source: &str,
        target: &str,
    ) -> crate::rename::template::RenamePreviewItem {
        crate::rename::template::RenamePreviewItem {
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
    fn test_safe_execute_rename_dry_run() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![make_safe_item("1", "/s1.mkv", "/t1.mkv")],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = safe_execute_rename(input).unwrap();
        assert!(output.allowed);
        assert!(output.rollback_plan.is_some());
    }

    #[test]
    fn test_safe_execute_rename_confirmed_requires_user_confirmation() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![make_safe_item("1", "/s1.mkv", "/t1.mkv")],
            mode: ExecutionMode::Confirmed,
            user_confirmed: false,
        };
        let output = safe_execute_rename(input).unwrap();
        assert!(!output.allowed);
        assert!(output.rejection_reason.unwrap().contains("用户确认"));
    }

    #[test]
    fn test_safe_execute_rename_empty_items() {
        let input = SafeExecuteInput {
            task_id: "test-task".to_string(),
            preview_items: vec![],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        let output = safe_execute_rename(input).unwrap();
        assert!(output.allowed);
        assert_eq!(output.summary.unwrap().total, 0);
    }
}
