// rename/template/extension_token.rs
// 职责：处理 {ext} / {Ext} 模板令牌，确保扩展名正确渲染
// 规则：
// 1. extension 可能是 "mkv" / ".mkv" / "" / None-like
// 2. 渲染时 {ext} 输出 ".mkv"（含前导点）或空字符串（无扩展名时）
// 3. 双重前导点保护：".mkv" → ".mkv"，不会变成 "..mkv"

/// 规范化扩展名：确保输出格式为 ".ext" 或空字符串
///
/// # 示例
/// - "mkv" → ".mkv"
/// - ".mkv" → ".mkv"
/// - "" → ""
pub fn normalize_extension(raw: &str) -> String {
    let trimmed = raw.trim_start_matches('.');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!(".{}", trimmed)
    }
}

/// 渲染 {ext} 令牌值（小写）
pub fn ext_token_value(raw_extension: &str) -> String {
    normalize_extension(raw_extension)
}

/// 渲染 {Ext} 令牌值（大写）
pub fn ext_token_value_uppercase(raw_extension: &str) -> String {
    normalize_extension(raw_extension).to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_mkv() {
        assert_eq!(normalize_extension("mkv"), ".mkv");
    }

    #[test]
    fn normalize_leading_dot() {
        assert_eq!(normalize_extension(".mkv"), ".mkv");
    }

    #[test]
    fn normalize_empty() {
        assert_eq!(normalize_extension(""), "");
    }

    #[test]
    fn normalize_dot_only() {
        assert_eq!(normalize_extension("."), "");
    }

    #[test]
    fn lowercase_ext() {
        assert_eq!(ext_token_value("mkv"), ".mkv");
        assert_eq!(ext_token_value(""), "");
    }

    #[test]
    fn uppercase_ext() {
        assert_eq!(ext_token_value_uppercase("mkv"), ".MKV");
        assert_eq!(ext_token_value_uppercase(""), "");
    }

    #[test]
    fn ext_with_leading_dot_single_dot_only() {
        // .mkv → .mkv, not ..mkv
        assert_eq!(ext_token_value(".mkv"), ".mkv");
        assert_eq!(ext_token_value(".MKV"), ".MKV");
    }
}
