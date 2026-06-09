// features/preview-workbench/rename-preview/RenamePreviewEmptyState.tsx — 重命名预览空状态
// 职责：无预览项时的提示

export function RenamePreviewEmptyState() {
  return (
    <div className="flex h-full flex-col items-center justify-center text-center">
      <div className="mb-3 text-sm text-white/40">暂无重命名预览</div>
      <p className="text-xs text-white/30">
        请选择左侧文件
      </p>
      <p className="mt-2 text-xs text-white/20">
        这里将显示 dry-run 重命名预览
      </p>
    </div>
  );
}
