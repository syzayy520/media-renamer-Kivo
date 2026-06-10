// rename/template 模块 - 模板渲染功能族
// 职责：重命名模板渲染、默认模板、扩展名处理
// 委托：render.rs / defaults.rs / extension_token.rs

pub mod defaults;
pub mod extension_token;
pub mod render;

// Re-export types and functions
pub use defaults::get_default_template;
pub use render::render;

use crate::parse::confidence::RuleMatchEvidence;
use crate::parse::movie_parser::{MediaType, ParsedMediaInfo};
use crate::shared::result_types::ConflictType;
use serde::{Deserialize, Serialize};

/// 元数据来源
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetadataSource {
    /// 本地规则解析
    LocalRule,
    /// TMDb API
    Tmdb,
    /// 手动输入
    Manual,
}

/// 重命名冲突
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameConflict {
    /// 冲突类型
    pub conflict_type: ConflictType,
    /// 源路径
    pub source_path: String,
    /// 目标路径
    pub target_path: String,
    /// 冲突描述
    pub message: String,
    /// 是否阻塞（阻塞则不能执行）
    pub blocking: bool,
}

/// 重命名预览项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenamePreviewItem {
    /// 预览 ID
    pub id: String,
    /// 解析信息
    pub parsed_info: ParsedMediaInfo,
    /// 原始路径
    pub source_path: String,
    /// 原始文件名
    pub original_name: String,
    /// 提议的新文件名
    pub proposed_name: String,
    /// 目标路径
    pub target_path: String,
    /// 媒体类型
    pub media_type: MediaType,
    /// 置信度
    pub confidence: u8,
    /// 是否需要人工确认
    pub needs_manual_review: bool,
    /// 是否跳过
    pub should_skip: bool,
    /// 冲突列表
    pub conflicts: Vec<RenameConflict>,
    /// 置信度证据
    pub evidence: Vec<RuleMatchEvidence>,
    /// 元数据来源
    pub metadata_source: MetadataSource,
}
