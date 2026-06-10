use super::file_item::PreviewFileItem;
use super::group_status::GroupStatus;
use serde::{Deserialize, Serialize};

/// 媒体类型分组
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupMediaType {
    Movie,
    Tv,
    Anime,
    Mixed,
    Unknown,
}

impl GroupMediaType {
    pub fn display_name(&self) -> &'static str {
        match self {
            GroupMediaType::Movie => "电影",
            GroupMediaType::Tv => "电视剧",
            GroupMediaType::Anime => "动漫",
            GroupMediaType::Mixed => "混合",
            GroupMediaType::Unknown => "未知",
        }
    }
}

/// 文件夹组 — 代表一个逻辑媒体单元（一部电影 / 一季电视剧）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderGroup {
    /// 组唯一 ID
    pub id: String,
    /// 组媒体类型
    pub media_type: GroupMediaType,
    /// 原始文件夹名或位置
    pub original_folder_name: String,
    /// 目标文件夹名
    pub target_folder_name: String,
    /// 原始路径
    pub original_path: String,
    /// 目标路径
    pub target_path: String,
    /// 文件数量
    pub file_count: u32,
    /// TMDb 匹配状态
    pub tmdb_match_status: TmdbMatchStatus,
    /// 文件夹策略
    pub folder_policy: Option<String>,
    /// 命名规则预设
    pub naming_preset: Option<String>,
    /// 组级状态
    pub status: GroupStatus,
    /// 是否跳过整组
    pub should_skip: bool,
    /// 是否需要人工审核
    pub needs_manual_review: bool,
    /// 子文件列表
    pub children: Vec<PreviewFileItem>,
    /// 折叠状态（前端控制，后端不持久化）
    #[serde(skip)]
    pub collapsed: bool,
}

/// TMDb 匹配状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TmdbMatchStatus {
    /// 未搜索
    NotSearched,
    /// 已匹配
    Matched,
    /// 未匹配
    NotFound,
    /// 多个候选
    MultipleCandidates,
    /// 需审核
    NeedsReview,
}

impl TmdbMatchStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            TmdbMatchStatus::NotSearched => "未搜索",
            TmdbMatchStatus::Matched => "已匹配",
            TmdbMatchStatus::NotFound => "未找到",
            TmdbMatchStatus::MultipleCandidates => "多个候选",
            TmdbMatchStatus::NeedsReview => "需审核",
        }
    }
}

/// 预览计划树根 — 包含所有文件夹组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewPlanTree {
    pub groups: Vec<FolderGroup>,
    pub total_files: u32,
    pub total_groups: u32,
    pub skipped_groups: u32,
    pub blocked_groups: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_media_type_display() {
        assert_eq!(GroupMediaType::Movie.display_name(), "电影");
        assert_eq!(GroupMediaType::Tv.display_name(), "电视剧");
        assert_eq!(GroupMediaType::Anime.display_name(), "动漫");
    }

    #[test]
    fn test_tmdb_match_status_display() {
        assert_eq!(TmdbMatchStatus::NotSearched.display_name(), "未搜索");
        assert_eq!(TmdbMatchStatus::Matched.display_name(), "已匹配");
    }
}
