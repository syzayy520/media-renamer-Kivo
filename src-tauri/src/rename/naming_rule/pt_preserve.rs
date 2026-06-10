//! PT/BT 原始发布名中的技术信息提取
//! 用于在中文前缀 + PT 保留策略中保留原始技术信息

/// 从原始文件名提取 PT/BT 技术信息段
/// 例如: "First.Blood.1982.1080p.BluRay.x264.DTS-Group"
/// 返回: "First.Blood.1982.1080p.BluRay.x264.DTS-Group"
pub fn extract_pt_release_name(original_filename_without_ext: &str) -> String {
    original_filename_without_ext.to_string()
}

/// 从原始文件名提取不含标题的技术段
/// 例如: "First.Blood.1982.1080p.BluRay.x264.DTS-Group"
/// 如果提供了已知标题，移除标题部分
pub fn extract_pt_technical_segment(original_name: &str, known_title: Option<&str>) -> String {
    if let Some(title) = known_title {
        // 尝试移除标题前缀（点号分隔）
        let title_dotted = title.replace(' ', ".");
        let lower_original = original_name.to_lowercase();

        // 检查标题是否出现在开头
        if lower_original.starts_with(&title_dotted.to_lowercase()) {
            let remaining = &original_name[title_dotted.len()..];
            // 移除开头的点
            return remaining.trim_start_matches('.').to_string();
        }

        // 检查是否有年份紧随其后
        let title_with_sep = format!("{}.", title_dotted);
        if lower_original.starts_with(&title_with_sep.to_lowercase()) {
            let remaining = &original_name[title_with_sep.len()..];
            return remaining.to_string();
        }
    }

    original_name.to_string()
}

/// 中文前缀 PT 文件名构建
/// zh_title: 中文标题
/// pt_segment: PT 技术段（不含原标题）
/// extension: 扩展名（含前导点）
pub fn build_chinese_prefix_pt_name(zh_title: &str, pt_segment: &str, extension: &str) -> String {
    let base = if pt_segment.is_empty() {
        zh_title.to_string()
    } else {
        format!("{}.{}", zh_title, pt_segment)
    };
    format!("{}{}", base, extension)
}

/// 纯 PT 保留文件名
pub fn build_pt_name(original_name: &str, extension: &str) -> String {
    format!("{}{}", original_name, extension)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pt_release_name_passthrough() {
        let name = "First.Blood.1982.1080p.BluRay.x264.DTS-Group";
        assert_eq!(extract_pt_release_name(name), name);
    }

    #[test]
    fn test_extract_technical_segment_with_known_title() {
        let name = "First.Blood.1982.1080p.BluRay.x264.DTS-Group";
        let result = extract_pt_technical_segment(name, Some("First Blood"));
        assert!(result.contains("1982"));
        assert!(result.contains("1080p"));
        assert!(result.contains("BluRay"));
        assert!(!result.contains("First.Blood"));
    }

    #[test]
    fn test_extract_technical_segment_unknown_title() {
        let name = "First.Blood.1982.1080p.BluRay.x264.DTS-Group";
        let result = extract_pt_technical_segment(name, None);
        assert_eq!(result, name);
    }

    #[test]
    fn test_build_chinese_prefix_pt_name() {
        let result = build_chinese_prefix_pt_name(
            "第一滴血",
            "First.Blood.1982.1080p.BluRay.x264.DTS-Group",
            ".mkv",
        );
        assert!(result.starts_with("第一滴血"));
        assert!(result.ends_with(".mkv"));
        assert!(result.contains("1080p"));
    }

    #[test]
    fn test_build_chinese_prefix_pt_name_no_pt_segment() {
        let result = build_chinese_prefix_pt_name("第一滴血", "", ".mkv");
        assert_eq!(result, "第一滴血.mkv");
    }

    #[test]
    fn test_build_pt_name() {
        let result = build_pt_name("First.Blood.1982.1080p.BluRay", ".mkv");
        assert_eq!(result, "First.Blood.1982.1080p.BluRay.mkv");
    }
}
