use serde::{Deserialize, Serialize};

/// 组级状态 — 聚合子文件状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupStatus {
    /// 就绪 — 所有子文件安全、无冲突
    Ready,
    /// 需审核 — 有子文件需要人工审核
    NeedsReview,
    /// 冲突 — 有子文件目标路径冲突
    Conflict,
    /// 已跳过 — 整组被标记跳过
    Skipped,
    /// 阻断 — 存在阻断级错误
    Blocker,
    /// 部分跳过 — 仅部分子文件跳过
    PartialSkipped,
}

impl GroupStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            GroupStatus::Ready => "就绪",
            GroupStatus::NeedsReview => "需审核",
            GroupStatus::Conflict => "冲突",
            GroupStatus::Skipped => "已跳过",
            GroupStatus::Blocker => "阻断",
            GroupStatus::PartialSkipped => "部分跳过",
        }
    }

    /// 是否可执行
    pub fn is_executable(&self) -> bool {
        matches!(self, GroupStatus::Ready | GroupStatus::PartialSkipped)
    }

    /// 是否需要显示警告
    pub fn has_warning(&self) -> bool {
        matches!(
            self,
            GroupStatus::NeedsReview | GroupStatus::Conflict | GroupStatus::PartialSkipped
        )
    }

    /// 是否为阻断状态
    pub fn is_blocker(&self) -> bool {
        matches!(self, GroupStatus::Blocker)
    }
}

/// 从子文件状态列表聚合为组状态
pub fn aggregate_group_status(
    child_statuses: &[crate::rename::plan_tree::file_item::FileSafetyStatus],
    group_skipped: bool,
) -> GroupStatus {
    if group_skipped {
        return GroupStatus::Skipped;
    }

    if child_statuses.is_empty() {
        return GroupStatus::Ready;
    }

    let has_blocker = child_statuses.iter().any(|s| {
        matches!(
            s,
            crate::rename::plan_tree::file_item::FileSafetyStatus::Blocker
        )
    });
    let has_conflict = child_statuses.iter().any(|s| {
        matches!(
            s,
            crate::rename::plan_tree::file_item::FileSafetyStatus::Conflict
        )
    });
    let has_needs_review = child_statuses.iter().any(|s| {
        matches!(
            s,
            crate::rename::plan_tree::file_item::FileSafetyStatus::NeedsReview
        )
    });
    let has_skipped = child_statuses.iter().any(|s| {
        matches!(
            s,
            crate::rename::plan_tree::file_item::FileSafetyStatus::Skipped
        )
    });
    let all_skipped = child_statuses.iter().all(|s| {
        matches!(
            s,
            crate::rename::plan_tree::file_item::FileSafetyStatus::Skipped
        )
    });

    if has_blocker {
        GroupStatus::Blocker
    } else if all_skipped {
        GroupStatus::Skipped
    } else if has_skipped {
        GroupStatus::PartialSkipped
    } else if has_conflict {
        GroupStatus::Conflict
    } else if has_needs_review {
        GroupStatus::NeedsReview
    } else {
        GroupStatus::Ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rename::plan_tree::file_item::FileSafetyStatus;

    #[test]
    fn test_all_ready() {
        let status =
            aggregate_group_status(&[FileSafetyStatus::Ready, FileSafetyStatus::Ready], false);
        assert_eq!(status, GroupStatus::Ready);
        assert!(status.is_executable());
    }

    #[test]
    fn test_has_blocker() {
        let status =
            aggregate_group_status(&[FileSafetyStatus::Ready, FileSafetyStatus::Blocker], false);
        assert_eq!(status, GroupStatus::Blocker);
        assert!(!status.is_executable());
    }

    #[test]
    fn test_group_skipped() {
        let status =
            aggregate_group_status(&[FileSafetyStatus::Ready, FileSafetyStatus::Ready], true);
        assert_eq!(status, GroupStatus::Skipped);
    }

    #[test]
    fn test_partial_skipped() {
        let status =
            aggregate_group_status(&[FileSafetyStatus::Ready, FileSafetyStatus::Skipped], false);
        assert_eq!(status, GroupStatus::PartialSkipped);
    }

    #[test]
    fn test_has_conflict() {
        let status = aggregate_group_status(
            &[FileSafetyStatus::Ready, FileSafetyStatus::Conflict],
            false,
        );
        assert_eq!(status, GroupStatus::Conflict);
    }

    #[test]
    fn test_needs_review() {
        let status = aggregate_group_status(
            &[FileSafetyStatus::Ready, FileSafetyStatus::NeedsReview],
            false,
        );
        assert_eq!(status, GroupStatus::NeedsReview);
    }

    #[test]
    fn test_empty_children() {
        let status = aggregate_group_status(&[], false);
        assert_eq!(status, GroupStatus::Ready);
    }
}
