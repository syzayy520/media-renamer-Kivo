// 文件检测器模块
// 职责：检测视频文件类型和伴随文件类型

/// 视频文件扩展名白名单
const VIDEO_EXTENSIONS: &[&str] = &[
    "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm", "ts", "m2ts", "mts", "mpg", "mpeg", "m4v",
];

/// 伴随文件扩展名白名单
const COMPANION_EXTENSIONS: &[&str] = &[
    "srt", "ass", "ssa", "sub", "idx", "nfo", "jpg", "jpeg", "png", "webp", "txt",
];

/// 检查是否为视频文件
pub fn is_video(extension: &str) -> bool {
    let ext = extension.to_lowercase();
    VIDEO_EXTENSIONS.contains(&ext.as_str())
}

/// 检查是否为伴随文件
pub fn is_companion(extension: &str) -> bool {
    let ext = extension.to_lowercase();
    COMPANION_EXTENSIONS.contains(&ext.as_str())
}

/// 获取视频文件扩展名列表
pub fn get_video_extensions() -> &'static [&'static str] {
    VIDEO_EXTENSIONS
}

/// 获取伴随文件扩展名列表
pub fn get_companion_extensions() -> &'static [&'static str] {
    COMPANION_EXTENSIONS
}

/// 检查是否为媒体相关文件（视频或伴随）
pub fn is_media_file(extension: &str) -> bool {
    is_video(extension) || is_companion(extension)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_video() {
        assert!(is_video("mkv"));
        assert!(is_video("MP4"));
        assert!(is_video("avi"));
        assert!(!is_video("txt"));
        assert!(!is_video("srt"));
    }

    #[test]
    fn test_is_companion() {
        assert!(is_companion("srt"));
        assert!(is_companion("ASS"));
        assert!(is_companion("nfo"));
        assert!(!is_companion("mkv"));
    }

    #[test]
    fn test_is_media_file() {
        assert!(is_media_file("mkv"));
        assert!(is_media_file("srt"));
        assert!(!is_media_file("exe"));
    }

    #[test]
    fn test_video_extensions_count() {
        assert_eq!(VIDEO_EXTENSIONS.len(), 13);
    }

    #[test]
    fn test_companion_extensions_count() {
        assert_eq!(COMPANION_EXTENSIONS.len(), 11);
    }
}
