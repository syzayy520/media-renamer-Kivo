// pipeline/result 模块 - 流水线结果类型定义
// 职责：定义 PipelineResult 及子结构，纯数据类型，无业务逻辑

use crate::rename::safety_checker::SafetyReport;
use crate::rename::template::RenamePreviewItem;
use serde::{Deserialize, Serialize};

/// 扫描摘要（从 ScanReport 精简，供前端展示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    /// 扫描目录
    pub scan_path: String,
    /// 视频文件数（进入重命名主链路）
    pub video_count: u32,
    /// 伴随文件数（仅统计）
    pub companion_count: u32,
    /// 权限错误数
    pub permission_error_count: u32,
    /// 扫描耗时（毫秒）
    pub scan_duration_ms: u64,
}

/// 流水线执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    /// 审计任务 ID（已写入数据库）
    pub task_id: String,
    /// 扫描摘要
    pub scan: ScanSummary,
    /// 重命名预览列表
    pub previews: Vec<RenamePreviewItem>,
    /// 安全检查报告
    pub safety: SafetyReport,
    /// 成功解析的文件数
    pub parsed_count: u32,
    /// 未识别的文件数
    pub unknown_count: u32,
}
