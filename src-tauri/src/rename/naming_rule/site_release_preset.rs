use super::token::*;

pub fn site_movie_rule() -> NamingRule {
    NamingRule::new("site-movie-rule")
        .add_token(TokenConfig::new(NamingToken::EnglishTitle).with_case(CaseStrategy::PtDotStyle))
        .add_token(TokenConfig::new(NamingToken::Year).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::Resolution).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::Source).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::VideoCodec).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::AudioCodec).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::AudioChannels).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::ReleaseGroup).with_separator(Separator::None).with_prefix("-"))
        .add_token(TokenConfig::new(NamingToken::Ext).with_separator(Separator::None))
}

pub fn site_movie_video_last_rule() -> NamingRule {
    NamingRule::new("site-movie-video-last-rule")
        .add_token(TokenConfig::new(NamingToken::EnglishTitle).with_case(CaseStrategy::PtDotStyle))
        .add_token(TokenConfig::new(NamingToken::Year).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::Resolution).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::Source).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::AudioCodec).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::AudioChannels).with_separator(Separator::Dot))
        .add_token(TokenConfig::new(NamingToken::VideoCodec).with_separator(Separator::Dot).with_empty_policy(EmptyPolicy::NeedsReview))
        .add_token(TokenConfig::new(NamingToken::ReleaseGroup).with_separator(Separator::None).with_prefix("-"))
        .add_token(TokenConfig::new(NamingToken::Ext).with_separator(Separator::None))
}

pub fn original_release_rule() -> NamingRule {
    NamingRule::new("original-release-rule")
        .add_token(TokenConfig::new(NamingToken::OriginalReleaseName))
        .add_token(TokenConfig::new(NamingToken::Ext).with_separator(Separator::None))
}
