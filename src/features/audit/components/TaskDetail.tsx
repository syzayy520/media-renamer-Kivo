// features/audit/components/TaskDetail.tsx — 任务详情
// 职责：展示选中任务的完整信息

import type { RenameTask } from '../../../api/audit/types';
import { AuditStatusBadge } from './AuditStatusBadge';

interface TaskDetailProps {
  task: RenameTask;
}

export function TaskDetail({ task }: TaskDetailProps) {
  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-5">
      <div className="mb-4 flex items-center gap-2">
        <AuditStatusBadge status={task.status} />
        <span className="text-xs text-white/30">
          Dry-run preview（当前 UI 不执行真实重命名）
        </span>
      </div>

      <dl className="space-y-2">
        <Row label="Task ID" value={task.id} mono />
        <Row label="模板" value={task.template} />
        <Row label="文件数" value={String(task.total_files)} />
        <Row
          label="创建时间"
          value={new Date(task.created_at).toLocaleString('zh-CN')}
        />
        <Row
          label="更新时间"
          value={new Date(task.updated_at).toLocaleString('zh-CN')}
        />
        {task.error_message && (
          <Row label="错误信息" value={task.error_message} danger />
        )}
      </dl>
    </div>
  );
}

function Row({
  label,
  value,
  mono,
  danger,
}: {
  label: string;
  value: string;
  mono?: boolean;
  danger?: boolean;
}) {
  return (
    <div className="flex gap-3 text-sm">
      <dt className="w-20 shrink-0 text-white/40">{label}</dt>
      <dd
        className={`min-w-0 break-all ${mono ? 'font-mono' : ''} ${
          danger ? 'text-red-400' : 'text-white/70'
        }`}
      >
        {value}
      </dd>
    </div>
  );
}
