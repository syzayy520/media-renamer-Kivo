// ExecutionMode 测试
// 职责：测试 ExecutionMode 枚举

use app_lib::rename::execution::ExecutionMode;

#[test]
fn test_default_is_dry_run() {
    let mode = ExecutionMode::default();
    assert_eq!(mode, ExecutionMode::DryRun);
}

#[test]
fn test_mode_equality() {
    assert_eq!(ExecutionMode::DryRun, ExecutionMode::DryRun);
    assert_eq!(ExecutionMode::Confirmed, ExecutionMode::Confirmed);
    assert_ne!(ExecutionMode::DryRun, ExecutionMode::Confirmed);
}

#[test]
fn test_mode_clone() {
    let mode = ExecutionMode::Confirmed;
    let cloned = mode;
    assert_eq!(mode, cloned);
}