use super::separator_cleanup::*;
use super::template_validation::*;
use super::title_strategy::*;
use super::token::*;

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
    pub fn new() -> Self { Self::default() }
    pub fn with_zh_title(mut self, t: impl Into<String>) -> Self { self.zh_title = Some(t.into()); self }
    pub fn with_english_title(mut self, t: impl Into<String>) -> Self { self.english_title = Some(t.into()); self }
    pub fn with_year(mut self, y: u16) -> Self { self.year = Some(y); self }
    pub fn with_ext(mut self, e: impl Into<String>) -> Self { self.ext = Some(e.into()); self }
    pub fn with_resolution(mut self, r: impl Into<String>) -> Self { self.resolution = Some(r.into()); self }
    pub fn with_pt_info(mut self, name: impl Into<String>) -> Self { self.original_release_name = Some(name.into()); self }
}

pub fn token_value(token: NamingToken, ctx: &TokenContext, resolved: Option<&ResolvedTitle>) -> Option<String> {
    use NamingToken::*;
    match token {
        ZhTitle => resolved.map(|r| r.primary.clone()).or_else(|| ctx.zh_title.clone()),
        EnglishTitle => english_title_value(ctx, resolved),
        OriginalTitle => ctx.original_title.clone(),
        OriginalNameWithoutExt => ctx.original_name_without_ext.as_deref().map(stem_without_ext),
        OriginalReleaseName => ctx.original_release_name.clone().or_else(|| ctx.original_name_without_ext.as_deref().map(stem_without_ext)),
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
        DolbyVision => ctx.dolby_vision.map(|dv| if dv { "DV" } else { "" }.to_string()),
        AudioCodec => ctx.audio_codec.clone(),
        AudioChannels => ctx.audio_channels.clone(),
        AudioLanguage => ctx.audio_language.clone(),
        ReleaseGroup => ctx.release_group.clone(),
        TmdbId => ctx.tmdb_id.map(|id| id.to_string()),
        ImdbId => ctx.imdb_id.clone(),
        TvdbId => ctx.tvdb_id.map(|id| id.to_string()),
        ShowTitle => ctx.show_title.clone(),
        Season => ctx.season.map(|s| format!("S{:02}", s)),
        Episode => ctx.episode.as_ref().map(|eps| render_episode(eps)),
        EpisodeTitle => ctx.episode_title.clone(),
        AbsoluteEpisode => ctx.absolute_episode.map(|e| format!("{:03}", e)),
        SeasonTitle => ctx.season_title.clone(),
        Ext => ext_value(ctx.ext.as_deref().unwrap_or("")),
        SubtitleLanguage => ctx.subtitle_language.clone(),
        FileRole => ctx.file_role.clone(),
    }
}

pub fn apply_case(value: &str, strategy: CaseStrategy) -> String {
    match strategy {
        CaseStrategy::AsIs => value.to_string(),
        CaseStrategy::LowerCase => value.to_lowercase(),
        CaseStrategy::UpperCase => value.to_uppercase(),
        CaseStrategy::TitleCase => title_case(value),
        CaseStrategy::PtDotStyle => value.split_whitespace().collect::<Vec<_>>().join("."),
    }
}

pub fn apply_wrapper(value: &str, wrapper: WrapperStyle) -> String {
    match wrapper {
        WrapperStyle::None => value.to_string(),
        WrapperStyle::Parentheses => format!("({})", value),
        WrapperStyle::Brackets => format!("[{}]", value),
        WrapperStyle::Braces => format!("{{{}}}", value),
    }
}

pub fn render_naming_rule(rule: &NamingRule, ctx: &TokenContext, strategy: TitleStrategy) -> NamingRenderResult {
    if let Some((effective_rule, effective_strategy)) = effective_rule_for_placeholder(rule, strategy) {
        return render_naming_rule(&effective_rule, ctx, effective_strategy);
    }

    let resolved = resolve_title_for_strategy(
        strategy,
        ctx.zh_title.as_deref(),
        ctx.english_title.as_deref(),
        ctx.original_title.as_deref(),
    );

    let mut parts: Vec<String> = Vec::new();
    let mut needs_review = false;

    for (i, config) in rule.tokens.iter().enumerate() {
        if !config.enabled { continue; }

        let raw_value = token_value(config.token, ctx, Some(&resolved));
        match config.empty_policy {
            EmptyPolicy::Hide => {
                if raw_value.as_deref().unwrap_or("").is_empty() { continue; }
            }
            EmptyPolicy::Default => {
                if raw_value.as_deref().unwrap_or("").is_empty() { continue; }
            }
            EmptyPolicy::NeedsReview => {
                if raw_value.as_deref().unwrap_or("").is_empty() {
                    needs_review = true;
                    continue;
                }
            }
        }

        let val = raw_value.unwrap_or_default();
        if val.is_empty() { continue; }

        let cased = apply_case(&val, config.case_strategy);
        let wrapped = apply_wrapper(&cased, config.wrapper);
        let token_text = if config.prefix.is_empty() && config.suffix.is_empty() {
            wrapped
        } else {
            format!("{}{}{}", config.prefix, wrapped, config.suffix)
        };

        if i > 0 && !parts.is_empty() {
            let sep = config.separator.as_str();
            if !sep.is_empty() { parts.push(sep.to_string()); }
        }
        parts.push(token_text);
    }

    let cleaned = full_cleanup(&parts.join(""));
    let validation = validate_name(&cleaned);
    needs_review = needs_review || !validation.is_valid;

    NamingRenderResult { name: cleaned, needs_review, validation }
}

