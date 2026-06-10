use super::policy::FolderPolicy;
use serde::{Deserialize, Serialize};

/// 文件夹操作类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FolderOperation {
    /// 保持原文件夹不变
    KeepFolder,
    /// 创建新文件夹
    CreateFolder,
    /// 重命名文件夹
    RenameFolder,
    /// 移动文件进入文件夹
    MoveFileIntoFolder,
    /// 移动文件出文件夹
    MoveFileOutOfFolder,
    /// 规范文件夹名
    NormalizeFolderName,
    /// 保留原始文件夹（PT模式下的特殊情况）
    PreserveOriginalFolder,
    /// 清理空文件夹（可选且默认关闭）
    CleanEmptyFolder,
}

/// 单个文件夹操作计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderOperationPlan {
    pub operation: FolderOperation,
    pub source_path: String,
    pub target_path: String,
    pub is_folder: bool,
    /// 操作顺序号（越小越先执行）
    pub order: u32,
    /// 是否可回滚
    pub reversible: bool,
}

/// 文件夹操作规划器
pub fn plan_folder_operations(
    policy: FolderPolicy,
    original_parent_dir: &str,
    target_folder_name: &str,
    has_existing_folder: bool,
    original_folder_name: Option<&str>,
) -> Vec<FolderOperationPlan> {
    let mut plans = Vec::new();

    match policy {
        FolderPolicy::KeepOriginalStructure => {
            // 不创建、不重命名、不移动
            // 只保留原结构不变
            if has_existing_folder {
                plans.push(FolderOperationPlan {
                    operation: FolderOperation::KeepFolder,
                    source_path: original_folder_name.unwrap_or("").to_string(),
                    target_path: original_folder_name.unwrap_or("").to_string(),
                    is_folder: true,
                    order: 0,
                    reversible: true,
                });
            }
        }
        FolderPolicy::OneMovieOneFolder => {
            if has_existing_folder {
                // 已有文件夹：保留文件夹（文件已经在里面）
                plans.push(FolderOperationPlan {
                    operation: FolderOperation::KeepFolder,
                    source_path: original_folder_name.unwrap_or("").to_string(),
                    target_path: original_folder_name.unwrap_or("").to_string(),
                    is_folder: true,
                    order: 0,
                    reversible: true,
                });
            } else {
                // 没有文件夹：创建文件夹，然后文件移入
                let target_dir = format!("{}\\{}", original_parent_dir, target_folder_name);
                plans.push(FolderOperationPlan {
                    operation: FolderOperation::CreateFolder,
                    source_path: String::new(),
                    target_path: target_dir.clone(),
                    is_folder: true,
                    order: 1,
                    reversible: true,
                });
                // 文件会在 execution 阶段自动移入
            }
        }
        FolderPolicy::NormalizeExistingFolders => {
            if has_existing_folder {
                let old_name = original_folder_name.unwrap_or("");
                let target_dir = format!("{}\\{}", original_parent_dir, target_folder_name);
                if old_name != target_folder_name {
                    plans.push(FolderOperationPlan {
                        operation: FolderOperation::NormalizeFolderName,
                        source_path: format!("{}\\{}", original_parent_dir, old_name),
                        target_path: target_dir,
                        is_folder: true,
                        order: 3, // 子文件先处理后改名
                        reversible: true,
                    });
                }
            } else {
                // 没有文件夹则创建
                let target_dir = format!("{}\\{}", original_parent_dir, target_folder_name);
                plans.push(FolderOperationPlan {
                    operation: FolderOperation::CreateFolder,
                    source_path: String::new(),
                    target_path: target_dir,
                    is_folder: true,
                    order: 1,
                    reversible: true,
                });
            }
        }
        FolderPolicy::ChineseFolderPtFile => {
            // 中文文件夹名 + PT文件名
            // 文件夹用中文，文件保留原PT命名
            if has_existing_folder {
                let old_name = original_folder_name.unwrap_or("");
                let target_dir = format!("{}\\{}", original_parent_dir, target_folder_name);
                plans.push(FolderOperationPlan {
                    operation: FolderOperation::NormalizeFolderName,
                    source_path: format!("{}\\{}", original_parent_dir, old_name),
                    target_path: target_dir,
                    is_folder: true,
                    order: 3,
                    reversible: true,
                });
            } else {
                let target_dir = format!("{}\\{}", original_parent_dir, target_folder_name);
                plans.push(FolderOperationPlan {
                    operation: FolderOperation::CreateFolder,
                    source_path: String::new(),
                    target_path: target_dir,
                    is_folder: true,
                    order: 1,
                    reversible: true,
                });
            }
        }
        FolderPolicy::Flatten => {
            if has_existing_folder {
                // 把子文件移到上级目录
                plans.push(FolderOperationPlan {
                    operation: FolderOperation::MoveFileOutOfFolder,
                    source_path: original_folder_name.unwrap_or("").to_string(),
                    target_path: original_parent_dir.to_string(),
                    is_folder: false,
                    order: 1,
                    reversible: true,
                });
                // 可选清理空文件夹（默认关闭）
            }
        }
        FolderPolicy::TvShowStructure => {
            // 创建 show/season 层级
            let show_dir = format!("{}\\{}", original_parent_dir, target_folder_name);
            plans.push(FolderOperationPlan {
                operation: FolderOperation::CreateFolder,
                source_path: String::new(),
                target_path: show_dir,
                is_folder: true,
                order: 1,
                reversible: true,
            });
            // season 文件夹由 TV 识别逻辑动态生成
        }
        FolderPolicy::NoFolder => {
            // 完全不创建文件夹
        }
    }

    plans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_original_with_folder_does_nothing() {
        let plans = plan_folder_operations(
            FolderPolicy::KeepOriginalStructure,
            "D:\\Movies",
            "第一滴血 (1982)",
            true,
            Some("Rambo.First.Blood"),
        );
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].operation, FolderOperation::KeepFolder);
    }

    #[test]
    fn test_keep_original_without_folder_does_nothing() {
        let plans = plan_folder_operations(
            FolderPolicy::KeepOriginalStructure,
            "D:\\Downloads",
            "第一滴血 (1982)",
            false,
            None,
        );
        assert!(plans.is_empty());
    }

    #[test]
    fn test_one_movie_one_folder_without_folder_creates() {
        let plans = plan_folder_operations(
            FolderPolicy::OneMovieOneFolder,
            "D:\\Downloads",
            "第一滴血 (1982)",
            false,
            None,
        );
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].operation, FolderOperation::CreateFolder);
        assert!(plans[0].target_path.contains("第一滴血 (1982)"));
    }

    #[test]
    fn test_one_movie_one_folder_with_folder_keeps() {
        let plans = plan_folder_operations(
            FolderPolicy::OneMovieOneFolder,
            "D:\\Movies",
            "第一滴血 (1982)",
            true,
            Some("Rambo"),
        );
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].operation, FolderOperation::KeepFolder);
    }

    #[test]
    fn test_normalize_existing_renames() {
        let plans = plan_folder_operations(
            FolderPolicy::NormalizeExistingFolders,
            "D:\\Movies",
            "第一滴血 (1982)",
            true,
            Some("Rambo.First.Blood"),
        );
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].operation, FolderOperation::NormalizeFolderName);
    }

    #[test]
    fn test_flatten_creates_move_out() {
        let plans = plan_folder_operations(
            FolderPolicy::Flatten,
            "D:\\Downloads",
            "第一滴血 (1982)",
            true,
            Some("Rambo"),
        );
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].operation, FolderOperation::MoveFileOutOfFolder);
    }

    #[test]
    fn test_no_folder_no_ops() {
        let plans = plan_folder_operations(
            FolderPolicy::NoFolder,
            "D:\\Downloads",
            "第一滴血 (1982)",
            true,
            Some("Rambo"),
        );
        assert!(plans.is_empty());
    }
}
