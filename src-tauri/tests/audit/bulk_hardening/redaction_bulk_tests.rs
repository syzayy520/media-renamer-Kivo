// 审计脱敏批量测试
// 职责：测试脱敏功能在大规模数据下的正确性

use app_lib::audit::redaction::redact_text;

#[test]
fn test_redact_api_key_patterns() {
    let texts = vec![
        "api_key: abcdef123456789012345678",
        "Bearer=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
        "tmdb_api_key=sk-1234567890abcdef",
    ];
    for text in texts {
        let redacted = redact_text(text);
        assert!(redacted.contains("***REDACTED***"), "Should redact: {}", text);
    }
}

#[test]
fn test_redact_100_sensitive_entries() {
    for i in 0..100 {
        let text = format!("Processing file {} with api_key: secret_key_value_that_is_long_enough_{}", i, i);
        let redacted = redact_text(&text);
        assert!(redacted.contains("***REDACTED***"));
        assert!(!redacted.contains("secret_key_value_that_is_long_enough"));
    }
}

#[test]
fn test_redact_1000_entries_no_panics() {
    for i in 0..1000 {
        let text = format!("Entry {} token: my_secret_token_value_that_is_long_{}", i, i);
        let redacted = redact_text(&text);
        // Should not panic, and should redact token patterns
        assert!(redacted.contains("***REDACTED***"));
    }
}

#[test]
fn test_non_sensitive_text_unchanged() {
    let texts = vec![
        "Renamed file.mkv to new_name.mkv",
        "Task completed successfully",
        "Scan found 50 video files",
    ];
    for text in texts {
        let redacted = redact_text(text);
        assert_eq!(redacted, text, "Non-sensitive text should be unchanged");
    }
}

#[test]
fn test_mixed_sensitive_and_normal_bulk() {
    for i in 0..500 {
        let normal = format!("Renamed file {}", i);
        let sensitive = format!("api_key: key_value_that_is_long_enough_{}", i);
        
        let redacted_normal = redact_text(&normal);
        let redacted_sensitive = redact_text(&sensitive);
        
        assert_eq!(redacted_normal, normal);
        assert!(redacted_sensitive.contains("***REDACTED***"));
    }
}
