// features/preview-workbench/candidate-search/TmdbSearchEmptyState.tsx — TMDb 搜索空状态（Mock UI）
// 职责：展示搜索结果为空的状态
// 安全边界：不调用 backend command，不 invoke，不 fetch

/**
 * TMDb 搜索空状态 Mock UI
 * 展示搜索无结果时的提示信息
 */
export function TmdbSearchEmptyState() {
  return (
    <div className="flex flex-col items-center justify-center py-8 text-center">
      {/* 图标 */}
      <div className="mb-3 text-4xl text-white/20">🔍</div>

      {/* 标题 */}
      <div className="mb-2 text-sm font-medium text-white/60">
        No candidates found
      </div>

      {/* 说明 */}
      <div className="max-w-[250px] text-xs text-white/40">
        Try adjusting your search query or filters. This is a mock empty state for P2-010.
      </div>
    </div>
  );
}
