use crate::media_info::contracts::{NormalizedAudioInfo, NormalizedMediaInfo, NormalizedVideoInfo};

pub fn parse_filename_media_info(file_path: &str) -> NormalizedMediaInfo {
    let name = file_stem(file_path);
    let tokens = split_release_tokens(&name);

    NormalizedMediaInfo {
        resolution: find_first(&tokens, is_resolution),
        source: find_first(&tokens, is_source),
        release_group: release_group(&name),
        video: NormalizedVideoInfo {
            codec: find_first(&tokens, is_video_codec),
            bit_depth: find_first(&tokens, is_bit_depth),
            hdr_format: find_first(&tokens, is_hdr),
            dolby_vision: Some(tokens.iter().any(|token| matches_normalized(token, &["dv", "dovi", "dolbyvision"]))),
        },
        audio: NormalizedAudioInfo {
            codec: find_first(&tokens, is_audio_codec),
            channels: find_first(&tokens, is_audio_channels),
            language: find_first(&tokens, is_language),
        },
    }
}

fn file_stem(file_path: &str) -> String {
    let name = file_path.rsplit(|ch| ch == '\\' || ch == '/').next().unwrap_or(file_path);
    match name.rfind('.') {
        Some(index) => name[..index].to_string(),
        None => name.to_string(),
    }
}

fn split_release_tokens(value: &str) -> Vec<String> {
    value
        .split(|ch| ch == '.' || ch == '_' || ch == ' ' || ch == '[' || ch == ']' || ch == '(' || ch == ')')
        .filter(|item| !item.trim().is_empty())
        .map(|item| item.trim().to_string())
        .collect()
}

fn find_first(tokens: &[String], matcher: fn(&str) -> bool) -> Option<String> {
    tokens.iter().find(|token| matcher(token)).cloned()
}

fn release_group(value: &str) -> Option<String> {
    value.rsplit_once('-')
        .map(|(_, group)| group.trim())
        .filter(|group| !group.is_empty())
        .map(str::to_string)
}

fn is_resolution(value: &str) -> bool {
    matches_normalized(value, &["720p", "1080p", "2160p", "4320p", "4k"])
}

fn is_source(value: &str) -> bool {
    matches_normalized(value, &["bluray", "blu-ray", "webdl", "web-dl", "webrip", "hdtv", "remux", "uhd", "dvd"])
}

fn is_video_codec(value: &str) -> bool {
    matches_normalized(value, &["x264", "x265", "h264", "h265", "hevc", "avc", "av1", "vp9"])
}

fn is_bit_depth(value: &str) -> bool {
    matches_normalized(value, &["8bit", "10bit", "12bit"])
}

fn is_hdr(value: &str) -> bool {
    matches_normalized(value, &["hdr", "hdr10", "hdr10plus", "hlg", "dv", "dovi"])
}

fn is_audio_codec(value: &str) -> bool {
    matches_normalized(value, &["aac", "ac3", "eac3", "ddp", "dts", "truehd", "atmos", "flac", "opus", "mp3"])
}

fn is_audio_channels(value: &str) -> bool {
    let normalized = normalize_token(value);
    matches!(normalized.as_str(), "10" | "20" | "51" | "61" | "71")
}

fn is_language(value: &str) -> bool {
    matches_normalized(value, &["chinese", "mandarin", "cantonese", "english", "japanese", "korean", "dual", "multi"])
}

fn matches_normalized(value: &str, options: &[&str]) -> bool {
    let normalized = normalize_token(value);
    options.iter().any(|option| normalized == normalize_token(option))
}

fn normalize_token(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pt_release_fields() {
        let info = parse_filename_media_info("First.Blood.1982.1080p.BluRay.x265.DTS.5.1-PTer.mkv");

        assert_eq!(info.resolution.as_deref(), Some("1080p"));
        assert_eq!(info.source.as_deref(), Some("BluRay"));
        assert_eq!(info.video.codec.as_deref(), Some("x265"));
        assert_eq!(info.audio.codec.as_deref(), Some("DTS"));
        assert_eq!(info.release_group.as_deref(), Some("PTer"));
    }
}
