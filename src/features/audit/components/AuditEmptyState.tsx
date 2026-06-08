// features/audit/components/AuditEmptyState.tsx — 空状态
// 职责：没有审计任务时的提示

import { Link } from 'react-router-dom';

interface AuditEmptyStateProps {
  reason?: string;
}

export function AuditEmptyState({
  reason = '暂无审计任务',
}: AuditEmptyStateProps) {
  return (
    <div className="rounded-lg border border-white/5 bg-white/[0.02] p-12 text-center">
      <p className="mb-2 text-lg font-medium text-white/50">{reason}</p>
      <p className="mb-6 text-sm text-white/30">
        请先前往扫描页面生成 Dry-run Preview。
      </p>
      <Link
        to="/"
        className="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-500"
      >
        前往扫描
      </Link>
    </div>
  );
}
