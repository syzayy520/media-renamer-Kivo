// features/audit/components/AuditToolbar.tsx — 审计工具栏
// 职责：刷新按钮 + 任务计数

interface AuditToolbarProps {
  isLoading: boolean;
  taskCount: number;
  onRefresh: () => void;
}

export function AuditToolbar({
  isLoading,
  taskCount,
  onRefresh,
}: AuditToolbarProps) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-xs text-white/30">{taskCount} 个任务</span>
      <button
        type="button"
        disabled={isLoading}
        onClick={onRefresh}
        className={`inline-flex items-center gap-2 rounded-md px-4 py-1.5 text-xs transition-colors ${
          isLoading
            ? 'cursor-not-allowed text-white/20'
            : 'text-white/50 hover:bg-white/5 hover:text-white/70'
        }`}
      >
        {isLoading ? '加载中...' : '刷新'}
      </button>
    </div>
  );
}
