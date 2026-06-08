// 敏感字段脱敏模块
// 职责：识别并脱敏敏感字段（API Key、Token、Secret 等）
// 不承担日志写入职责

use once_cell::sync::Lazy;
use regex::Regex;

/// TMDb API Key 模式
static TMDB_KEY_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)(tmdb[_-]?api[_-]?key|tmdbkey)\s*[:=]\s*['"]?([a-zA-Z0-9]{32})['"]?"#)
        .unwrap()
});

/// 通用 API Key 模式
static API_KEY_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)(api[_-]?key)\s*[:=]\s*['"]?([a-zA-Z0-9\-_]{16,})['"]?"#).unwrap()
});

/// Token 模式
static TOKEN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)(token|bearer)\s*[:=]\s*['"]?([a-zA-Z0-9\-_.]{16,})['"]?"#).unwrap()
});

/// Secret 模式
static SECRET_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)(secret|password|passwd|pwd)\s*[:=]\s*['"]?([^\s'"]{8,})['"]?"#).unwrap()
});

/// URL Query 参数中的 key
static URL_KEY_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)([?&])(api_key|apikey|key|token)=([^&\s]+)").unwrap());

/// JSON 字段中的敏感值
static JSON_SENSITIVE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)"(api_key|apikey|token|secret|password|tmdb_api_key)"\s*:\s*"([^"]+)""#)
        .unwrap()
});

/// 脱敏替换标记
const REDACTED: &str = "***REDACTED***";

/// 对文本进行敏感字段脱敏
///
/// 覆盖范围：
/// - TMDb API Key
/// - 通用 API Key
/// - Token / Bearer
/// - Secret / Password
/// - URL Query 中的 key 参数
/// - JSON 字段中的敏感值
pub fn redact_text(input: &str) -> String {
    let mut result = input.to_string();

    // JSON 字段脱敏（优先，因为可能包含其他模式的内容）
    result = JSON_SENSITIVE_RE
        .replace_all(&result, |caps: &regex::Captures| {
            format!(r#""{}": "{}""#, &caps[1], REDACTED)
        })
        .to_string();

    // URL Query 参数脱敏
    result = URL_KEY_RE
        .replace_all(&result, |caps: &regex::Captures| {
            format!("{}{}={}", &caps[1], &caps[2], REDACTED)
        })
        .to_string();

    // TMDb Key 脱敏
    result = TMDB_KEY_RE
        .replace_all(&result, |caps: &regex::Captures| {
            format!("{}: {}", &caps[1], REDACTED)
        })
        .to_string();

    // 通用 API Key 脱敏
    result = API_KEY_RE
        .replace_all(&result, |caps: &regex::Captures| {
            format!("{}: {}", &caps[1], REDACTED)
        })
        .to_string();

    // Token 脱敏
    result = TOKEN_RE
        .replace_all(&result, |caps: &regex::Captures| {
            format!("{}: {}", &caps[1], REDACTED)
        })
        .to_string();

    // Secret / Password 脱敏
    result = SECRET_RE
        .replace_all(&result, |caps: &regex::Captures| {
            format!("{}: {}", &caps[1], REDACTED)
        })
        .to_string();

    result
}

/// 检查文本是否包含敏感信息
pub fn contains_sensitive(text: &str) -> bool {
    TMDB_KEY_RE.is_match(text)
        || API_KEY_RE.is_match(text)
        || TOKEN_RE.is_match(text)
        || SECRET_RE.is_match(text)
        || URL_KEY_RE.is_match(text)
        || JSON_SENSITIVE_RE.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_tmdb_key() {
        let input = "tmdb_api_key: abcdef1234567890abcdef1234567890";
        let result = redact_text(input);
        assert!(result.contains(REDACTED));
        assert!(!result.contains("abcdef1234567890abcdef1234567890"));
    }

    #[test]
    fn test_redact_api_key() {
        let input = "api_key=abcdefghijklmnop";
        let result = redact_text(input);
        assert!(result.contains(REDACTED));
        assert!(!result.contains("abcdefghijklmnop"));
    }

    #[test]
    fn test_redact_token() {
        let input = "token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let result = redact_text(input);
        assert!(result.contains(REDACTED));
        assert!(!result.contains("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"));
    }

    #[test]
    fn test_redact_secret() {
        let input = "secret: my_super_secret_value_123";
        let result = redact_text(input);
        assert!(result.contains(REDACTED));
        assert!(!result.contains("my_super_secret_value_123"));
    }

    #[test]
    fn test_redact_url_query_key() {
        let input = "https://api.example.com/data?api_key=abcdef123456&lang=zh";
        let result = redact_text(input);
        assert!(result.contains("api_key=***REDACTED***"));
        assert!(result.contains("lang=zh")); // 非敏感参数不脱敏
    }

    #[test]
    fn test_redact_json_field() {
        let input = r#"{"tmdb_api_key": "abcdef1234567890", "title": "Test"}"#;
        let result = redact_text(input);
        assert!(result.contains(REDACTED));
        assert!(!result.contains("abcdef1234567890"));
        assert!(result.contains(r#""title": "Test""#)); // 非敏感字段不脱敏
    }

    #[test]
    fn test_normal_text_not_redacted() {
        let input = "This is a normal text about movies and TV shows.";
        let result = redact_text(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_multiple_sensitive_fields() {
        let input = "api_key=abc123def456ghi789 & token: xyz9876543210123456";
        let result = redact_text(input);
        assert!(result.contains(REDACTED));
        assert!(!result.contains("abc123def456ghi789"));
        assert!(!result.contains("xyz9876543210123456"));
    }

    #[test]
    fn test_contains_sensitive_true() {
        assert!(contains_sensitive(
            "tmdb_api_key: abcdef1234567890abcdef1234567890"
        ));
        assert!(contains_sensitive("api_key=abcdefghijklmnop"));
    }

    #[test]
    fn test_contains_sensitive_false() {
        assert!(!contains_sensitive("This is normal text."));
        assert!(!contains_sensitive("Movie title: The Matrix"));
    }
}
