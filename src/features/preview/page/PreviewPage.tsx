// features/preview/page/PreviewPage.tsx — 预览页面
// 职责：展示 Dry-run Preview 结果表格

import { useMemo } from 'react';
import { useScanStore } from '../../scan/state/scanStore';
import { usePreviewFilterStore } from '../state/previewFilterStore';
import { PreviewEmptyState } from '../components/PreviewEmptyState';
import { PreviewToolbar } from '../components/PreviewToolbar';
import { PreviewStats } from '../components/PreviewStats';
import { PreviewTable } from '../components/PreviewTable';

export function PreviewPage() {
  const result = useScanStore((s) => s.result);
  const { searchText, setSearchText, activeFilter, setActiveFilter } =
    usePreviewFilterStore();

  // 筛选逻辑
  const filtered = useMemo(() => {
    if (!result) return [];
    let items = result.previews;

    if (searchText.trim()) {
      const q = searchText.toLowerCase();
      items = items.filter(
        (item) =>
          item.original_name.toLowerCase().includes(q) ||
          item.proposed_name.toLowerCase().includes(q),
      );
    }

    switch (activeFilter) {
      case 'conflict':
        items = items.filter((item) => item.conflicts.length > 0);
        break;
      case 'needs_review':
        items = items.filter((item) => item.needs_manual_review);
        break;
      case 'safe':
        items = items.filter(
          (item) => item.conflicts.length === 0 && !item.needs_manual_review,
        );
        break;
    }

    return items;
  }, [result, searchText, activeFilter]);

  // 无结果 → 空状态
  if (!result || result.previews.length === 0) {
    return <PreviewEmptyState />;
  }

  const totalCount = result.previews.length;

  return (
    <div className="space-y-4">
      {/* 标题 + Dry-run 安全说明 */}
      <div className="rounded-lg border border-white/10 bg-white/5 p-4">
        <h2 className="mb-1 text-lg font-semibold">重命名预览</h2>
        <p className="text-sm text-white/50">
          当前仅展示预览，不会真实修改媒体文件。
        </p>
        {!result.safety.can_execute && (
          <div className="mt-3 rounded-lg border border-amber-500/20 bg-amber-500/5 px-4 py-2">
            <p className="text-sm text-amber-400">
              当前预览存在阻塞项，仅供检查，不会执行真实重命名。
            </p>
          </div>
        )}
      </div>

      {/* 统计摘要 */}
      <PreviewStats result={result} />

      {/* 工具栏 */}
      <PreviewToolbar
        searchText={searchText}
        onSearchChange={setSearchText}
        activeFilter={activeFilter}
        onFilterChange={setActiveFilter}
        totalCount={totalCount}
        filteredCount={filtered.length}
      />

      {/* 表格 */}
      <PreviewTable items={filtered} />
    </div>
  );
}
