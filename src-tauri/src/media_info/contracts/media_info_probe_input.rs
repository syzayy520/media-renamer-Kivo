use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MediaInfoProbeInput {
    pub file_path: String,
}
