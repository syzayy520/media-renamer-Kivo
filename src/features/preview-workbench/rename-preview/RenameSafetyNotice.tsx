// features/preview-workbench/rename-preview/RenameSafetyNotice.tsx — 安全提示
// 职责：提示当前为 dry-run 只读预览，不会修改文件

export function RenameSafetyNotice() {
  return (
    <div className="mt-4 rounded-lg border border-amber-500/20 bg-amber-500/5 p-3">
      <div className="mb-1 text-xs font-medium text-amber-400">安全提示</div>
      <p className="text-xs text-white/40">
        当前仅展示 dry-run 预览，不会修改文件。
      </p>
      <p className="mt-1 text-xs text-white/20">
        所有操作均为只读，无真实文件修改。
      </p>
    </div>
  );
}
