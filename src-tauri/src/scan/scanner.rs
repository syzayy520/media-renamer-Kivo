// 扫描器模块
// 职责：目录遍历、文件识别、权限检查

use super::file_detector;
use crate::shared::result_types::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

/// 媒体文件项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    /// 唯一标识
    pub id: String,
    /// 文件完整路径
    pub file_path: String,
    /// 文件名（含扩展名）
    pub file_name: String,
    /// 扩展名
    pub extension: String,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 是否视频文件
    pub is_video: bool,
    /// 是否伴随文件
    pub is_companion: bool,
}

impl MediaItem {
    pub fn new(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);

        let is_video = file_detector::is_video(&extension);
        let is_companion = file_detector::is_companion(&extension);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            file_path: path.to_string_lossy().to_string(),
            file_name,
            extension,
            file_size,
            is_video,
            is_companion,
        }
    }
}

/// 扫描报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    /// 扫描目录
    pub scan_path: String,
    /// 总文件数
    pub total_files: u32,
    /// 视频文件数
    pub video_count: u32,
    /// 伴随文件数
    pub companion_count: u32,
    /// 权限错误列表
    pub permission_errors: Vec<String>,
    /// 扫描耗时（毫秒）
    pub scan_duration_ms: u64,
}

impl ScanReport {
    pub fn new(scan_path: String) -> Self {
        Self {
            scan_path,
            total_files: 0,
            video_count: 0,
            companion_count: 0,
            permission_errors: Vec::new(),
            scan_duration_ms: 0,
        }
    }
}

/// 扫描配置
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// 最大文件数量限制
    pub max_files: u32,
    /// 是否跟随符号链接
    pub follow_links: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            max_files: 100_000,
            follow_links: false,
        }
    }
}

/// 扫描目录
pub fn scan_directory(path: &Path) -> AppResult<(ScanReport, Vec<MediaItem>)> {
    scan_directory_with_config(path, ScanConfig::default())
}

/// 使用配置扫描目录
pub fn scan_directory_with_config(
    path: &Path,
    config: ScanConfig,
) -> AppResult<(ScanReport, Vec<MediaItem>)> {
    if !path.exists() {
        return Err(AppError::Io(format!(
            "Path does not exist: {}",
            path.display()
        )));
    }

    if !path.is_dir() {
        return Err(AppError::Io(format!(
            "Path is not a directory: {}",
            path.display()
        )));
    }

    let start_time = std::time::Instant::now();
    let mut report = ScanReport::new(path.to_string_lossy().to_string());
    let mut items = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(config.follow_links)
        .into_iter()
    {
        // 检查文件数量限制
        if items.len() >= config.max_files as usize {
            return Err(AppError::MaxFileLimitReached {
                limit: config.max_files,
            });
        }

        match entry {
            Ok(entry) => {
                let file_path = entry.path();

                // 只处理文件
                if !file_path.is_file() {
                    continue;
                }

                // 获取扩展名
                if let Some(ext) = file_path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();

                    // MVP：只扫描视频文件进入重命名主链路
                    // 伴随文件（字幕、海报等）只识别统计，不进入自动重命名
                    if file_detector::is_video(&ext_str) {
                        let item = MediaItem::new(file_path);
                        report.video_count += 1;
                        report.total_files += 1;
                        items.push(item);
                    } else if file_detector::is_companion(&ext_str) {
                        // 伴随文件只统计，不进入重命名主链路
                        report.companion_count += 1;
                    }
                }
            }
            Err(e) => {
                // 记录权限错误，继续扫描
                report.permission_errors.push(e.to_string());
            }
        }
    }

    report.scan_duration_ms = start_time.elapsed().as_millis() as u64;

    Ok((report, items))
}

/// 取消扫描的标记
use std::sync::atomic::{AtomicBool, Ordering};

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

/// 取消扫描
pub fn cancel_scan() {
    CANCEL_FLAG.store(true, Ordering::Relaxed);
}

/// 重置取消标记
pub fn reset_cancel_flag() {
    CANCEL_FLAG.store(false, Ordering::Relaxed);
}

/// 检查是否已取消
pub fn is_cancelled() -> bool {
    CANCEL_FLAG.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_scan_empty_directory() {
        let dir = tempdir().unwrap();
        let (report, items) = scan_directory(dir.path()).unwrap();
        assert_eq!(items.len(), 0);
        assert_eq!(report.total_files, 0);
    }

    #[test]
    fn test_scan_with_video_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.mkv"), b"test").unwrap();
        fs::write(dir.path().join("test.mp4"), b"test").unwrap();
        fs::write(dir.path().join("test.txt"), b"test").unwrap();
        fs::write(dir.path().join("test.srt"), b"test").unwrap();
        fs::write(dir.path().join("test.nfo"), b"test").unwrap();

        let (report, items) = scan_directory(dir.path()).unwrap();
        // MVP：只有视频文件进入重命名主链路
        assert_eq!(items.len(), 2); // 2 video files
        assert_eq!(report.video_count, 2);
        // 伴随文件只统计，不进入重命名
        assert_eq!(report.companion_count, 3); // .txt, .srt, .nfo
        assert_eq!(report.total_files, 2); // total_files = video files only
                                           // 验证所有 items 都是视频文件
        assert!(items.iter().all(|item| item.is_video));
        assert!(items.iter().all(|item| !item.is_companion));
    }

    #[test]
    fn test_scan_nonexistent_path() {
        let path = PathBuf::from("/nonexistent/path");
        let result = scan_directory(&path);
        assert!(result.is_err());
    }
}
