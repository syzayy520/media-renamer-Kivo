// features/safety/components/SafetyOverview.tsx — 安全检查概览
// 职责：展示 SafetyReport 统计卡片

import type { PipelineResult } from '../../../api/session/types';
import { Badge } from '../../../shared/ui/Badge';

interface SafetyOverviewProps {
  result: PipelineResult;
}

export function SafetyOverview({ result }: SafetyOverviewProps) {
  const { safety, previews } = result;
  const conflictCount = previews.filter((p) => p.conflicts.length > 0).length;
  const needsReviewCount = previews.filter((p) => p.needs_manual_review && p.conflicts.length === 0).length;

  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-5">
      <div className="mb-2 text-[11px] text-white/40">
        Task ID: {result.task_id}
      </div>

      <div className="mb-4 flex items-center gap-2">
        {safety.can_execute ? (
          <Badge variant="success">Safe for dry-run preview</Badge>
        ) : (
          <Badge variant="danger">Blocked</Badge>
        )}
        <span className="text-xs text-white/30">
          （当前 UI 不会真实修改文件）
        </span>
      </div>

      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        <StatCard label="预览项" value={previews.length} color="text-blue-400" />
        <StatCard label="阻塞原因" value={safety.blocking_reasons.length} color="text-red-400" />
        <StatCard label="有冲突" value={conflictCount} color="text-red-400" />
        <StatCard label="需审核" value={needsReviewCount} color="text-amber-400" />
      </div>
    </div>
  );
}

function StatCard({
  label,
  value,
  color,
}: {
  label: string;
  value: number;
  color: string;
}) {
  return (
    <div>
      <div className={`text-lg font-bold ${color}`}>{value}</div>
      <div className="text-xs text-white/50">{label}</div>
    </div>
  );
}
