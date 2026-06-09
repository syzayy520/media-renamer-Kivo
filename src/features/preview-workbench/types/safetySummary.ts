// features/preview-workbench/types/safetySummary.ts — Safety Summary 类型
// 职责：定义安全摘要相关的类型，确保类型安全

import type { RenamePreviewItem, SafetyReport } from '../../../api/session/types';

/**
 * 安全摘要数据接口
 * 用于展示当前选中文件的安全状态
 */
export interface SafetySummaryData {
  /** 是否为 dry-run 模式 */
  isDryRun: boolean;
  /** 是否有冲突 */
  hasConflicts: boolean;
  /** 冲突数量 */
  conflictCount: number;
  /** 是否需要人工审核 */
  needsReview: boolean;
  /** 是否应该跳过 */
  shouldSkip: boolean;
  /** 置信度 (0-100) */
  confidence: number;
  /** 源文件路径 */
  sourcePath: string;
  /** 目标文件路径 */
  targetPath: string;
  /** 元数据来源 */
  metadataSource: 'LocalRule' | 'Tmdb' | 'Manual';
  /** 全局安全报告（如果存在） */
  globalSafety: SafetyReport | null;
}

/**
 * 安全摘要面板属性
 */
export interface SafetySummaryPanelProps {
  /** 当前选中的预览项 */
  previewItem: RenamePreviewItem | null;
  /** 全局安全报告 */
  globalSafety: SafetyReport | null;
  /** 是否有选中项 */
  hasSelection: boolean;
}

/**
 * 安全摘要项属性
 */
export interface SafetySummaryItemProps {
  /** 标签 */
  label: string;
  /** 值 */
  value: string | number | boolean;
  /** 状态颜色 */
  statusColor?: string;
  /** 是否为布尔值 */
  isBoolean?: boolean;
}
