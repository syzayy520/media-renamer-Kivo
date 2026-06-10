use super::separator_cleanup::*;
use super::template_validation::*;
use super::title_strategy::*;
use super::token::*;

/// Token 值上下文 — 渲染 token 所需的所有数据
#[derive(Debug, Clone, Default)]
pub struct TokenContext {
    pub zh_title: Option<String>,
    pub english_title: Option<String>,
    pub original_title: Option<String>,
    pub original_name_without_ext: Option<String>,
    pub original_release_name: Option<String>,
    pub year: Option<u16>,
    pub release_date: Option<String>,
    pub air_date: Option<String>,
    pub season_year: Option<u16>,
    pub resolution: Option<String>,
    pub source: Option<String>,
    pub edition: Option<String>,
    pub remux: Option<bool>,
    pub video_codec: Option<String>,
    pub video_bit_depth: Option<String>,
    pub hdr_format: Option<String>,
    pub dolby_vision: Option<bool>,
    pub audio_codec: Option<String>,
    pub audio_channels: Option<String>,
    pub audio_language: Option<String>,
    pub release_group: Option<String>,
    pub tmdb_id: Option<u32>,
    pub imdb_id: Option<String>,
    pub tvdb_id: Option<u32>,
    pub show_title: Option<String>,
    pub season: Option<u32>,
    pub episode: Option<Vec<u32>>,
    pub episode_title: Option<String>,
    pub absolute_episode: Option<u32>,
    pub season_title: Option<String>,
    pub ext: Option<String>,
    pub subtitle_language: Option<String>,
    pub file_role: Option<String>,
}

impl TokenContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_zh_title(mut self, t: impl Into<String>) -> Self {
        self.zh_title = Some(t.into());
        self
    }

    pub fn with_english_title(mut self, t: impl Into<String>) -> Self {
        self.english_title = Some(t.into());
        self
    }

    pub fn with_year(mut self, y: u16) -> Self {
        self.year = Some(y);
        self
    }

    pub fn with_ext(mut self, e: impl Into<String>) -> Self {
        self.ext = Some(e.into());
        self
    }

    pub fn with_resolution(mut self, r: impl Into<String>) -> Self {
        self.resolution = Some(r.into());
        self
    }

    pub fn with_pt_info(mut self, name: impl Into<String>) -> Self {
        self.original_release_name = Some(name.into());
        self
    }
}

/// 获取 token 对应的原始值
pub fn token_value(
    token: NamingToken,
    ctx: &TokenContext,
    resolved: Option<&ResolvedTitle>,
) -> Option<String> {
    use NamingToken::*;
    match token {
        ZhTitle => resolved
            .map(|r| r.primary.clone())
            .or_else(|| ctx.zh_title.clone()),
        EnglishTitle => resolved.map(|r| r.fallback.clone()).or_else(|| {
            ctx.english_title
                .clone()
                .or_else(|| ctx.original_title.clone())
        }),
        OriginalTitle => ctx.original_title.clone(),
        OriginalNameWithoutExt => ctx.original_name_without_ext.clone(),
        OriginalReleaseName => ctx.original_release_name.clone(),
        Year => ctx.year.map(|y| y.to_string()),
        ReleaseDate => ctx.release_date.clone(),
        AirDate => ctx.air_date.clone(),
        SeasonYear => ctx.season_year.map(|y| y.to_string()),
        Resolution => ctx.resolution.clone(),
        Source => ctx.source.clone(),
        Edition => ctx.edition.clone(),
        Remux => ctx.remux.map(|r| if r { "Remux" } else { "" }.to_string()),
        VideoCodec => ctx.video_codec.clone(),
        VideoBitDepth => ctx.video_bit_depth.clone(),
        HdrFormat => ctx.hdr_format.clone(),
        DolbyVision => ctx
            .dolby_vision
            .map(|dv| if dv { "DV" } else { "" }.to_string()),
        AudioCodec => ctx.audio_codec.clone(),
        AudioChannels => ctx.audio_channels.clone(),
        AudioLanguage => ctx.audio_language.clone(),
        ReleaseGroup => ctx.release_group.clone(),
        TmdbId => ctx.tmdb_id.map(|id| id.to_string()),
        ImdbId => ctx.imdb_id.clone(),
        TvdbId => ctx.tvdb_id.map(|id| id.to_string()),
        ShowTitle => ctx.show_title.clone(),
        Season => ctx.season.map(|s| format!("S{:02}", s)),
        Episode => ctx.episode.as_ref().map(|eps| {
            if eps.len() == 1 {
                format!("E{:02}", eps[0])
            } else {
                eps.iter()
                    .map(|e| format!("E{:02}", e))
                    .collect::<Vec<_>>()
                    .join("-")
            }
        }),
        EpisodeTitle => ctx.episode_title.clone(),
        AbsoluteEpisode => ctx.absolute_episode.map(|e| format!("{:03}", e)),
        SeasonTitle => ctx.season_title.clone(),
        Ext => {
            let raw = ctx.ext.as_deref().unwrap_or("");
            if raw.is_empty() {
                None
            } else if raw.starts_with('.') {
                Some(raw.to_string())
            } else {
                Some(format!(".{}", raw))
            }
        }
        SubtitleLanguage => ctx.subtitle_language.clone(),
        FileRole => ctx.file_role.clone(),
    }
}

