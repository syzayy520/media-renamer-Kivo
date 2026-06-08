// threshold 模块 - 置信度阈值管理
// 职责：置信度阈值 get/set（委托 config_loader 持久化）

use crate::config::config_loader::{load_config, save_config};
use crate::shared::result_types::AppResult;
use std::path::Path;

/// 默认置信度阈值
pub const DEFAULT_CONFIDENCE_THRESHOLD: u8 = 70;

/// 获取当前置信度阈值
pub fn get_threshold(config_dir: &Path) -> AppResult<u8> {
    let config = load_config(config_dir)?;
    Ok(config.thresholds.confidence)
}

/// 设置置信度阈值
pub fn set_threshold(value: u8, config_dir: &Path) -> AppResult<()> {
    let mut config = load_config(config_dir)?;
    config.thresholds.confidence = value;
    save_config(&config, config_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_dir() -> PathBuf {
        tempfile::tempdir().unwrap().keep()
    }

    #[test]
    fn test_get_default_threshold() {
        let dir = temp_dir();
        let val = get_threshold(&dir).unwrap();
        assert_eq!(val, DEFAULT_CONFIDENCE_THRESHOLD);
    }

    #[test]
    fn test_set_and_get_threshold() {
        let dir = temp_dir();
        set_threshold(85, &dir).unwrap();
        let val = get_threshold(&dir).unwrap();
        assert_eq!(val, 85);
    }

    #[test]
    fn test_threshold_boundary_zero() {
        let dir = temp_dir();
        set_threshold(0, &dir).unwrap();
        assert_eq!(get_threshold(&dir).unwrap(), 0);
    }

    #[test]
    fn test_threshold_boundary_hundred() {
        let dir = temp_dir();
        set_threshold(100, &dir).unwrap();
        assert_eq!(get_threshold(&dir).unwrap(), 100);
    }
}
