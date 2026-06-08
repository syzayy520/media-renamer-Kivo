// 平台适配模块
// 职责：平台特定的路径处理和文件操作

use std::path::Path;

/// 检查文件是否可写
pub fn is_writable(path: &Path) -> bool {
    // 在 Windows 上，检查文件属性
    #[cfg(target_os = "windows")]
    {
        use std::fs;
        if let Ok(metadata) = fs::metadata(path) {
            return !metadata.permissions().readonly();
        }
        false
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::MetadataExt;
        if let Ok(metadata) = std::fs::metadata(path) {
            return metadata.mode() & 0o200 != 0;
        }
        false
    }
}

/// 获取文件大小
pub fn get_file_size(path: &Path) -> u64 {
    std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

/// 检查路径是否存在
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// 获取文件扩展名（小写）
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
}

/// 获取文件名（不含扩展名）
pub fn get_stem(path: &Path) -> Option<String> {
    path.file_stem()
        .map(|stem| stem.to_string_lossy().to_string())
}

/// 获取完整文件名（含扩展名）
pub fn get_filename(path: &Path) -> Option<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_extension() {
        let path = PathBuf::from("test.mkv");
        assert_eq!(get_extension(&path), Some("mkv".to_string()));
    }

    #[test]
    fn test_get_stem() {
        let path = PathBuf::from("test.mkv");
        assert_eq!(get_stem(&path), Some("test".to_string()));
    }

    #[test]
    fn test_get_filename() {
        let path = PathBuf::from("/dir/test.mkv");
        assert_eq!(get_filename(&path), Some("test.mkv".to_string()));
    }
}