/// 应用大小写策略
pub fn apply_case(value: &str, strategy: CaseStrategy) -> String {
    match strategy {
        CaseStrategy::AsIs => value.to_string(),
        CaseStrategy::LowerCase => value.to_lowercase(),
        CaseStrategy::UpperCase => value.to_uppercase(),
        CaseStrategy::TitleCase => {
            // 按分隔符拆分，每段首字母大写
            let delimiters = [' ', '-', '_', '.'];
            let mut result = String::with_capacity(value.len());
            let mut capitalize_next = true;
            for ch in value.chars() {
                if delimiters.contains(&ch) {
                    result.push(ch);
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(ch.to_uppercase().next().unwrap_or(ch));
                    capitalize_next = false;
                } else {
                    result.push(ch);
                }
            }
            result
        }
        CaseStrategy::PtDotStyle => value.to_lowercase().replace(' ', "."),
    }
}

/// 应用包裹方式
pub fn apply_wrapper(value: &str, wrapper: WrapperStyle) -> String {
    match wrapper {
        WrapperStyle::None => value.to_string(),
        WrapperStyle::Parentheses => format!("({})", value),
        WrapperStyle::Brackets => format!("[{}]", value),
        WrapperStyle::Braces => format!("{{{}}}", value),
    }
}

/// 完整渲染命名规则
pub fn render_naming_rule(
    rule: &NamingRule,
    ctx: &TokenContext,
    strategy: TitleStrategy,
) -> NamingRenderResult {
    let resolved = resolve_title_for_strategy(
        strategy,
        ctx.zh_title.as_deref(),
        ctx.english_title.as_deref(),
        ctx.original_title.as_deref(),
    );

    let mut parts: Vec<String> = Vec::new();
    let mut needs_review = false;

    for (i, config) in rule.tokens.iter().enumerate() {
        if !config.enabled {
            continue;
        }

        let raw_value = token_value(config.token, ctx, Some(&resolved));

        match config.empty_policy {
            EmptyPolicy::Hide => {
                if let Some(ref val) = raw_value {
                    if val.is_empty() {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            EmptyPolicy::Default => {
                // 空值时使用默认占位（暂时跳过）
                if raw_value.is_none() || raw_value.as_deref() == Some("") {
                    continue;
                }
            }
            EmptyPolicy::NeedsReview => {
                if raw_value.is_none() || raw_value.as_deref() == Some("") {
                    needs_review = true;
                    continue;
                }
            }
        }

        let val = raw_value.unwrap_or_default();
        if val.is_empty() {
            continue;
        }

        // 应用大小写
        let cased = apply_case(&val, config.case_strategy);

        // 应用包裹
        let wrapped = apply_wrapper(&cased, config.wrapper);

        // 构建最终 token 文本
        let token_text = if config.prefix.is_empty() && config.suffix.is_empty() {
            wrapped
        } else {
            format!("{}{}{}", config.prefix, wrapped, config.suffix)
        };

        // 添加分隔符（第一个 token 不加）
        if i > 0 && !parts.is_empty() {
            let sep = config.separator.as_str();
            if !sep.is_empty() {
                parts.push(sep.to_string());
            }
        }

        parts.push(token_text);
    }

    let raw_name = parts.join("");
    let cleaned = full_cleanup(&raw_name);
    let validation = validate_name(&cleaned);

    needs_review = needs_review || !validation.is_valid;

    NamingRenderResult {
        name: cleaned,
        needs_review,
        validation,
    }
}

/// 命名渲染结果
#[derive(Debug, Clone)]
pub struct NamingRenderResult {
    pub name: String,
    pub needs_review: bool,
    pub validation: NamingValidation,
}

/// 便捷函数：使用 Clean Library 预设渲染命名
pub fn render_clean_library(ctx: &TokenContext, strategy: TitleStrategy) -> NamingRenderResult {
    render_naming_rule(&super::preset::preset_clean_library(), ctx, strategy)
}

/// 便捷函数：使用 Bilingual 预设渲染命名
pub fn render_bilingual(ctx: &TokenContext) -> NamingRenderResult {
    render_naming_rule(
        &super::preset::preset_bilingual(),
        ctx,
        TitleStrategy::Bilingual,
    )
}

/// 便捷函数：使用 Chinese Prefix PT 预设渲染命名
pub fn render_chinese_prefix_pt(ctx: &TokenContext) -> NamingRenderResult {
    render_naming_rule(
        &super::preset::preset_chinese_prefix_pt(),
        ctx,
        TitleStrategy::ChinesePrefixPt,
    )
}

/// 便捷函数：使用 PT Preserve 预设渲染命名（纯PT保留）
pub fn render_pt_preserve(ctx: &TokenContext) -> NamingRenderResult {
    render_naming_rule(
        &super::preset::preset_pt_preserve(),
        ctx,
        TitleStrategy::ChinesePrefixPt,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_movie_ctx() -> TokenContext {
        TokenContext::new()
            .with_zh_title("第一滴血")
            .with_english_title("First Blood")
            .with_year(1982)
            .with_ext("mkv")
            .with_resolution("1080p")
            .with_pt_info("First.Blood.1982.1080p.BluRay.x264.DTS-Group")
    }

    #[test]
    fn test_clean_library_chinese() {
        let ctx = make_movie_ctx();
        let result = render_clean_library(&ctx, TitleStrategy::ChineseOnly);
        assert_eq!(result.name, "第一滴血 (1982).mkv");
        assert!(!result.needs_review);
    }

    #[test]
    fn test_clean_library_bilingual() {
        let ctx = make_movie_ctx();
        let result = render_clean_library(&ctx, TitleStrategy::Bilingual);
        assert_eq!(result.name, "第一滴血 - First Blood (1982).mkv");
    }

    #[test]
    fn test_clean_library_english() {
        let ctx = make_movie_ctx();
        let result = render_clean_library(&ctx, TitleStrategy::EnglishOnly);
        assert_eq!(result.name, "First Blood (1982).mkv");
    }

    #[test]
    fn test_no_trailing_dot_when_no_ext() {
        let mut ctx = make_movie_ctx();
        ctx.ext = None;
        let result = render_clean_library(&ctx, TitleStrategy::ChineseOnly);
        assert_eq!(result.name, "第一滴血 (1982)");
        assert!(!result.name.ends_with('.'));
    }

    #[test]
    fn test_extension_with_leading_dot_no_double_dot() {
        let mut ctx = make_movie_ctx();
        ctx.ext = Some(".mkv".to_string());
        let result = render_clean_library(&ctx, TitleStrategy::ChineseOnly);
        assert_eq!(result.name, "第一滴血 (1982).mkv");
        assert!(!result.name.contains(".."));
    }

    #[test]
    fn test_pt_preserve_output() {
        let ctx = make_movie_ctx();
        let result = render_chinese_prefix_pt(&ctx);
        assert!(result.name.starts_with("第一滴血"));
        assert!(result.name.contains("First.Blood"));
        assert!(result.name.ends_with(".mkv"));
    }

    #[test]
    fn test_bilingual_preset() {
        let ctx = make_movie_ctx();
        let result = render_bilingual(&ctx);
        assert!(result.name.contains("第一滴血 - First Blood"));
        assert!(result.name.contains("(1982)"));
    }

    #[test]
    fn test_custom_token_order() {
        let ctx = make_movie_ctx();
        let rule = NamingRule::new("Custom")
            .add_token(TokenConfig::new(NamingToken::Resolution).with_separator(Separator::Dot))
            .add_token(TokenConfig::new(NamingToken::ZhTitle).with_separator(Separator::Dot))
            .add_token(TokenConfig::new(NamingToken::Year).with_separator(Separator::Dot))
            .add_token(TokenConfig::new(NamingToken::Ext));

        let result = render_naming_rule(&rule, &ctx, TitleStrategy::ChineseOnly);
        assert!(result.name.starts_with("1080p"));
        assert!(result.name.ends_with(".mkv"));
    }

    #[test]
    fn test_empty_token_hidden_without_dangling() {
        let ctx = make_movie_ctx();
        let rule = NamingRule::new("EmptyTest")
            .add_token(TokenConfig::new(NamingToken::ZhTitle))
            .add_token(
                TokenConfig::new(NamingToken::AudioCodec)
                    .with_separator(Separator::Dot)
                    .with_empty_policy(EmptyPolicy::Hide),
            )
            .add_token(TokenConfig::new(NamingToken::Ext));

        let result = render_naming_rule(&rule, &ctx, TitleStrategy::ChineseOnly);
        // audio_codec is None → hidden, no dangling dot
        assert_eq!(result.name, "第一滴血.mkv");
        assert!(!result.name.contains(".."));
    }

    #[test]
    fn test_wrapper_parentheses() {
        assert_eq!(apply_wrapper("1982", WrapperStyle::Parentheses), "(1982)");
    }

    #[test]
    fn test_wrapper_brackets() {
        assert_eq!(
            apply_wrapper("tmdb-1368", WrapperStyle::Brackets),
            "[tmdb-1368]"
        );
    }

    #[test]
    fn test_case_title_case() {
        assert_eq!(
            apply_case("first blood", CaseStrategy::TitleCase),
            "First Blood"
        );
    }

    #[test]
    fn test_case_pt_dot_style() {
        assert_eq!(
            apply_case("First Blood", CaseStrategy::PtDotStyle),
            "first.blood"
        );
    }
}
