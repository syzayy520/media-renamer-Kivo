// features/preview-workbench/rename-preview/RenamePreviewPanel.tsx — 右栏重命名预览面板
// 职责：只读展示 scanStore 中的 preview item 详情，不执行 rename
// 集成选择状态：根据 selectedPreviewId 查找对应 preview item
// 集成安全摘要：在预览下方显示安全摘要面板

import { useMemo } from 'react';
import { useScanStore } from '../../scan/state/scanStore';
import { useWorkbenchSelectionStore } from '../state/workbenchSelectionStore';
import type { RenamePreviewItem, SafetyReport } from '../../../api/session/types';
import { RenamePreviewEmptyState } from './RenamePreviewEmptyState';
import { RenamePathPreview } from './RenamePathPreview';
import { RenameSafetyNotice } from './RenameSafetyNotice';
import { SafetySummaryPanel } from '../safety-summary/SafetySummaryPanel';

export function RenamePreviewPanel() {
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

  // 如果没有选中预览项，显示空状态
  if (!previewItem) {
    return <RenamePreviewEmptyState />;
  }

  return (
    <div className="flex h-full flex-col">
      {/* 标题 */}
      <div className="mb-3 flex items-center justify-between">
        <span className="text-sm font-medium text-white/60">重命名预览</span>
        <span className="rounded-full bg-white/5 px-2 py-0.5 text-xs text-white/40">
          只读预览
        </span>
      </div>

      {/* 原文件名 */}
      <div className="mb-3">
        <div className="mb-1 text-xs text-white/40">原文件名</div>
        <div className="rounded bg-white/5 px-3 py-2 text-sm text-white/80">
          {previewItem.original_name ?? '(未知)'}
        </div>
      </div>

      {/* 建议新文件名 */}
      <div className="mb-3">
        <div className="mb-1 text-xs text-white/40">建议新文件名</div>
        <div className="rounded bg-blue-500/10 px-3 py-2 text-sm text-blue-300">
          {previewItem.proposed_name ?? '(未生成)'}
        </div>
      </div>

      {/* 路径预览 */}
      <RenamePathPreview item={previewItem} />

      {/* 状态信息 */}
      <div className="mb-3 flex gap-2 text-xs">
        <span className="text-white/40">
          状态: <span className={getStatusColor(previewItem)}>{getStatusText(previewItem)}</span>
        </span>
        <span className="text-white/40">
          置信度: <span className="text-white/60">{previewItem.confidence ?? 0}%</span>
        </span>
      </div>

      {/* 安全提示 */}
      <RenameSafetyNotice />

      {/* 安全摘要面板 */}
      <div className="mt-4 border-t border-white/10 pt-4">
        <SafetySummaryPanel />
      </div>
    </div>
  );
}

function getStatusColor(item: RenamePreviewItem): string {
  if (item.conflicts.length > 0) return 'text-red-400';
  if (item.needs_manual_review) return 'text-amber-400';
  return 'text-green-400';
}

function getStatusText(item: RenamePreviewItem): string {
  if (item.conflicts.length > 0) return '冲突';
  if (item.needs_manual_review) return '待审';
  return '安全';
}
