// 安全检查模块
// 职责：检查重命名预览项的安全性

use super::conflict_detector;
use super::template::RenamePreviewItem;
use crate::parse::confidence::DEFAULT_CONFIDENCE_THRESHOLD;
use crate::shared::path_utils;
use serde::{Deserialize, Serialize};

/// 安全检查报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyReport {
    /// 是否可以执行
    pub can_execute: bool,
    /// 是否为预演模式（默认 true）
    pub dry_run: bool,
    /// 检查结果列表
    pub checks: Vec<SafetyCheck>,
    /// 阻塞原因列表
    pub blocking_reasons: Vec<String>,
}

/// 单项安全检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCheck {
    /// 检查名称
    pub name: String,
    /// 是否通过
    pub passed: bool,
    /// 消息
    pub message: String,
}

/// 执行所有安全检查
pub fn check_all(preview_items: &[RenamePreviewItem]) -> SafetyReport {
    let mut checks = Vec::new();
    let mut blocking_reasons = Vec::new();

    // 检查置信度
    let low_confidence_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| item.confidence < DEFAULT_CONFIDENCE_THRESHOLD)
        .collect();

    if low_confidence_items.is_empty() {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: true,
            message: "All items have sufficient confidence".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: false,
            message: format!(
                "{} items have low confidence (< {})",
                low_confidence_items.len(),
                DEFAULT_CONFIDENCE_THRESHOLD
            ),
        });
        blocking_reasons.push(format!(
            "Low confidence items: {}",
            low_confidence_items.len()
        ));
    }

    // 检查人工确认
    let manual_review_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| item.needs_manual_review)
        .collect();

    if manual_review_items.is_empty() {
        checks.push(SafetyCheck {
            name: "manual_review".to_string(),
            passed: true,
            message: "No items require manual review".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "manual_review".to_string(),
            passed: false,
            message: format!("{} items require manual review", manual_review_items.len()),
        });
        blocking_reasons.push(format!(
            "Manual review required: {}",
            manual_review_items.len()
        ));
    }

    // 检查冲突
    let conflicts = conflict_detector::detect_conflicts(preview_items);
    let blocking_conflicts: Vec<_> = conflicts.iter().filter(|c| c.blocking).collect();

    if blocking_conflicts.is_empty() {
        checks.push(SafetyCheck {
            name: "conflicts".to_string(),
            passed: true,
            message: "No blocking conflicts detected".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "conflicts".to_string(),
            passed: false,
            message: format!("{} blocking conflicts detected", blocking_conflicts.len()),
        });
        blocking_reasons.push(format!("Blocking conflicts: {}", blocking_conflicts.len()));
    }

    // 检查非法字符
    let invalid_char_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| path_utils::has_invalid_chars(&item.target_path))
        .collect();

    if invalid_char_items.is_empty() {
        checks.push(SafetyCheck {
            name: "invalid_chars".to_string(),
            passed: true,
            message: "No invalid characters in target paths".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "invalid_chars".to_string(),
            passed: false,
            message: format!("{} items have invalid characters", invalid_char_items.len()),
        });
        blocking_reasons.push(format!("Invalid characters: {}", invalid_char_items.len()));
    }

    // 检查路径过长
    let long_path_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| path_utils::is_path_too_long(&item.target_path))
        .collect();

    if long_path_items.is_empty() {
        checks.push(SafetyCheck {
            name: "path_length".to_string(),
            passed: true,
            message: "All paths are within length limits".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "path_length".to_string(),
            passed: false,
            message: format!("{} items have paths too long", long_path_items.len()),
        });
        blocking_reasons.push(format!("Paths too long: {}", long_path_items.len()));
    }

    // 检查跳过项
    let skipped_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| item.should_skip)
        .collect();

    checks.push(SafetyCheck {
        name: "skipped".to_string(),
        passed: true,
        message: format!("{} items marked as skipped", skipped_items.len()),
    });

    SafetyReport {
        can_execute: blocking_reasons.is_empty(),
        dry_run: true, // 默认为预演模式
        checks,
        blocking_reasons,
    }
}

