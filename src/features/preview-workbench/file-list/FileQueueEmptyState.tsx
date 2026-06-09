// features/preview-workbench/file-list/FileQueueEmptyState.tsx — 空状态
// 职责：无扫描结果时的提示

export function FileQueueEmptyState() {
  return (
    <div className="flex h-full flex-col items-center justify-center text-center">
      <div className="mb-3 text-3xl">📂</div>
      <div className="mb-1 text-sm font-medium text-white/50">暂无扫描结果</div>
      <div className="text-xs text-white/30">
        请先在扫描页完成 Dry-run 扫描
      </div>
    </div>
  );
}
