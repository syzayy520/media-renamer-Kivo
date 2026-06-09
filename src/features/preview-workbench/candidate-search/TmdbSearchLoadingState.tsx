// features/preview-workbench/candidate-search/TmdbSearchLoadingState.tsx — TMDb 搜索加载状态（Mock UI）
// 职责：展示搜索加载中的状态
// 安全边界：不调用 backend command，不 invoke，不 fetch

/**
 * TMDb 搜索加载状态 Mock UI
 * 展示搜索进行中的加载动画和提示信息
 */
export function TmdbSearchLoadingState() {
  return (
    <div className="flex flex-col items-center justify-center py-8 text-center">
      {/* 加载动画 */}
      <div className="mb-4">
        <div className="h-8 w-8 animate-spin rounded-full border-2 border-blue-400/30 border-t-blue-400/80" />
      </div>

      {/* 加载标题 */}
      <div className="mb-2 text-sm font-medium text-white/60">
        Searching TMDb...
      </div>

      {/* 加载说明 */}
      <div className="max-w-[250px] text-xs text-white/40">
        This is a mock loading state for P2-010. No network request is sent.
      </div>

      {/* 进度占位 */}
      <div className="mt-4 w-full max-w-[200px]">
        <div className="h-1 overflow-hidden rounded-full bg-white/10">
          <div className="h-full w-1/2 animate-pulse rounded-full bg-blue-400/50" />
        </div>
      </div>
    </div>
  );
}
