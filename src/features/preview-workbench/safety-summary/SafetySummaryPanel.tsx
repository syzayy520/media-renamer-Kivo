// features/preview-workbench/safety-summary/SafetySummaryPanel.tsx — 安全面板
// 职责：展示当前选中文件的 dry-run 安全摘要，只读展示
// 禁止：invoke / execute_rename / rollback / 真实导出

import { useMemo } from 'react';
import { useScanStore } from '../../scan/state/scanStore';
import { useWorkbenchSelectionStore } from '../state/workbenchSelectionStore';
import type { RenamePreviewItem, SafetyReport } from '../../../api/session/types';
import { SafetySummaryEmptyState } from './SafetySummaryEmptyState';
import { SafetySummaryItem } from './SafetySummaryItem';

export function SafetySummaryPanel() {
  const result = useScanStore(
    (s: { result: { previews: RenamePreviewItem[]; safety: SafetyReport } | null }) => s.result,
  );
  const selectedPreviewId = useWorkbenchSelectionStore((s) => s.selectedPreviewId);

  // 根据 selectedPreviewId 查找对应的 preview item
  const previewItem = useMemo(() => {
    if (!result?.previews || !selectedPreviewId) {
      return null;
    }
    return result.previews.find((item) => item.id === selectedPreviewId) ?? null;
  }, [result, selectedPreviewId]);

  // 获取全局安全报告
  const globalSafety = result?.safety ?? null;

  // 如果没有选中项，显示空状态
  if (!selectedPreviewId || !previewItem) {
    return <SafetySummaryEmptyState />;
  }

  // 计算安全摘要数据
  const hasConflicts = previewItem.conflicts.length > 0;
  const conflictCount = previewItem.conflicts.length;
  const needsReview = previewItem.needs_manual_review;
  const shouldSkip = previewItem.should_skip;
  const confidence = previewItem.confidence;
  const metadataSource = previewItem.metadata_source;

  // 获取状态颜色
  const getStatusColor = () => {
    if (hasConflicts) return 'text-red-400';
    if (needsReview) return 'text-amber-400';
    if (shouldSkip) return 'text-gray-400';
    return 'text-green-400';
  };

  // 获取状态文本
  const getStatusText = () => {
    if (hasConflicts) return '冲突';
    if (needsReview) return '待审';
    if (shouldSkip) return '跳过';
    return '安全';
  };

  return (
    <div className="flex h-full flex-col">
      {/* 标题 */}
      <div className="mb-3 flex items-center justify-between">
        <span className="text-sm font-medium text-white/60">安全摘要</span>
        <span className="rounded-full bg-blue-500/20 px-2.5 py-0.5 text-xs font-medium text-blue-400">
          Dry-run Only
        </span>
      </div>

      {/* 安全摘要内容 */}
      <div className="flex-1 space-y-2 overflow-y-auto">
        {/* 状态概览 */}
        <div className="rounded-md bg-white/5 p-3">
          <div className="mb-2 flex items-center justify-between">
            <span className="text-xs text-white/40">当前状态</span>
            <span className={`text-xs font-medium ${getStatusColor()}`}>
              {getStatusText()}
            </span>
          </div>
          <div className="text-xs text-white/30">
            {hasConflicts
              ? `发现 ${conflictCount} 个冲突，需要处理`
              : needsReview
                ? '需要人工审核'
                : shouldSkip
                  ? '文件将被跳过'
                  : '文件可以安全重命名'}
          </div>
        </div>

        {/* 详细信息 */}
        <div className="space-y-2">
          <SafetySummaryItem
            label="Dry-run 模式"
            value={true}
            isBoolean={true}
            statusColor="text-blue-400"
          />

          <SafetySummaryItem
            label="置信度"
            value={`${confidence}%`}
            statusColor={
              confidence >= 80
                ? 'text-green-400'
                : confidence >= 60
                  ? 'text-amber-400'
                  : 'text-red-400'
            }
          />

          <SafetySummaryItem
            label="元数据来源"
            value={metadataSource === 'LocalRule' ? '本地规则' : metadataSource === 'Tmdb' ? 'TMDb' : '手动'}
            statusColor="text-white/60"
          />

          <SafetySummaryItem
            label="有冲突"
            value={hasConflicts}
            isBoolean={true}
            statusColor={hasConflicts ? 'text-red-400' : 'text-green-400'}
          />

          <SafetySummaryItem
            label="需要审核"
            value={needsReview}
            isBoolean={true}
            statusColor={needsReview ? 'text-amber-400' : 'text-green-400'}
          />

          <SafetySummaryItem
            label="将跳过"
            value={shouldSkip}
            isBoolean={true}
            statusColor={shouldSkip ? 'text-gray-400' : 'text-green-400'}
          />
        </div>

        {/* 文件路径 */}
        <div className="rounded-md bg-white/5 p-3">
          <div className="mb-1 text-xs text-white/40">源文件</div>
          <div className="truncate text-xs text-white/60" title={previewItem.source_path}>
            {previewItem.source_path}
          </div>
        </div>

        <div className="rounded-md bg-white/5 p-3">
          <div className="mb-1 text-xs text-white/40">目标文件</div>
          <div className="truncate text-xs text-white/60" title={previewItem.target_path}>
            {previewItem.target_path}
          </div>
        </div>

        {/* 全局安全状态 */}
        {globalSafety && (
          <div className="rounded-md bg-white/5 p-3">
            <div className="mb-1 text-xs text-white/40">全局安全状态</div>
            <div className="flex items-center gap-2">
              <span
                className={`text-xs font-medium ${
                  globalSafety.can_execute ? 'text-green-400' : 'text-red-400'
                }`}
              >
                {globalSafety.can_execute ? '可执行' : '不可执行'}
              </span>
              <span className="text-xs text-white/30">
                {globalSafety.dry_run ? '(Dry-run)' : '(真实执行)'}
              </span>
            </div>
            {globalSafety.blocking_reasons.length > 0 && (
              <div className="mt-1 text-xs text-red-400">
                阻止原因: {globalSafety.blocking_reasons.join(', ')}
              </div>
            )}
          </div>
        )}
      </div>

      {/* 底部提示 */}
      <div className="mt-3 border-t border-white/10 pt-3 text-center text-xs text-white/30">
        所有操作均为只读预览，不会执行真实文件更改
      </div>
    </div>
  );
}
