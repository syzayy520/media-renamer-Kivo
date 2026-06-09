// features/settings/components/SettingsErrorPanel.tsx — 错误面板
// 职责：展示设置页面的错误信息

interface SettingsErrorPanelProps {
  message: string;
  onDismiss: () => void;
}

export function SettingsErrorPanel({
  message,
  onDismiss,
}: SettingsErrorPanelProps) {
  return (
    <div className="flex items-start gap-3 rounded-lg border border-red-500/20 bg-red-500/5 p-4">
      <p className="min-w-0 flex-1 break-all text-sm text-red-400">{message}</p>
      <button
        type="button"
        onClick={onDismiss}
        className="shrink-0 text-sm text-red-400/60 hover:text-red-400"
      >
        关闭
      </button>
    </div>
  );
}
