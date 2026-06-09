// features/preview-workbench/file-list/FileQueuePanel.tsx — 左栏文件队列面板
// 职责：只读展示 scanStore 中的 preview items，集成全局选择状态

import { useMemo } from 'react';
import { useScanStore } from '../../scan/state/scanStore';
import { useWorkbenchSelectionStore } from '../state/workbenchSelectionStore';
import type { PipelineResult, RenamePreviewItem } from '../../../api/session/types';
import { FileQueueItem } from './FileQueueItem';
import { FileQueueEmptyState } from './FileQueueEmptyState';

// 本地类型：描述从 scanStore 读取的 result 字段
// 不直接导入 ScanState（未导出），只描述所需字段
interface ScanStoreResultSlice {
  result: PipelineResult | null;
}

export function FileQueuePanel() {
  const result = useScanStore((s: ScanStoreResultSlice) => s.result);
  const selectedPreviewId = useWorkbenchSelectionStore((s) => s.selectedPreviewId);
  const selectPreview = useWorkbenchSelectionStore((s) => s.selectPreview);

  // 从 scanStore 读取 preview items
  const previewItems: RenamePreviewItem[] = useMemo(() => result?.previews ?? [], [result]);

  // 计算统计信息
  const stats = useMemo(() => {
    const total = previewItems.length;
    const conflict = previewItems.filter((item) => item.conflicts.length > 0).length;
    const needsReview = previewItems.filter((item) => item.needs_manual_review).length;
    const safe = previewItems.filter(
      (item) => item.conflicts.length === 0 && !item.needs_manual_review,
    ).length;
    return { total, conflict, needsReview, safe };
  }, [previewItems]);

  // 无扫描结果 → 空状态
  if (!result || previewItems.length === 0) {
    return <FileQueueEmptyState />;
  }

  return (
    <div className="flex h-full flex-col">
      {/* 标题 */}
      <div className="mb-3 flex items-center justify-between">
        <span className="text-sm font-medium text-white/60">文件队列</span>
        <span className="rounded-full bg-white/5 px-2 py-0.5 text-xs text-white/40">
          {stats.total}
        </span>
      </div>

      {/* 统计摘要 */}
      <div className="mb-3 flex gap-2 text-xs">
        <span className="text-white/40">
          安全: <span className="text-green-400">{stats.safe}</span>
        </span>
        <span className="text-white/40">
          冲突: <span className="text-red-400">{stats.conflict}</span>
        </span>
        <span className="text-white/40">
          待审: <span className="text-amber-400">{stats.needsReview}</span>
        </span>
      </div>

      {/* 文件列表 */}
      <div className="flex-1 overflow-y-auto">
        {previewItems.map((item) => (
          <FileQueueItem
            key={item.id}
            item={item}
            isSelected={selectedPreviewId === item.id}
            onSelect={() => selectPreview(item.id)}
          />
        ))}
      </div>

      {/* 底部统计 */}
      <div className="mt-3 border-t border-white/10 pt-3 text-center text-xs text-white/30">
        {stats.total} 文件 · {stats.conflict} 冲突 · {stats.needsReview} 待审
      </div>
    </div>
  );
}
