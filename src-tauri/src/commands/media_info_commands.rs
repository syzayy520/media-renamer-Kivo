use crate::media_info::{probe_media_info, MediaInfoProbeInput, MediaInfoProbeOutput};

#[tauri::command]
pub async fn probe_media_info_fields(input: MediaInfoProbeInput) -> Result<MediaInfoProbeOutput, String> {
    Ok(probe_media_info(input).await)
}
