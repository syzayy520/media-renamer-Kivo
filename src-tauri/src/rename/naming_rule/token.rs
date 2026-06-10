/// 命名 Token 类型 — 所有可用于命名规则的字段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NamingToken {
    // 基础标题
    /// 中文标题 (来自 TMDb 候选或被解析的标题)
    ZhTitle,
    /// 英文标题
    EnglishTitle,
    /// 原标题 (original_title from TMDb)
    OriginalTitle,
    /// 原始文件名（去扩展名）
    OriginalNameWithoutExt,
    /// 原始 PT/BT 发布名主体
    OriginalReleaseName,

    // 日期/年份
    /// 年份 (e.g. 1982)
    Year,
    /// 完整发布日期
    ReleaseDate,
    /// 首播日期
    AirDate,
    /// 季年份
    SeasonYear,

    // 媒体信息
    /// 分辨率 (e.g. 1080p, 2160p)
    Resolution,
    /// 来源 (e.g. BluRay, WEB-DL, HDTV)
    Source,
    /// 版本 (e.g. Director's Cut, Extended)
    Edition,
    /// 是否 Remux
    Remux,
    /// 视频编码 (e.g. x265, x264, HEVC)
    VideoCodec,
    /// 视频位深 (e.g. 10bit, 8bit)
    VideoBitDepth,
    /// HDR 格式 (e.g. HDR10, HDR10+, Dolby Vision)
    HdrFormat,
    /// Dolby Vision
    DolbyVision,
    /// 音频编码 (e.g. DTS-HD.MA, TrueHD, AAC)
    AudioCodec,
    /// 音频声道 (e.g. 5.1, 7.1, 2.0)
    AudioChannels,
    /// 音频语言
    AudioLanguage,
    /// 发布组
    ReleaseGroup,

    // 影视库 ID
    /// TMDb ID
    TmdbId,
    /// IMDb ID
    ImdbId,
    /// TVDB ID (后续扩展)
    TvdbId,

    // 剧集信息
    /// 剧名
    ShowTitle,
    /// 季号
    Season,
    /// 集号
    Episode,
    /// 集标题
    EpisodeTitle,
    /// 绝对集号
    AbsoluteEpisode,
    /// 季标题
    SeasonTitle,

    // 文件信息
    /// 扩展名
    Ext,
    /// 字幕语言
    SubtitleLanguage,
    /// 文件角色 (主视频/字幕/图片/NFO等)
    FileRole,
}

impl NamingToken {
    /// 所有可用 token 列表
    pub fn all() -> Vec<NamingToken> {
        use NamingToken::*;
        vec![
            ZhTitle,
            EnglishTitle,
            OriginalTitle,
            OriginalNameWithoutExt,
            OriginalReleaseName,
            Year,
            ReleaseDate,
            AirDate,
            SeasonYear,
            Resolution,
            Source,
            Edition,
            Remux,
            VideoCodec,
            VideoBitDepth,
            HdrFormat,
            DolbyVision,
            AudioCodec,
            AudioChannels,
            AudioLanguage,
            ReleaseGroup,
            TmdbId,
            ImdbId,
            TvdbId,
            ShowTitle,
            Season,
            Episode,
            EpisodeTitle,
            AbsoluteEpisode,
            SeasonTitle,
            Ext,
            SubtitleLanguage,
            FileRole,
        ]
    }

    /// 分类显示标签
    pub fn category(&self) -> &'static str {
        use NamingToken::*;
        match self {
            ZhTitle
            | EnglishTitle
            | OriginalTitle
            | OriginalNameWithoutExt
            | OriginalReleaseName => "标题",
            Year | ReleaseDate | AirDate | SeasonYear => "日期",
            Resolution | Source | Edition | Remux | VideoCodec | VideoBitDepth | HdrFormat
            | DolbyVision | AudioCodec | AudioChannels | AudioLanguage | ReleaseGroup => "媒体信息",
            TmdbId | ImdbId | TvdbId => "ID",
            ShowTitle | Season | Episode | EpisodeTitle | AbsoluteEpisode | SeasonTitle => "剧集",
            Ext | SubtitleLanguage | FileRole => "文件",
        }
    }

    pub fn display_name(&self) -> &'static str {
        use NamingToken::*;
        match self {
            ZhTitle => "中文标题",
            EnglishTitle => "英文标题",
            OriginalTitle => "原标题",
            OriginalNameWithoutExt => "原始文件名",
            OriginalReleaseName => "PT发布名",
            Year => "年份",
            ReleaseDate => "发布日期",
            AirDate => "首播日期",
            SeasonYear => "季年份",
            Resolution => "分辨率",
            Source => "来源",
            Edition => "版本",
            Remux => "Remux",
            VideoCodec => "视频编码",
            VideoBitDepth => "位深",
            HdrFormat => "HDR",
            DolbyVision => "Dolby Vision",
            AudioCodec => "音频编码",
            AudioChannels => "音频声道",
            AudioLanguage => "音频语言",
            ReleaseGroup => "发布组",
            TmdbId => "TMDb ID",
            ImdbId => "IMDb ID",
            TvdbId => "TVDB ID",
            ShowTitle => "剧名",
            Season => "季号",
            Episode => "集号",
            EpisodeTitle => "集标题",
            AbsoluteEpisode => "绝对集号",
            SeasonTitle => "季标题",
            Ext => "扩展名",
            SubtitleLanguage => "字幕语言",
            FileRole => "文件角色",
        }
    }
}

