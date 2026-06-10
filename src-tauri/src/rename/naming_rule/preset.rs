use super::token::*;

/// 预设 1: Clean Library
/// 第一滴血 (1982).mkv
pub fn preset_clean_library() -> NamingRule {
    NamingRule::new("clean-library")
        .with_description("清爽媒体库命名：中文标题 (年份).扩展名")
        .add_token(TokenConfig::new(NamingToken::ZhTitle))
        .add_token(
            TokenConfig::new(NamingToken::Year)
                .with_separator(Separator::Space)
                .with_wrapper(WrapperStyle::Parentheses),
        )
        .add_token(TokenConfig::new(NamingToken::Ext))
}

/// 预设 2: Bilingual Library
/// 第一滴血 - First Blood (1982).mkv
pub fn preset_bilingual() -> NamingRule {
    NamingRule::new("bilingual-library")
        .with_description("中英双语命名：中文标题 - 英文标题 (年份).扩展名")
        .add_token(TokenConfig::new(NamingToken::ZhTitle))
        .add_token(
            TokenConfig::new(NamingToken::EnglishTitle)
                .with_separator(Separator::Space)
                .with_prefix("- "),
        )
        .add_token(
            TokenConfig::new(NamingToken::Year)
                .with_separator(Separator::Space)
                .with_wrapper(WrapperStyle::Parentheses),
        )
        .add_token(TokenConfig::new(NamingToken::Ext))
}

/// 预设 3: PT Preserve
/// First.Blood.1982.1080p.BluRay.x264.DTS-HD.MA.5.1-Group.mkv
pub fn preset_pt_preserve() -> NamingRule {
    NamingRule::new("pt-preserve")
        .with_description("PT原样保留：原始发布名.扩展名")
        .add_token(
            TokenConfig::new(NamingToken::OriginalReleaseName).with_separator(Separator::Dot),
        )
        .add_token(TokenConfig::new(NamingToken::Year).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::Resolution).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::Source).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::VideoCodec).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::AudioCodec).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::ReleaseGroup).with_separator(Separator::Dash))
        .add_token(TokenConfig::new(NamingToken::Ext))
}

/// 预设 4: Chinese Prefix PT
/// 第一滴血.First.Blood.1982.1080p.BluRay.x264.DTS-Group.mkv
pub fn preset_chinese_prefix_pt() -> NamingRule {
    NamingRule::new("chinese-prefix-pt")
        .with_description("中文名前缀 + PT保留：中文名.技术信息.扩展名")
        .add_token(TokenConfig::new(NamingToken::ZhTitle))
        .add_token(
            TokenConfig::new(NamingToken::OriginalReleaseName).with_separator(Separator::Dot),
        )
        .add_token(TokenConfig::new(NamingToken::Ext))
}

/// 预设 5: BT Friendly
/// 第一滴血.First.Blood.1982.1080p.BluRay.x264.DTS.mkv
pub fn preset_bt_friendly() -> NamingRule {
    NamingRule::new("bt-friendly")
        .with_description("BT友好命名：中文.英文.年份.分辨率.来源.视频编码.音频编码.扩展名")
        .add_token(TokenConfig::new(NamingToken::ZhTitle))
        .add_token(TokenConfig::new(NamingToken::EnglishTitle).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::Year).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::Resolution).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::Source).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::VideoCodec).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::AudioCodec).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::Ext))
}

/// 预设 6: Jellyfin / Emby
/// 第一滴血 (1982).mkv
pub fn preset_jellyfin_emby() -> NamingRule {
    NamingRule::new("jellyfin-emby")
        .with_description("Jellyfin/Emby兼容命名：中文标题 (年份).扩展名，支持文件夹策略")
        .add_token(TokenConfig::new(NamingToken::ZhTitle))
        .add_token(
            TokenConfig::new(NamingToken::Year)
                .with_separator(Separator::Space)
                .with_wrapper(WrapperStyle::Parentheses),
        )
        .add_token(TokenConfig::new(NamingToken::Ext))
}

/// 预设 7: TMDb ID Friendly
/// 第一滴血 (1982) [tmdbid-1368].mkv
pub fn preset_tmdb_id_friendly() -> NamingRule {
    NamingRule::new("tmdb-id-friendly")
        .with_description("TMDb ID友好命名：中文标题 (年份) [tmdbid-xxx].扩展名")
        .add_token(TokenConfig::new(NamingToken::ZhTitle))
        .add_token(
            TokenConfig::new(NamingToken::Year)
                .with_separator(Separator::Space)
                .with_wrapper(WrapperStyle::Parentheses),
        )
        .add_token(
            TokenConfig::new(NamingToken::TmdbId)
                .with_separator(Separator::Space)
                .with_prefix("tmdb-")
                .with_wrapper(WrapperStyle::Brackets)
                .with_empty_policy(EmptyPolicy::Hide),
        )
        .add_token(TokenConfig::new(NamingToken::Ext))
}

/// 返回所有内置预设
pub fn all_presets() -> Vec<(String, String, NamingRule)> {
    vec![
        (
            "clean-library".to_string(),
            "清爽媒体库".to_string(),
            preset_clean_library(),
        ),
        (
            "bilingual-library".to_string(),
            "中英双语".to_string(),
            preset_bilingual(),
        ),
        (
            "pt-preserve".to_string(),
            "PT原样保留".to_string(),
            preset_pt_preserve(),
        ),
        (
            "chinese-prefix-pt".to_string(),
            "中文前缀+PT".to_string(),
            preset_chinese_prefix_pt(),
        ),
        (
            "bt-friendly".to_string(),
            "BT友好".to_string(),
            preset_bt_friendly(),
        ),
        (
            "jellyfin-emby".to_string(),
            "Jellyfin/Emby".to_string(),
            preset_jellyfin_emby(),
        ),
        (
            "tmdb-id-friendly".to_string(),
            "TMDb ID友好".to_string(),
            preset_tmdb_id_friendly(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_presets_have_name_and_description() {
        for (id, label, rule) in all_presets() {
            assert!(!id.is_empty());
            assert!(!label.is_empty());
            assert!(!rule.name.is_empty());
            assert!(!rule.description.is_empty());
            assert!(!rule.tokens.is_empty());
        }
    }

    #[test]
    fn test_clean_library_has_zh_title_and_year_and_ext() {
        let rule = preset_clean_library();
        let tokens: Vec<_> = rule.tokens.iter().map(|t| t.token).collect();
        assert!(tokens.contains(&NamingToken::ZhTitle));
        assert!(tokens.contains(&NamingToken::Year));
        assert!(tokens.contains(&NamingToken::Ext));
    }

    #[test]
    fn test_bilingual_has_zh_and_en() {
        let rule = preset_bilingual();
        let tokens: Vec<_> = rule.tokens.iter().map(|t| t.token).collect();
        assert!(tokens.contains(&NamingToken::ZhTitle));
        assert!(tokens.contains(&NamingToken::EnglishTitle));
    }

    #[test]
    fn test_pt_preserve_has_tech_tokens() {
        let rule = preset_pt_preserve();
        let tokens: Vec<_> = rule.tokens.iter().map(|t| t.token).collect();
        assert!(tokens.contains(&NamingToken::Resolution));
        assert!(tokens.contains(&NamingToken::Source));
        assert!(tokens.contains(&NamingToken::ReleaseGroup));
    }

    #[test]
    fn test_tmdb_id_friendly_has_tmdb_id() {
        let rule = preset_tmdb_id_friendly();
        let tokens: Vec<_> = rule.tokens.iter().map(|t| t.token).collect();
        assert!(tokens.contains(&NamingToken::TmdbId));
    }
}
