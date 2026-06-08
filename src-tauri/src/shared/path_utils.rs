// 路径工具模块
// 职责：路径清理、验证、转换

use std::path::Path;

/// 清理文件名，移除非法字符
pub fn sanitize_filename(name: &str) -> String {
    // Windows 非法字符: < > : " / \ | ? *
    let illegal_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut sanitized = String::with_capacity(name.len());

    for c in name.chars() {
        if illegal_chars.contains(&c) {
            sanitized.push('_');
        } else {
            sanitized.push(c);
        }
    }

    // 移除首尾空格和点
    sanitized
        .trim_matches(|c: char| c.is_whitespace() || c == '.')
        .to_string()
}

/// 检查路径是否过长 (Windows 260 字符限制)
pub fn is_path_too_long(path: &str) -> bool {
    path.len() > 260
}

/// 检查路径是否包含非法字符
pub fn has_invalid_chars(path: &str) -> bool {
    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
    // 只检查文件名部分，不检查路径分隔符
    if let Some(file_name) = Path::new(path).file_name() {
        let name = file_name.to_string_lossy();
        name.chars().any(|c| invalid_chars.contains(&c))
    } else {
        false
    }
}

/// 规范化路径
pub fn normalize_path(path: &str) -> String {
    // 统一使用反斜杠
    path.replace('/', "\\")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test<>file.txt"), "test__file.txt");
        assert_eq!(sanitize_filename("normal_file.txt"), "normal_file.txt");
        assert_eq!(sanitize_filename(" file.txt "), "file.txt");
    }

    #[test]
    fn test_is_path_too_long() {
        assert!(!is_path_too_long("short/path"));
        assert!(is_path_too_long(&"a".repeat(261)));
    }

    #[test]
    fn test_has_invalid_chars() {
        assert!(has_invalid_chars("test<file>.txt"));
        assert!(!has_invalid_chars("normal_file.txt"));
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("C:/Users/test"), "C:\\Users\\test");
    }
}
