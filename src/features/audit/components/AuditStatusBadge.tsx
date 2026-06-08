// features/audit/components/AuditStatusBadge.tsx — 任务状态标签
// 职责：根据 TaskStatus 显示对应 badge

import type { TaskStatus } from '../../../api/audit/types';
import { Badge } from '../../../shared/ui/Badge';

interface AuditStatusBadgeProps {
  status: TaskStatus;
}

const statusConfig: Record<
  TaskStatus,
  { variant: 'success' | 'warning' | 'danger' | 'info'; label: string }
> = {
  Previewing: { variant: 'info', label: '预览中' },
  Pending: { variant: 'warning', label: '待执行' },
  Executing: { variant: 'info', label: '执行中' },
  Completed: { variant: 'success', label: '已完成' },
  Failed: { variant: 'danger', label: '失败' },
  RolledBack: { variant: 'warning', label: '已回滚' },
};

export function AuditStatusBadge({ status }: AuditStatusBadgeProps) {
  const cfg = statusConfig[status] ?? { variant: 'info' as const, label: status };
  return <Badge variant={cfg.variant}>{cfg.label}</Badge>;
}
