use crate::rename::naming_context::media_info_context_merge::{first_value, media_info_for_item};
use crate::rename::naming_context::release_name_context::{
    original_name_without_ext, original_release_name,
};
use crate::rename::naming_rule::TokenContext;
use crate::rename::template::RenamePreviewItem;

pub fn build_token_context(item: &RenamePreviewItem) -> TokenContext {
    let info = &item.parsed_info;
    let media_info = media_info_for_item(item);

    TokenContext {
        zh_title: clean_title(&info.title),
        english_title: None,
        original_title: None,
        original_name_without_ext: Some(original_name_without_ext(item)),
        original_release_name: Some(original_release_name(item)),
        year: info.year,
        release_date: None,
        air_date: None,
        season_year: None,
        resolution: first_value(info.resolution.clone(), media_info.resolution.clone()),
        source: first_value(info.source.clone(), media_info.source.clone()),
        edition: None,
        remux: None,
        video_codec: first_value(info.video_codec.clone(), media_info.video.codec.clone()),
        video_bit_depth: media_info.video.bit_depth.clone(),
        hdr_format: media_info.video.hdr_format.clone(),
        dolby_vision: media_info.video.dolby_vision,
        audio_codec: first_value(info.audio_codec.clone(), media_info.audio.codec.clone()),
        audio_channels: media_info.audio.channels.clone(),
        audio_language: media_info.audio.language.clone(),
        release_group: first_value(info.group.clone(), media_info.release_group.clone()),
        tmdb_id: None,
        imdb_id: None,
        tvdb_id: None,
        show_title: None,
        season: info.season.map(|s| s as u32),
        episode: info.episode.map(|e| vec![e as u32]),
        episode_title: info.episode_title.clone(),
        absolute_episode: None,
        season_title: None,
        ext: Some(info.media_item.extension.clone()),
        subtitle_language: None,
        file_role: None,
    }
}

fn clean_title(title: &str) -> Option<String> {
    if title.is_empty() || title == "Unknown" {
        None
    } else {
        Some(title.to_string())
    }
}
