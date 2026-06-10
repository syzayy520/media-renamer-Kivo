use serde::{Deserialize, Serialize};

/// 文件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileRole {
    /// 主视频文件
    MainVideo,
    /// 字幕文件
    Subtitle,
    /// 图片文件（海报/同人画）
    Image,
    /// NFO 元数据文件
    Nfo,
    /// 额外文件
    Extra,
    /// 未知
    Unknown,
}

impl FileRole {
    pub fn display_name(&self) -> &'static str {
        match self {
            FileRole::MainVideo => "主视频",
            FileRole::Subtitle => "字幕",
            FileRole::Image => "图片",
            FileRole::Nfo => "NFO",
            FileRole::Extra => "额外",
            FileRole::Unknown => "未知",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            FileRole::MainVideo => "🎞",
            FileRole::Subtitle => "📝",
            FileRole::Image => "🖼",
            FileRole::Nfo => "📄",
            FileRole::Extra => "📦",
            FileRole::Unknown => "❓",
        }
    }
}

/// 子文件项 — 文件夹组中的单个文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewFileItem {
    /// 文件唯一 ID
    pub id: String,
    /// 文件角色/类型
    pub file_role: FileRole,
    /// 原始文件名
    pub original_name: String,
    /// 目标文件名
    pub target_name: String,
    /// 扩展名
    pub extension: String,
    /// 字幕语言（仅字幕文件）
    pub subtitle_language: Option<String>,
    /// 原始路径
    pub original_path: String,
    /// 目标路径
    pub target_path: String,
    /// 是否跳过
    pub should_skip: bool,
    /// 是否需要人工审核
    pub needs_manual_review: bool,
    /// 元数据来源
    pub metadata_source: MetadataSource,
    /// 安全状态
    pub safety_status: FileSafetyStatus,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f32,
}

/// 元数据来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetadataSource {
    Parsed,
    Tmdb,
    Manual,
    Preserved,
}

/// 文件安全状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileSafetyStatus {
    Ready,
    NeedsReview,
    Conflict,
    Skipped,
    Blocker,
}

impl FileSafetyStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            FileSafetyStatus::Ready => "就绪",
            FileSafetyStatus::NeedsReview => "需审核",
            FileSafetyStatus::Conflict => "冲突",
            FileSafetyStatus::Skipped => "已跳过",
            FileSafetyStatus::Blocker => "阻断",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_role_display() {
        assert_eq!(FileRole::MainVideo.display_name(), "主视频");
        assert_eq!(FileRole::Subtitle.display_name(), "字幕");
    }

    #[test]
    fn test_file_role_icon() {
        assert_eq!(FileRole::MainVideo.icon(), "🎞");
        assert_eq!(FileRole::Image.icon(), "🖼");
    }

    #[test]
    fn test_safety_status_display() {
        assert_eq!(FileSafetyStatus::Ready.display_name(), "就绪");
        assert_eq!(FileSafetyStatus::Blocker.display_name(), "阻断");
    }
}
