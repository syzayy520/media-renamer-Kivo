use serde::Serialize;

use super::normalized_media_info::NormalizedMediaInfo;

#[derive(Debug, Clone, Serialize)]
pub struct MediaInfoProbeOutput {
    pub success: bool,
    pub file_path: String,
    pub info: NormalizedMediaInfo,
    pub warnings: Vec<String>,
}
