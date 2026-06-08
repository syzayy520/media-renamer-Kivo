// features/audit/components/AuditLogList.tsx — 审计日志列表
// 职责：展示选中任务的审计日志

import type { AuditLogEntry } from '../../../api/audit/types';

interface AuditLogListProps {
  logs: AuditLogEntry[];
}

const eventTypeLabels: Record<string, string> = {
  task_created: '任务创建',
  scan_started: '扫描开始',
  scan_complete: '扫描完成',
  preview_generated: '预览生成',
  execute_started: '执行开始',
  execute_completed: '执行完成',
  rollback_started: '回滚开始',
  rollback_completed: '回滚完成',
};

export function AuditLogList({ logs }: AuditLogListProps) {
  if (logs.length === 0) {
    return (
      <div className="rounded-lg border border-white/5 bg-white/[0.02] p-6 text-center">
        <p className="text-sm text-white/30">暂无审计日志</p>
      </div>
    );
  }

  return (
    <div className="space-y-1.5">
      {logs.map((log) => (
        <div
          key={log.id}
          className="flex items-start gap-3 rounded border border-white/5 bg-white/[0.02] px-4 py-3"
        >
          <span className="shrink-0 rounded bg-white/5 px-1.5 py-0.5 text-[10px] font-mono text-white/30">
            {formatTime(log.created_at)}
          </span>
          <span className="shrink-0 rounded bg-blue-600/10 px-1.5 py-0.5 text-[10px] text-blue-400">
            {eventTypeLabels[log.event_type] ?? log.event_type}
          </span>
          <span className="min-w-0 flex-1 break-all text-xs text-white/50">
            {log.message}
          </span>
        </div>
      ))}
    </div>
  );
}

function formatTime(iso: string): string {
  try {
    return new Date(iso).toLocaleString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  } catch {
    return iso.slice(11, 19);
  }
}
