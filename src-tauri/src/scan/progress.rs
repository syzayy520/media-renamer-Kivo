// 扫描进度模块
// 职责：管理扫描进度状态

use serde::{Deserialize, Serialize};

/// 扫描进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    /// 已扫描文件数
    pub scanned: u32,
    /// 总文件数（估计值）
    pub total: u32,
    /// 当前扫描的文件路径
    pub current_file: String,
}

impl ScanProgress {
    pub fn new() -> Self {
        Self {
            scanned: 0,
            total: 0,
            current_file: String::new(),
        }
    }

    pub fn with_total(total: u32) -> Self {
        Self {
            scanned: 0,
            total,
            current_file: String::new(),
        }
    }

    pub fn update(&mut self, current_file: String) {
        self.scanned += 1;
        self.current_file = current_file;
    }

    pub fn progress_percent(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        (self.scanned as f32 / self.total as f32) * 100.0
    }
}

impl Default for ScanProgress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_percent() {
        let mut progress = ScanProgress::with_total(100);
        assert_eq!(progress.progress_percent(), 0.0);

        progress.update("file1.txt".to_string());
        assert_eq!(progress.progress_percent(), 1.0);

        progress.update("file2.txt".to_string());
        assert_eq!(progress.progress_percent(), 2.0);
    }

    #[test]
    fn test_progress_zero_total() {
        let progress = ScanProgress::new();
        assert_eq!(progress.progress_percent(), 0.0);
    }
}