/// 检查单个项是否安全
pub fn check_single(item: &RenamePreviewItem) -> Vec<SafetyCheck> {
    let mut checks = Vec::new();

    // 置信度检查
    if item.confidence < DEFAULT_CONFIDENCE_THRESHOLD {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: false,
            message: format!(
                "Low confidence: {} < {}",
                item.confidence, DEFAULT_CONFIDENCE_THRESHOLD
            ),
        });
    } else {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: true,
            message: format!("Confidence: {}", item.confidence),
        });
    }

    // 人工确认检查
    if item.needs_manual_review {
        checks.push(SafetyCheck {
            name: "manual_review".to_string(),
            passed: false,
            message: "Requires manual review".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "manual_review".to_string(),
            passed: true,
            message: "No manual review required".to_string(),
        });
    }

    // 非法字符检查
    if path_utils::has_invalid_chars(&item.target_path) {
        checks.push(SafetyCheck {
            name: "invalid_chars".to_string(),
            passed: false,
            message: format!("Invalid characters in: {}", item.target_path),
        });
    } else {
        checks.push(SafetyCheck {
            name: "invalid_chars".to_string(),
            passed: true,
            message: "No invalid characters".to_string(),
        });
    }

    // 路径长度检查
    if path_utils::is_path_too_long(&item.target_path) {
        checks.push(SafetyCheck {
            name: "path_length".to_string(),
            passed: false,
            message: format!("Path too long: {}", item.target_path.len()),
        });
    } else {
        checks.push(SafetyCheck {
            name: "path_length".to_string(),
            passed: true,
            message: "Path length OK".to_string(),
        });
    }

    checks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::movie_parser::MediaType;
    use crate::rename::template::{MetadataSource, RenameConflict};
    use crate::scan::MediaItem;

    fn make_test_item(
        confidence: u8,
        needs_review: bool,
        should_skip: bool,
        target: &str,
    ) -> RenamePreviewItem {
        RenamePreviewItem {
            id: "test-id".to_string(),
            parsed_info: crate::parse::movie_parser::ParsedMediaInfo {
                media_item: MediaItem {
                    id: "test".to_string(),
                    file_path: "C:\\test\\file.mkv".to_string(),
                    file_name: "file.mkv".to_string(),
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
                resolution: None,
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
            source_path: "C:\\test\\file.mkv".to_string(),
            original_name: "file.mkv".to_string(),
            proposed_name: "new.mkv".to_string(),
            target_path: target.to_string(),
            media_type: MediaType::Movie,
            confidence,
            needs_manual_review: needs_review,
            should_skip,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }

    #[test]
    fn test_check_all_safe_items() {
        let item = make_test_item(85, false, false, "C:\\test\\new.mkv");
        let report = check_all(&[item]);
        assert!(report.can_execute);
        assert!(report.dry_run);
        assert!(report.blocking_reasons.is_empty());
    }

    #[test]
    fn test_check_all_low_confidence_blocks() {
        let item = make_test_item(50, false, false, "C:\\test\\new.mkv");
        let report = check_all(&[item]);
        assert!(!report.can_execute);
        assert!(report
            .blocking_reasons
            .iter()
            .any(|r| r.contains("Low confidence")));
    }

    #[test]
    fn test_check_all_manual_review_blocks() {
        let item = make_test_item(85, true, false, "C:\\test\\new.mkv");
        let report = check_all(&[item]);
        assert!(!report.can_execute);
        assert!(report
            .blocking_reasons
            .iter()
            .any(|r| r.contains("Manual review")));
    }

    #[test]
    fn test_check_all_conflict_blocks() {
        let mut item = make_test_item(85, false, false, "C:\\test\\file.mkv"); // 目标等于源
        item.conflicts.push(RenameConflict {
            conflict_type: crate::shared::result_types::ConflictType::TargetExists,
            source_path: "C:\\test\\file.mkv".to_string(),
            target_path: "C:\\test\\file.mkv".to_string(),
            message: "Target exists".to_string(),
            blocking: true,
        });
        let report = check_all(&[item]);
        assert!(!report.can_execute);
        assert!(report
            .blocking_reasons
            .iter()
            .any(|r| r.contains("Blocking conflicts")));
    }

    #[test]
    fn test_check_all_invalid_chars_blocks() {
        let item = make_test_item(85, false, false, "C:\\test\\new<file>.mkv");
        let report = check_all(&[item]);
        assert!(!report.can_execute);
        assert!(report
            .blocking_reasons
            .iter()
            .any(|r| r.contains("Invalid characters")));
    }

    #[test]
    fn test_check_all_path_too_long_blocks() {
        let long_path = format!("C:\\test\\{}.mkv", "a".repeat(260));
        let item = make_test_item(85, false, false, &long_path);
        let report = check_all(&[item]);
        assert!(!report.can_execute);
        assert!(report
            .blocking_reasons
            .iter()
            .any(|r| r.contains("Paths too long")));
    }

    #[test]
    fn test_check_all_safe_items_pass() {
        let item1 = make_test_item(85, false, false, "C:\\test\\new1.mkv");
        let item2 = make_test_item(90, false, false, "C:\\test\\new2.mkv");
        let report = check_all(&[item1, item2]);
        assert!(report.can_execute);
        assert_eq!(report.checks.len(), 6); // confidence, manual_review, conflicts, invalid_chars, path_length, skipped
    }

    #[test]
    fn test_check_single_safe() {
        let item = make_test_item(85, false, false, "C:\\test\\new.mkv");
        let checks = check_single(&item);
        assert!(checks.iter().all(|c| c.passed));
    }

    #[test]
    fn test_check_single_low_confidence() {
        let item = make_test_item(50, false, false, "C:\\test\\new.mkv");
        let checks = check_single(&item);
        let confidence_check = checks.iter().find(|c| c.name == "confidence").unwrap();
        assert!(!confidence_check.passed);
    }
}
