// features/audit/components/TaskList.tsx — 任务列表
// 职责：展示任务列表，支持选择

import type { RenameTask } from '../../../api/audit/types';
import { AuditStatusBadge } from './AuditStatusBadge';

interface TaskListProps {
  tasks: RenameTask[];
  selectedTaskId: string | null;
  onSelect: (id: string) => void;
}

export function TaskList({ tasks, selectedTaskId, onSelect }: TaskListProps) {
  return (
    <div className="overflow-y-auto rounded-lg border border-white/10">
      <table className="w-full table-auto border-collapse" aria-label="审计任务列表">
        <thead>
          <tr className="border-b border-white/10 bg-white/[0.03]">
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              状态
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              Task ID
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              文件数
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              时间
            </th>
          </tr>
        </thead>
        <tbody>
          {tasks.map((task) => (
            <tr
              key={task.id}
              tabIndex={0}
              aria-selected={selectedTaskId === task.id}
              onClick={() => onSelect(task.id)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  onSelect(task.id);
                }
              }}
              className={`cursor-pointer border-b border-white/5 outline-none transition-colors hover:bg-white/[0.02] focus-visible:ring-1 focus-visible:ring-blue-500/50 ${
                selectedTaskId === task.id ? 'bg-blue-600/10' : ''
              }`}
            >
              <td className="whitespace-nowrap px-4 py-2">
                <AuditStatusBadge status={task.status} />
              </td>
              <td className="max-w-[200px] truncate px-4 py-2 font-mono text-xs text-white/50">
                {task.id}
              </td>
              <td className="whitespace-nowrap px-4 py-2 text-xs text-white/40">
                {task.total_files}
              </td>
              <td className="whitespace-nowrap px-4 py-2 text-xs text-white/30">
                {formatTime(task.created_at)}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function formatTime(iso: string): string {
  try {
    return new Date(iso).toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  } catch {
    return iso.slice(0, 16);
  }
}
