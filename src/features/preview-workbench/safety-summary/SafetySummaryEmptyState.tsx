// features/preview-workbench/safety-summary/SafetySummaryEmptyState.tsx — 安全摘要空状态
// 职责：当没有选中文件时显示空状态提示

export function SafetySummaryEmptyState() {
  return (
    <div className="flex h-full flex-col items-center justify-center p-4 text-center">
      <div className="mb-3 rounded-full bg-white/5 p-3">
        <svg
          className="h-6 w-6 text-white/30"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
          />
        </svg>
      </div>
      <h3 className="mb-1 text-sm font-medium text-white/60">请选择左侧文件</h3>
      <p className="text-xs text-white/40">这里将显示 dry-run 安全摘要</p>
    </div>
  );
}
