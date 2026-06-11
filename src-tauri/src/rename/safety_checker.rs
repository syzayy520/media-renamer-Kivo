// 安全检查模块
// 职责：检查重命名预览项的安全性

use super::conflict_detector;
use super::template::RenamePreviewItem;
use crate::parse::confidence::DEFAULT_CONFIDENCE_THRESHOLD;
use crate::shared::path_utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyReport {
    pub can_execute: bool,
    pub dry_run: bool,
    pub checks: Vec<SafetyCheck>,
    pub blocking_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

pub fn check_all(preview_items: &[RenamePreviewItem]) -> SafetyReport {
    let mut checks = Vec::new();
    let mut blocking_reasons = Vec::new();

    let low_confidence_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| !item.should_skip && item.confidence < DEFAULT_CONFIDENCE_THRESHOLD)
        .collect();

    if low_confidence_items.is_empty() {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: true,
            message: "All executable items have sufficient confidence".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: false,
            message: format!(
                "{} executable items have low confidence (< {})",
                low_confidence_items.len(),
                DEFAULT_CONFIDENCE_THRESHOLD
            ),
        });
        blocking_reasons.push(format!("Low confidence items: {}", low_confidence_items.len()));
    }

    let manual_review_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| !item.should_skip && item.needs_manual_review)
        .collect();

    checks.push(SafetyCheck {
        name: "manual_review".to_string(),
        passed: true,
        message: if manual_review_items.is_empty() {
            "No executable items require manual review".to_string()
        } else {
            format!(
                "{} executable items need review; this is a warning and does not block execution",
                manual_review_items.len()
            )
        },
    });

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

    let invalid_char_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| !item.should_skip && path_utils::has_invalid_chars(&item.target_path))
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
            message: format!("{} executable items have invalid characters", invalid_char_items.len()),
        });
        blocking_reasons.push(format!("Invalid characters: {}", invalid_char_items.len()));
    }

    let long_path_items: Vec<&RenamePreviewItem> = preview_items
        .iter()
        .filter(|item| !item.should_skip && path_utils::is_path_too_long(&item.target_path))
        .collect();

    if long_path_items.is_empty() {
        checks.push(SafetyCheck {
            name: "path_length".to_string(),
            passed: true,
            message: "All executable paths are within length limits".to_string(),
        });
    } else {
        checks.push(SafetyCheck {
            name: "path_length".to_string(),
            passed: false,
            message: format!("{} executable items have paths too long", long_path_items.len()),
        });
        blocking_reasons.push(format!("Paths too long: {}", long_path_items.len()));
    }

    let skipped_items: Vec<&RenamePreviewItem> = preview_items.iter().filter(|item| item.should_skip).collect();
    checks.push(SafetyCheck {
        name: "skipped".to_string(),
        passed: true,
        message: format!("{} items marked as skipped", skipped_items.len()),
    });

    SafetyReport {
        can_execute: blocking_reasons.is_empty(),
        dry_run: true,
        checks,
        blocking_reasons,
    }
}

pub fn check_single(item: &RenamePreviewItem) -> Vec<SafetyCheck> {
    let mut checks = Vec::new();

    if item.confidence < DEFAULT_CONFIDENCE_THRESHOLD {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: false,
            message: format!("Low confidence: {} < {}", item.confidence, DEFAULT_CONFIDENCE_THRESHOLD),
        });
    } else {
        checks.push(SafetyCheck {
            name: "confidence".to_string(),
            passed: true,
            message: format!("Confidence: {}", item.confidence),
        });
    }

    checks.push(SafetyCheck {
        name: "manual_review".to_string(),
        passed: true,
        message: if item.needs_manual_review {
            "Needs manual review warning".to_string()
        } else {
            "No manual review required".to_string()
        },
    });

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
