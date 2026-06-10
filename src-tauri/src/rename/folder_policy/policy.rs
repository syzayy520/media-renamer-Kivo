use serde::{Deserialize, Serialize};

/// 文件夹策略枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FolderPolicy {
    /// 保持原结构：原来有文件夹保留，没有不建
    #[default]
    KeepOriginalStructure,
    /// 一片一夹：没有独立文件夹的电影自动创建目标文件夹
    OneMovieOneFolder,
    /// 规范已有文件夹：已有文件夹也按规则重命名
    NormalizeExistingFolders,
    /// 中文文件夹 + PT文件名
    ChineseFolderPtFile,
    /// 去除文件夹：移动到上级目录
    Flatten,
    /// 剧集目录结构：show/season 层级
    TvShowStructure,
    /// 不建文件夹：只改文件名
    NoFolder,
}

impl FolderPolicy {
    pub fn display_name(&self) -> &'static str {
        match self {
            FolderPolicy::KeepOriginalStructure => "保持原结构",
            FolderPolicy::OneMovieOneFolder => "一片一夹",
            FolderPolicy::NormalizeExistingFolders => "规范文件夹名",
            FolderPolicy::ChineseFolderPtFile => "中文文件夹+PT文件",
            FolderPolicy::Flatten => "去除文件夹",
            FolderPolicy::TvShowStructure => "剧集目录结构",
            FolderPolicy::NoFolder => "不建文件夹",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FolderPolicy::KeepOriginalStructure => {
                "保持原目录结构不变，原来有文件夹就保留，没有就不建。"
            }
            FolderPolicy::OneMovieOneFolder => "为没有独立文件夹的媒体自动创建目标文件夹。",
            FolderPolicy::NormalizeExistingFolders => "已有文件夹也按命名规则重命名。",
            FolderPolicy::ChineseFolderPtFile => "文件夹用中文命名，文件保留PT/BT原始命名。",
            FolderPolicy::Flatten => "一键去除文件夹，把媒体文件移动到上级目录（高危险操作）。",
            FolderPolicy::TvShowStructure => "剧集统一 show/season 层级结构。",
            FolderPolicy::NoFolder => "完全不创建文件夹，只修改文件名。",
        }
    }

    /// 是否为高风险操作（需要额外的 safety gate）
    pub fn is_high_risk(&self) -> bool {
        matches!(self, FolderPolicy::Flatten)
    }

    /// 是否需要创建文件夹
    pub fn may_create_folder(&self) -> bool {
        matches!(
            self,
            FolderPolicy::OneMovieOneFolder
                | FolderPolicy::NormalizeExistingFolders
                | FolderPolicy::ChineseFolderPtFile
                | FolderPolicy::TvShowStructure
        )
    }

    /// 是否需要重命名文件夹
    pub fn may_rename_folder(&self) -> bool {
        matches!(
            self,
            FolderPolicy::NormalizeExistingFolders
                | FolderPolicy::ChineseFolderPtFile
                | FolderPolicy::TvShowStructure
        )
    }

    /// 是否需要移动文件出文件夹
    pub fn may_move_out_of_folder(&self) -> bool {
        matches!(self, FolderPolicy::Flatten)
    }

    /// 所有可用策略
    pub fn all() -> Vec<FolderPolicy> {
        vec![
            FolderPolicy::KeepOriginalStructure,
            FolderPolicy::OneMovieOneFolder,
            FolderPolicy::NormalizeExistingFolders,
            FolderPolicy::ChineseFolderPtFile,
            FolderPolicy::Flatten,
            FolderPolicy::TvShowStructure,
            FolderPolicy::NoFolder,
        ]
    }
}

/// 文件夹策略配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FolderPolicyConfig {
    /// 选中的文件夹策略
    pub policy: FolderPolicy,
    /// 执行后是否清理空文件夹（默认关闭）
    pub clean_empty_folders_after: bool,
}

impl FolderPolicyConfig {
    pub fn new(policy: FolderPolicy) -> Self {
        Self {
            policy,
            clean_empty_folders_after: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_policies_have_display_name() {
        for p in FolderPolicy::all() {
            assert!(!p.display_name().is_empty());
            assert!(!p.description().is_empty());
        }
    }

    #[test]
    fn test_flatten_is_high_risk() {
        assert!(FolderPolicy::Flatten.is_high_risk());
        assert!(!FolderPolicy::KeepOriginalStructure.is_high_risk());
    }

    #[test]
    fn test_one_movie_one_folder_creates_folder() {
        assert!(FolderPolicy::OneMovieOneFolder.may_create_folder());
        assert!(!FolderPolicy::KeepOriginalStructure.may_create_folder());
    }

    #[test]
    fn test_flatten_moves_out_of_folder() {
        assert!(FolderPolicy::Flatten.may_move_out_of_folder());
        assert!(!FolderPolicy::OneMovieOneFolder.may_move_out_of_folder());
    }

    #[test]
    fn test_default_is_keep_original() {
        assert_eq!(FolderPolicy::default(), FolderPolicy::KeepOriginalStructure);
    }

    #[test]
    fn test_clean_empty_default_off() {
        let config = FolderPolicyConfig::default();
        assert!(!config.clean_empty_folders_after);
    }
}
