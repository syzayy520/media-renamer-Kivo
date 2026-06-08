// features/safety/components/BlockingReasonsList.tsx — 阻塞原因列表
// 职责：展示 SafetyReport.blocking_reasons

import type { SafetyReport } from '../../../api/session/types';
import { Badge } from '../../../shared/ui/Badge';

interface BlockingReasonsListProps {
  safety: SafetyReport;
}

export function BlockingReasonsList({ safety }: BlockingReasonsListProps) {
  if (safety.blocking_reasons.length === 0) {
    return (
      <div className="rounded-lg border border-emerald-500/20 bg-emerald-500/5 p-4">
        <Badge variant="success">无阻塞原因</Badge>
        <p className="mt-2 text-sm text-white/40">当前没有阻塞原因，预览安全。</p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border border-red-500/20 bg-red-500/5 p-4">
      <div className="mb-3 flex items-center gap-2">
        <Badge variant="danger">阻塞原因 ({safety.blocking_reasons.length})</Badge>
      </div>
      <ul className="space-y-1.5">
        {safety.blocking_reasons.map((reason, i) => (
          <li key={i} className="pl-3 text-sm text-red-300 border-l-2 border-red-500/30">
            {reason}
          </li>
        ))}
      </ul>
    </div>
  );
}
