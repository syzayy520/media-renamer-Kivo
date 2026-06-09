// features/preview-workbench/candidate-search/TmdbSearchErrorState.tsx — TMDb 搜索错误状态（Mock UI）
// 职责：展示搜索错误的状态
// 安全边界：不调用 backend command，不 invoke，不 fetch

/**
 * TMDb 搜索错误状态 Mock UI
 * 展示搜索失败时的错误信息和重试按钮（disabled）
 */
export function TmdbSearchErrorState() {
  const mockError = {
    code: 'MOCK_RATE_LIMIT',
    message: 'Rate limit exceeded. Please try again later. (Mock error for P2-010)',
    retryable: true,
  };

  return (
    <div className="flex flex-col items-center justify-center py-8 text-center">
      {/* 错误图标 */}
      <div className="mb-3 text-4xl text-red-400/50">⚠️</div>

      {/* 错误标题 */}
      <div className="mb-2 text-sm font-medium text-red-400/80">
        Search Failed
      </div>

      {/* 错误信息 */}
      <div className="mb-4 max-w-[250px] text-xs text-white/60">
        {mockError.message}
      </div>

      {/* 错误代码 */}
      <div className="mb-4 rounded-md bg-white/5 px-3 py-2 text-xs text-white/40">
        Error Code: {mockError.code}
      </div>

      {/* 重试按钮（disabled） */}
      <button
        disabled
        className="rounded-md bg-red-500/20 px-4 py-2 text-xs font-medium text-red-400/50 disabled:cursor-not-allowed disabled:opacity-50"
      >
        Retry (disabled)
      </button>
    </div>
  );
}
