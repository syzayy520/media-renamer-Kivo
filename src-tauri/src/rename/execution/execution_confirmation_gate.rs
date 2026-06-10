// 确认门闸模块
// 职责：检查用户确认状态（Confirmed 模式必须确认）
// 不做安全检查，不做执行逻辑

use super::execution_contract::{SafeExecuteInput, SafeExecuteOutput};
use super::execution_mode::ExecutionMode;

/// 检查用户确认
///
/// Confirmed 模式必须 user_confirmed = true
/// DryRun 模式不需要确认
pub fn check_user_confirmation(input: &SafeExecuteInput) -> Option<SafeExecuteOutput> {
    if input.mode == ExecutionMode::Confirmed && !input.user_confirmed {
        return Some(SafeExecuteOutput {
            allowed: false,
            rejection_reason: Some("Confirmed 模式需要用户确认".to_string()),
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

    #[test]
    fn test_dry_run_no_confirmation_needed() {
        let input = SafeExecuteInput {
            task_id: "test".to_string(),
            preview_items: vec![],
            mode: ExecutionMode::DryRun,
            user_confirmed: false,
        };
        assert!(check_user_confirmation(&input).is_none());
    }

    #[test]
    fn test_confirmed_with_confirmation_passes() {
        let input = SafeExecuteInput {
            task_id: "test".to_string(),
            preview_items: vec![],
            mode: ExecutionMode::Confirmed,
            user_confirmed: true,
        };
        assert!(check_user_confirmation(&input).is_none());
    }

    #[test]
    fn test_confirmed_without_confirmation_blocked() {
        let input = SafeExecuteInput {
            task_id: "test".to_string(),
            preview_items: vec![],
            mode: ExecutionMode::Confirmed,
            user_confirmed: false,
        };
        let output = check_user_confirmation(&input).unwrap();
        assert!(!output.allowed);
        assert!(output.rejection_reason.unwrap().contains("用户确认"));
    }
}
