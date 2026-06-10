/// 标题策略 — 决定使用哪种语言标题
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TitleStrategy {
    /// 仅中文
    ChineseOnly,
    /// 仅英文/原文
    EnglishOnly,
    /// 双语: 中文 - 英文
    Bilingual,
    /// 中文前缀 + PT 原样保留
    ChinesePrefixPt,
    /// 中文文件夹 + PT 文件名 （此策略影响 folder-policy）
    ChineseFolderPtFile,
    /// 中文文件夹 + 中文前缀 PT 文件名
    ChineseFolderChinesePrefixPtFile,
}

impl TitleStrategy {
    pub fn display_name(&self) -> &'static str {
        match self {
            TitleStrategy::ChineseOnly => "仅中文",
            TitleStrategy::EnglishOnly => "仅英文",
            TitleStrategy::Bilingual => "中英双语",
            TitleStrategy::ChinesePrefixPt => "中文前缀 + PT保留",
            TitleStrategy::ChineseFolderPtFile => "中文文件夹 + PT文件名",
            TitleStrategy::ChineseFolderChinesePrefixPtFile => "中文文件夹 + 中文前缀PT文件名",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TitleStrategy::ChineseOnly => "使用中文标题命名，如：第一滴血 (1982).mkv",
            TitleStrategy::EnglishOnly => "使用英文原标题命名，如：First Blood (1982).mkv",
            TitleStrategy::Bilingual => "中英双语命名，如：第一滴血 - First Blood (1982).mkv",
            TitleStrategy::ChinesePrefixPt => "中文名前缀 + PT原始发布名保留，如：第一滴血.First.Blood.1982.1080p.BluRay.mkv",
            TitleStrategy::ChineseFolderPtFile => "中文文件夹名 + PT原始文件名，文件夹：第一滴血 (1982)/，文件保留PT原名",
            TitleStrategy::ChineseFolderChinesePrefixPtFile => "中文文件夹 + 中文前缀 + PT保留，文件夹：第一滴血 (1982)/，文件：第一滴血.First.Blood.1982.1080p.BluRay.mkv",
        }
    }

    /// 所有可用的标题策略
    pub fn all() -> Vec<TitleStrategy> {
        vec![
            TitleStrategy::ChineseOnly,
            TitleStrategy::EnglishOnly,
            TitleStrategy::Bilingual,
            TitleStrategy::ChinesePrefixPt,
            TitleStrategy::ChineseFolderPtFile,
            TitleStrategy::ChineseFolderChinesePrefixPtFile,
        ]
    }
}

/// 根据标题策略决定如何渲染 ZhTitle 和 EnglishTitle token
pub fn resolve_title_for_strategy(
    strategy: TitleStrategy,
    zh_title: Option<&str>,
    english_title: Option<&str>,
    original_title: Option<&str>,
) -> ResolvedTitle {
    let zh = zh_title.unwrap_or("");
    let en = english_title.or(original_title).unwrap_or("");

    match strategy {
        TitleStrategy::ChineseOnly => ResolvedTitle {
            primary: zh.to_string(),
            fallback: if zh.is_empty() {
                en.to_string()
            } else {
                String::new()
            },
            file_level_title: zh.to_string(),
            folder_level_title: zh.to_string(),
        },
        TitleStrategy::EnglishOnly => ResolvedTitle {
            primary: en.to_string(),
            fallback: if en.is_empty() {
                zh.to_string()
            } else {
                String::new()
            },
            file_level_title: en.to_string(),
            folder_level_title: en.to_string(),
        },
        TitleStrategy::Bilingual => {
            let combined = if !zh.is_empty() && !en.is_empty() {
                format!("{} - {}", zh, en)
            } else if !zh.is_empty() {
                zh.to_string()
            } else {
                en.to_string()
            };
            ResolvedTitle {
                primary: combined.clone(),
                fallback: String::new(),
                file_level_title: combined.clone(),
                folder_level_title: combined,
            }
        }
        TitleStrategy::ChinesePrefixPt => ResolvedTitle {
            primary: zh.to_string(),
            fallback: en.to_string(),
            file_level_title: zh.to_string(),
            folder_level_title: zh.to_string(),
        },
        TitleStrategy::ChineseFolderPtFile => ResolvedTitle {
            primary: zh.to_string(),
            fallback: en.to_string(),
            file_level_title: en.to_string(),
            folder_level_title: zh.to_string(),
        },
        TitleStrategy::ChineseFolderChinesePrefixPtFile => ResolvedTitle {
            primary: zh.to_string(),
            fallback: en.to_string(),
            file_level_title: zh.to_string(),
            folder_level_title: zh.to_string(),
        },
    }
}

/// 解析后的标题
#[derive(Debug, Clone)]
pub struct ResolvedTitle {
    /// 主要标题（用于模板中 ZhTitle / EnglishTitle token 渲染）
    pub primary: String,
    /// 回退标题（用于 PT 前缀策略的英文部分）
    pub fallback: String,
    /// 文件级标题（用于目标文件名）
    pub file_level_title: String,
    /// 文件夹级标题（用于目标文件夹名）
    pub folder_level_title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_only() {
        let r = resolve_title_for_strategy(
            TitleStrategy::ChineseOnly,
            Some("第一滴血"),
            Some("First Blood"),
            None,
        );
        assert_eq!(r.primary, "第一滴血");
        assert_eq!(r.file_level_title, "第一滴血");
    }

    #[test]
    fn test_english_only() {
        let r = resolve_title_for_strategy(
            TitleStrategy::EnglishOnly,
            Some("第一滴血"),
            Some("First Blood"),
            None,
        );
        assert_eq!(r.primary, "First Blood");
    }

    #[test]
    fn test_bilingual() {
        let r = resolve_title_for_strategy(
            TitleStrategy::Bilingual,
            Some("第一滴血"),
            Some("First Blood"),
            None,
        );
        assert_eq!(r.primary, "第一滴血 - First Blood");
    }

    #[test]
    fn test_chinese_prefix_pt() {
        let r = resolve_title_for_strategy(
            TitleStrategy::ChinesePrefixPt,
            Some("第一滴血"),
            Some("First Blood"),
            None,
        );
        assert_eq!(r.file_level_title, "第一滴血");
        assert_eq!(r.fallback, "First Blood");
    }

    #[test]
    fn test_chinese_folder_pt_file() {
        let r = resolve_title_for_strategy(
            TitleStrategy::ChineseFolderPtFile,
            Some("第一滴血"),
            Some("First Blood"),
            None,
        );
        assert_eq!(r.folder_level_title, "第一滴血");
        assert_eq!(r.file_level_title, "First Blood");
    }

    #[test]
    fn test_all_strategies_have_display_name() {
        for s in TitleStrategy::all() {
            assert!(!s.display_name().is_empty());
            assert!(!s.description().is_empty());
        }
    }

    #[test]
    fn test_empty_zh_falls_back_to_en() {
        let r =
            resolve_title_for_strategy(TitleStrategy::ChineseOnly, None, Some("First Blood"), None);
        assert_eq!(r.primary, "");
        assert_eq!(r.fallback, "First Blood");
    }

    #[test]
    fn test_empty_en_falls_back_to_zh() {
        let r =
            resolve_title_for_strategy(TitleStrategy::EnglishOnly, Some("第一滴血"), None, None);
        assert_eq!(r.primary, "");
        assert_eq!(r.fallback, "第一滴血");
    }
}
