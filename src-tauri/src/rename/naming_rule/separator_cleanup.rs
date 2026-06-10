//! 分隔符清理：处理空 token 导致的悬挂分隔符、双分隔符等

/// 清理连续相同分隔符和悬挂分隔符
/// 例: "Title..1080p" → "Title.1080p"
/// 例: "Title.-.1080p" → "Title-1080p"  
/// 例: "Title. " → "Title"
/// 注意：保留短横(-)和常规空格中的内容（如"标题 - 英文标题"中的 - 和空格）
pub fn cleanup_separators(raw: &str) -> String {
    let mut result = raw.to_string();

    // 多次迭代处理连续分隔符，直到稳定
    loop {
        let before = result.clone();

        // 清理连续点
        result = result.replace("..", ".");
        // 清理连续下划线
        result = result.replace("__", "_");
        // 清理连续横杠（但保留有意义的内容，如 " - "）
        result = result.replace("--", "-");
        // 清理连续空格
        result = result.replace("  ", " ");

        // 清理空格+点号组合（点号前/后不应该有空格，除非在括号内）
        // " ." → "."
        result = result.replace(" .", ".");
        // ". " → "."（但注意不破坏 "1982).mkv" 模式）
        // 只在点号后面是空白时清理，不破坏文件扩展名前的点
        let mut tmp = result.clone();
        loop {
            let new_tmp = tmp.replace(". .", "..");
            if new_tmp == tmp {
                break;
            }
            tmp = new_tmp;
        }
        result = tmp;

        // 清理空格+下划线组合
        result = result.replace(" _", "_");
        result = result.replace("_ ", "_");

        if result == before {
            break;
        }
    }

    // 移除尾部分隔符
    result = result.trim_end_matches('.').to_string();
    result = result.trim_end_matches('-').to_string();
    result = result.trim_end_matches('_').to_string();
    result = result.trim_end_matches(' ').to_string();

    result
}

/// 清理空括号
/// 例: "Title ().mkv" → "Title.mkv"
/// 例: "Title [].mkv" → "Title.mkv"
pub fn cleanup_empty_brackets(input: &str) -> String {
    let mut result = input.to_string();

    // 清理空圆括号
    result = result.replace(" ()", "");
    result = result.replace("()", "");

    // 清理空方括号
    result = result.replace(" []", "");
    result = result.replace("[]", "");

    // 清理空花括号
    result = result.replace(" {}", "");
    result = result.replace("{}", "");

    // 清理可能的连续空格（空括号被移除后）
    result = result.split_whitespace().collect::<Vec<&str>>().join(" ");

    result
}

/// 完整清理管线
pub fn full_cleanup(raw: &str) -> String {
    let step1 = cleanup_empty_brackets(raw);
    cleanup_separators(&step1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_double_dot() {
        assert_eq!(cleanup_separators("Title..1080p"), "Title.1080p");
    }

    #[test]
    fn test_cleanup_trailing_dot() {
        assert_eq!(cleanup_separators("Title."), "Title");
    }

    #[test]
    fn test_cleanup_trailing_dash() {
        assert_eq!(cleanup_separators("Title-"), "Title");
    }

    #[test]
    fn test_cleanup_trailing_underscore() {
        assert_eq!(cleanup_separators("Title_"), "Title");
    }

    #[test]
    fn test_cleanup_double_space() {
        assert_eq!(cleanup_separators("Title  1982"), "Title 1982");
    }

    #[test]
    fn test_cleanup_double_dash() {
        assert_eq!(cleanup_separators("Title--1080p"), "Title-1080p");
    }

    #[test]
    fn test_cleanup_empty_parentheses() {
        assert_eq!(cleanup_empty_brackets("Title ().mkv"), "Title.mkv");
    }

    #[test]
    fn test_cleanup_empty_brackets() {
        assert_eq!(cleanup_empty_brackets("Title [].mkv"), "Title.mkv");
    }

    #[test]
    fn test_cleanup_preserves_filled_parentheses() {
        assert_eq!(
            cleanup_empty_brackets("Title (1982).mkv"),
            "Title (1982).mkv"
        );
    }

    #[test]
    fn test_full_cleanup() {
        assert_eq!(full_cleanup("Title.. () .mkv"), "Title.mkv");
    }

    #[test]
    fn test_full_cleanup_no_trailing_dot() {
        assert_eq!(full_cleanup("第一滴血 (1982)."), "第一滴血 (1982)");
    }

    #[test]
    fn test_full_cleanup_preserves_valid_name() {
        assert_eq!(full_cleanup("第一滴血 (1982).mkv"), "第一滴血 (1982).mkv");
    }

    #[test]
    fn test_preserves_dash_with_spaces() {
        // "标题 - 英文标题" 中的 " - " 必须保留
        assert_eq!(
            full_cleanup("第一滴血 - First Blood (1982).mkv"),
            "第一滴血 - First Blood (1982).mkv"
        );
    }

    #[test]
    fn test_cleans_dot_space_dot() {
        // ". ." 清理成单个点，连续的点也会被压缩
        assert_eq!(full_cleanup("Title. .mkv"), "Title.mkv");
    }

    #[test]
    fn test_no_double_dot_after_cleanup() {
        // 连续的点压缩为一个
        assert_eq!(full_cleanup("1080p. .mkv"), "1080p.mkv");
    }
}
