use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NormalizedMediaInfo {
    pub resolution: Option<String>,
    pub source: Option<String>,
    pub release_group: Option<String>,
    pub video: NormalizedVideoInfo,
    pub audio: NormalizedAudioInfo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NormalizedVideoInfo {
    pub codec: Option<String>,
    pub bit_depth: Option<String>,
    pub hdr_format: Option<String>,
    pub dolby_vision: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NormalizedAudioInfo {
    pub codec: Option<String>,
    pub channels: Option<String>,
    pub language: Option<String>,
}