#[derive(Debug, Clone)]
pub struct NamingRenderResult {
    pub name: String,
    pub needs_review: bool,
    pub validation: NamingValidation,
}

pub fn render_clean_library(ctx: &TokenContext, strategy: TitleStrategy) -> NamingRenderResult {
    render_naming_rule(&super::preset::preset_clean_library(), ctx, strategy)
}

pub fn render_bilingual(ctx: &TokenContext) -> NamingRenderResult {
    render_naming_rule(&super::preset::preset_bilingual(), ctx, TitleStrategy::Bilingual)
}

pub fn render_chinese_prefix_pt(ctx: &TokenContext) -> NamingRenderResult {
    render_naming_rule(&super::preset::preset_chinese_prefix_pt(), ctx, TitleStrategy::ChinesePrefixPt)
}

pub fn render_pt_preserve(ctx: &TokenContext) -> NamingRenderResult {
    render_naming_rule(&super::preset::preset_pt_preserve(), ctx, TitleStrategy::ChinesePrefixPt)
}

fn effective_rule_for_placeholder(rule: &NamingRule, strategy: TitleStrategy) -> Option<(NamingRule, TitleStrategy)> {
    if !is_preview_placeholder_rule(rule) { return None; }

    match rule.name.as_str() {
        "pt-0day-movie" => Some((super::site_release_preset::site_movie_rule(), TitleStrategy::EnglishOnly)),
        "pt-0day-video-postfix" => Some((super::site_release_preset::site_movie_video_last_rule(), TitleStrategy::EnglishOnly)),
        "pt-original-release" | "pt-preserve" => Some((super::site_release_preset::original_release_rule(), strategy)),
        "chinese-prefix-pt" => Some((super::preset::preset_chinese_prefix_pt(), TitleStrategy::ChinesePrefixPt)),
        _ => None,
    }
}

fn is_preview_placeholder_rule(rule: &NamingRule) -> bool {
    rule.tokens.len() == 3
        && rule.tokens[0].token == NamingToken::ZhTitle
        && rule.tokens[1].token == NamingToken::Year
        && rule.tokens[2].token == NamingToken::Ext
}

fn english_title_value(ctx: &TokenContext, resolved: Option<&ResolvedTitle>) -> Option<String> {
    resolved
        .and_then(|r| non_empty(&r.fallback).or_else(|| ascii_title(&r.primary)))
        .or_else(|| ctx.english_title.as_deref().and_then(non_empty))
        .or_else(|| ctx.original_title.as_deref().and_then(non_empty))
        .or_else(|| ctx.original_release_name.as_deref().and_then(title_from_release_name))
        .or_else(|| ctx.original_name_without_ext.as_deref().and_then(title_from_release_name))
}

fn title_from_release_name(value: &str) -> Option<String> {
    let stem = stem_without_ext(value);
    let mut parts = Vec::new();
    for part in stem.split(|ch| ch == '.' || ch == '_' || ch == ' ') {
        if part.is_empty() { continue; }
        if is_year(part) || is_technical_token(part) { break; }
        parts.push(part);
    }

    if parts.is_empty() { None } else { Some(parts.join(" ")) }
}

fn ascii_title(value: &str) -> Option<String> {
    if value.chars().any(|ch| ch.is_ascii_alphabetic()) && !value.chars().any(|ch| ('\u{4e00}'..='\u{9fff}').contains(&ch)) {
        non_empty(value)
    } else {
        None
    }
}

fn non_empty(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
}

fn stem_without_ext(value: &str) -> String {
    let name = value.rsplit(|ch| ch == '\\' || ch == '/').next().unwrap_or(value);
    match name.rfind('.') {
        Some(index) => name[..index].to_string(),
        None => name.to_string(),
    }
}

fn is_year(value: &str) -> bool {
    value.len() == 4 && value.chars().all(|ch| ch.is_ascii_digit())
}

fn is_technical_token(value: &str) -> bool {
    matches!(
        value.to_lowercase().as_str(),
        "720p" | "1080p" | "2160p" | "4320p" | "bluray" | "blu-ray" | "web-dl" | "webrip" | "hdtv" | "remux" | "x264" | "x265" | "h264" | "h265" | "hevc" | "avc" | "dts" | "aac" | "ddp" | "truehd" | "flac"
    )
}

fn render_episode(eps: &[u32]) -> String {
    if eps.len() == 1 {
        format!("E{:02}", eps[0])
    } else {
        eps.iter().map(|e| format!("E{:02}", e)).collect::<Vec<_>>().join("-")
    }
}

fn ext_value(raw: &str) -> Option<String> {
    if raw.is_empty() { None } else if raw.starts_with('.') { Some(raw.to_string()) } else { Some(format!(".{}", raw)) }
}

fn title_case(value: &str) -> String {
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
