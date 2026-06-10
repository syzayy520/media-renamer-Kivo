// 执行模式模块
// 职责：定义 ExecutionMode 枚举
// 不做执行逻辑，不做安全检查

use serde::{Deserialize, Serialize};

/// 执行模式
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// 预演模式（默认），不执行实际文件操作
    #[default]
    DryRun,
    /// 确认模式，执行真实重命名
    Confirmed,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
