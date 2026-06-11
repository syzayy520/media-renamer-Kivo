use crate::media_info::contracts::{MediaInfoProbeInput, MediaInfoProbeOutput};
use crate::media_info::probe::filename_media_info_parser::parse_filename_media_info;

pub async fn probe_media_info(input: MediaInfoProbeInput) -> MediaInfoProbeOutput {
    let mut warnings = Vec::new();
    let info = parse_filename_media_info(&input.file_path);

    if info.resolution.is_none() {
        warnings.push("resolution not detected from file name".to_string());
    }
    if info.video.codec.is_none() {
        warnings.push("video codec not detected from file name".to_string());
    }
    if info.audio.codec.is_none() {
        warnings.push("audio codec not detected from file name".to_string());
    }

    MediaInfoProbeOutput {
        success: true,
        file_path: input.file_path,
        info,
        warnings,
    }
}