/// 大小写策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CaseStrategy {
    /// 原样
    AsIs,
    /// 首字母大写 (Title Case)
    TitleCase,
    /// 全小写
    LowerCase,
    /// 全大写
    UpperCase,
    /// PT 点号风格 (全小写，点号分隔)
    PtDotStyle,
}

/// 空值策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EmptyPolicy {
    /// 空值时隐藏当前 token（含分隔符）
    Hide,
    /// 空值时显示默认占位值
    Default,
    /// 空值时标记整条规则需审核
    NeedsReview,
}

/// 包裹方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WrapperStyle {
    /// 无包裹
    None,
    /// 圆括号 (...)
    Parentheses,
    /// 方括号 [...]
    Brackets,
    /// 花括号 {...}
    Braces,
}

/// 分隔符类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Separator {
    /// 点号 .
    Dot,
    /// 空格
    Space,
    /// 横杠 -
    Dash,
    /// 下划线 _
    Underscore,
    /// 无分隔符
    None,
}

impl Separator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Separator::Dot => ".",
            Separator::Space => " ",
            Separator::Dash => "-",
            Separator::Underscore => "_",
            Separator::None => "",
        }
    }
}

/// 单个 Token 在命名规则中的配置
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TokenConfig {
    pub token: NamingToken,
    /// 前缀
    pub prefix: String,
    /// 后缀
    pub suffix: String,
    /// 分隔符（与前一token之间）
    pub separator: Separator,
    /// 空值策略
    pub empty_policy: EmptyPolicy,
    /// 大小写策略
    pub case_strategy: CaseStrategy,
    /// 包裹方式
    pub wrapper: WrapperStyle,
    /// 是否启用此 token
    pub enabled: bool,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            token: NamingToken::ZhTitle,
            prefix: String::new(),
            suffix: String::new(),
            separator: Separator::Space,
            empty_policy: EmptyPolicy::Hide,
            case_strategy: CaseStrategy::AsIs,
            wrapper: WrapperStyle::None,
            enabled: true,
        }
    }
}

impl TokenConfig {
    pub fn new(token: NamingToken) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }

    pub fn with_separator(mut self, sep: Separator) -> Self {
        self.separator = sep;
        self
    }

    pub fn with_wrapper(mut self, w: WrapperStyle) -> Self {
        self.wrapper = w;
        self
    }

    pub fn with_case(mut self, c: CaseStrategy) -> Self {
        self.case_strategy = c;
        self
    }

    pub fn with_empty_policy(mut self, p: EmptyPolicy) -> Self {
        self.empty_policy = p;
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }
}

/// 完整命名规则 — 一个有序 token 列表
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NamingRule {
    /// 规则名称
    pub name: String,
    /// 规则描述
    pub description: String,
    /// 有序 token 列表
    pub tokens: Vec<TokenConfig>,
}

impl NamingRule {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn add_token(mut self, config: TokenConfig) -> Self {
        self.tokens.push(config);
        self
    }

    /// 是否包含扩展名token
    pub fn has_ext_token(&self) -> bool {
        self.tokens
            .iter()
            .any(|t| t.enabled && t.token == NamingToken::Ext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tokens_have_display_name() {
        for token in NamingToken::all() {
            let name = token.display_name();
            assert!(!name.is_empty(), "{:?} has no display name", token);
        }
    }

    #[test]
    fn test_all_tokens_have_category() {
        for token in NamingToken::all() {
            let cat = token.category();
            assert!(!cat.is_empty(), "{:?} has no category", token);
        }
    }

    #[test]
    fn test_naming_rule_builder() {
        let rule = NamingRule::new("Test")
            .with_description("A test rule")
            .add_token(
                TokenConfig::new(NamingToken::ZhTitle).with_wrapper(WrapperStyle::Parentheses),
            )
            .add_token(
                TokenConfig::new(NamingToken::Year)
                    .with_wrapper(WrapperStyle::Parentheses)
                    .with_separator(Separator::Space),
            )
            .add_token(TokenConfig::new(NamingToken::Ext));

        assert_eq!(rule.tokens.len(), 3);
        assert!(rule.has_ext_token());
    }

    #[test]
    fn test_separator_as_str() {
        assert_eq!(Separator::Dot.as_str(), ".");
        assert_eq!(Separator::Space.as_str(), " ");
        assert_eq!(Separator::Dash.as_str(), "-");
        assert_eq!(Separator::Underscore.as_str(), "_");
        assert_eq!(Separator::None.as_str(), "");
    }
}
