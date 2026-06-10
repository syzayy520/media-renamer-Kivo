//! 命名结果验证：检查生成的命名是否符合规范
//! 检测并标记以下问题:
//! - 尾随 . - _
//! - 双点 ..
//! - 空括号 () [] {}
//! - 仅扩展名
//! - 文件名为空

/// 命名验证结果
#[derive(Debug, Clone)]
pub struct NamingValidation {
    pub is_valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl NamingValidation {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, msg: impl Into<String>) {
        self.is_valid = false;
        self.errors.push(msg.into());
    }

    pub fn add_warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
}

/// 验证生成的命名
pub fn validate_name(name: &str) -> NamingValidation {
    let mut v = NamingValidation::valid();

    // 检查空名
    if name.is_empty() {
        v.add_warning("文件名为空");
        return v;
    }

    // 检查仅扩展名
    if name.starts_with('.') && name[1..].find('.').is_none() {
        v.add_warning("文件名仅为扩展名");
        return v;
    }

    // 检查尾随点/横杠/下划线
    if name.ends_with('.') {
        v.add_error("文件名末尾有点(.)");
    }
    if name.ends_with('-') {
        v.add_error("文件名末尾有横杠(-)");
    }
    if name.ends_with('_') {
        v.add_error("文件名末尾有下划线(_)");
    }

    // 检查双点
    if name.contains("..") {
        v.add_error("文件名包含连续点(..)");
    }

    // 检查空括号
    if name.contains("()") || name.contains("( )") {
        v.add_error("文件名包含空圆括号");
    }
    if name.contains("[]") || name.contains("[ ]") {
        v.add_error("文件名包含空方括号");
    }
    if name.contains("{}") || name.contains("{ }") {
        v.add_error("文件名包含空花括号");
    }

    // 检查多个连续空格
    if name.contains("  ") || name.contains("\t") {
        v.add_warning("文件名包含多余空格或制表符");
    }

    // 检查非法文件名字符 (Windows)
    for ch in name.chars() {
        match ch {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' => {
                v.add_error(format!("文件名包含非法字符: '{}'", ch));
            }
            '\\' | '/' => {
                v.add_error(format!("文件名包含路径分隔符: '{}'", ch));
            }
            _ => {}
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let v = validate_name("第一滴血 (1982).mkv");
        assert!(v.is_valid);
    }

    #[test]
    fn test_trailing_dot_error() {
        let v = validate_name("第一滴血 (1982).");
        assert!(!v.is_valid);
        assert!(v.errors.iter().any(|e| e.contains("末尾有点")));
    }

    #[test]
    fn test_trailing_dash_error() {
        let v = validate_name("第一滴血 (1982)-");
        assert!(!v.is_valid);
    }

    #[test]
    fn test_double_dot_error() {
        let v = validate_name("第一滴血..mkv");
        assert!(!v.is_valid);
    }

    #[test]
    fn test_empty_parentheses_error() {
        let v = validate_name("Title ().mkv");
        assert!(!v.is_valid);
    }

    #[test]
    fn test_empty_brackets_error() {
        let v = validate_name("Title [].mkv");
        assert!(!v.is_valid);
    }

    #[test]
    fn test_empty_name_warning() {
        let v = validate_name("");
        assert!(v.warnings.iter().any(|w| w.contains("空")));
    }

    #[test]
    fn test_extension_only_warning() {
        let v = validate_name(".mkv");
        assert!(v.warnings.iter().any(|w| w.contains("扩展名")));
    }

    #[test]
    fn test_illegal_char_error() {
        let v = validate_name("Title<bad>.mkv");
        assert!(!v.is_valid);
        assert!(v.errors.iter().any(|e| e.contains("非法字符")));
    }
}
