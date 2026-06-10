// rename/candidate_apply/preview_apply_result 模块 - 候选应用结果类型
// 职责：定义候选应用的结果和错误类型

use crate::rename::template::RenamePreviewItem;
use serde::{Deserialize, Serialize};

/// 候选应用结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewApplyResult {
    /// 更新后的预览项
    pub updated_item: RenamePreviewItem,
    /// 应用是否成功
    pub success: bool,
    /// 应用消息
    pub message: String,
    /// 安全警告列表
    pub warnings: Vec<String>,
}

/// 候选应用错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreviewApplyError {
    /// 候选信息不完整
    IncompleteCandidate(String),
    /// 无法生成有效文件名
    InvalidFileName(String),
    /// 目标路径冲突
    TargetConflict(String),
    /// 未知错误
    Unknown(String),
}

impl std::fmt::Display for PreviewApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreviewApplyError::IncompleteCandidate(msg) => write!(f, "候选信息不完整: {}", msg),
            PreviewApplyError::InvalidFileName(msg) => write!(f, "无效文件名: {}", msg),
            PreviewApplyError::TargetConflict(msg) => write!(f, "目标路径冲突: {}", msg),
            PreviewApplyError::Unknown(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl std::error::Error for PreviewApplyError {}
