// redaction 模块 - 密钥脱敏
// 职责：检测敏感字段名、脱敏处理字符串中的敏感数据
// 注意：此模块独立于 audit/redaction.rs，专注于配置域的脱敏需求

use once_cell::sync::Lazy;
use regex::Regex;

/// 敏感字段名列表
static SENSITIVE_FIELDS: &[&str] = &[
    "api_key",
    "apikey",
    "api-key",
    "secret",
    "secret_key",
    "token",
    "access_token",
    "password",
    "passwd",
];

/// 检测字段名是否为敏感字段
pub fn is_sensitive_field(field_name: &str) -> bool {
    let lower = field_name.to_lowercase();
    SENSITIVE_FIELDS.iter().any(|f| lower.contains(f))
}

/// 脱敏处理字符串中的 API Key 模式
/// 匹配形如 api_key=xxx, apiKey=xxx, "api_key": "xxx" 等模式
pub fn redact_sensitive_data(data: &str) -> String {
    static API_KEY_PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r#"(?i)(api[_-]?key|secret|token|password)["']?\s*[=:]\s*["']?([A-Za-z0-9_\-\.]+)"#,
        )
        .unwrap()
    });

    API_KEY_PATTERN
        .replace_all(data, |caps: &regex::Captures| {
            let field = &caps[1];
            format!("{}=<REDACTED>", field)
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sensitive_field_api_key() {
        assert!(is_sensitive_field("api_key"));
        assert!(is_sensitive_field("apiKey"));
        assert!(is_sensitive_field("API-KEY"));
    }

    #[test]
    fn test_is_sensitive_field_token() {
        assert!(is_sensitive_field("access_token"));
        assert!(is_sensitive_field("token"));
    }

    #[test]
    fn test_is_sensitive_field_normal() {
        assert!(!is_sensitive_field("media_type"));
        assert!(!is_sensitive_field("title"));
        assert!(!is_sensitive_field("template"));
    }

    #[test]
    fn test_redact_api_key_in_string() {
        let data = "api_key=sk_1234567890abcdef";
        let result = redact_sensitive_data(data);
        assert!(result.contains("<REDACTED>"));
        assert!(!result.contains("sk_1234567890abcdef"));
    }

    #[test]
    fn test_redact_json_like() {
        let data = r#"{"api_key": "my_secret_key_123"}"#;
        let result = redact_sensitive_data(data);
        assert!(result.contains("<REDACTED>"));
        assert!(!result.contains("my_secret_key_123"));
    }

    #[test]
    fn test_no_redact_normal_data() {
        let data = "title=Movie.Name.2024.mkv";
        let result = redact_sensitive_data(data);
        assert_eq!(result, data);
    }
}
