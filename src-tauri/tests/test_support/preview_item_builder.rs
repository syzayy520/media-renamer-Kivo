// PreviewItem 流式构建器
// 职责：为测试创建 RenamePreviewItem 实例

use app_lib::parse::movie_parser::{MediaType, ParsedMediaInfo};
use app_lib::rename::template::{MetadataSource, RenamePreviewItem};
use app_lib::scan::MediaItem;
use uuid::Uuid;

pub struct PreviewItemBuilder {
    id: String,
    source_path: String,
    target_path: String,
    media_type: MediaType,
    confidence: u8,
    needs_manual_review: bool,
    should_skip: bool,
    title: String,
    year: Option<u16>,
    season: Option<u16>,
    episode: Option<u16>,
}

#[allow(dead_code)]
impl PreviewItemBuilder {
    pub fn new(index: usize) -> Self {
        Self {
            id: format!("preview-{}", index),
            source_path: format!("C:\\media\\file_{}.mkv", index),
            target_path: format!("C:\\media\\renamed_{}.mkv", index),
            media_type: MediaType::Movie,
            confidence: 85,
            needs_manual_review: false,
            should_skip: false,
            title: format!("Movie {}", index),
            year: Some(2020),
            season: None,
            episode: None,
        }
    }

    pub fn source_path(mut self, path: &str) -> Self {
        self.source_path = path.to_string();
        self
    }

    pub fn target_path(mut self, path: &str) -> Self {
        self.target_path = path.to_string();
        self
    }

    pub fn media_type(mut self, mt: MediaType) -> Self {
        self.media_type = mt;
        self
    }

    pub fn confidence(mut self, c: u8) -> Self {
        self.confidence = c;
        self
    }

    pub fn needs_manual_review(mut self, v: bool) -> Self {
        self.needs_manual_review = v;
        self
    }

    pub fn should_skip(mut self, v: bool) -> Self {
        self.should_skip = v;
        self
    }

    pub fn title(mut self, t: &str) -> Self {
        self.title = t.to_string();
        self
    }

    pub fn year(mut self, y: Option<u16>) -> Self {
        self.year = y;
        self
    }

    pub fn season(mut self, s: Option<u16>) -> Self {
        self.season = s;
        self
    }

    pub fn episode(mut self, e: Option<u16>) -> Self {
        self.episode = e;
        self
    }

    pub fn build(self) -> RenamePreviewItem {
        let file_name = std::path::Path::new(&self.source_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown.mkv".to_string());

        let extension = std::path::Path::new(&self.source_path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_else(|| "mkv".to_string());

        RenamePreviewItem {
            id: self.id,
            parsed_info: ParsedMediaInfo {
                media_item: MediaItem {
                    id: format!("media-{}", Uuid::new_v4()),
                    file_path: self.source_path.clone(),
                    file_name,
                    extension,
                    file_size: 0,
                    is_video: true,
                    is_companion: false,
                },
                media_type: self.media_type.clone(),
                title: self.title,
                year: self.year,
                season: self.season,
                episode: self.episode,
                episode_end: None,
                episode_title: None,
                resolution: Some("1080p".to_string()),
                source: None,
                video_codec: None,
                audio_codec: None,
                group: None,
                confidence: self.confidence,
                rule_sources: vec!["test".to_string()],
                special_type: None,
                extra_type: None,
                extra_number: None,
            },
            source_path: self.source_path,
            original_name: "file.mkv".to_string(),
            proposed_name: "renamed.mkv".to_string(),
            target_path: self.target_path,
            media_type: self.media_type,
            confidence: self.confidence,
            needs_manual_review: self.needs_manual_review,
            should_skip: self.should_skip,
            conflicts: vec![],
            evidence: vec![],
            metadata_source: MetadataSource::LocalRule,
        }
    }
}
