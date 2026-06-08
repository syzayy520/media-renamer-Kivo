// features/audit/components/AuditErrorPanel.tsx — 错误面板
// 职责：展示审计页面的错误信息

interface AuditErrorPanelProps {
  message: string;
  onDismiss: () => void;
}

export function AuditErrorPanel({ message, onDismiss }: AuditErrorPanelProps) {
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
