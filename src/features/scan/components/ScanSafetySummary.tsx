// features/scan/components/ScanSafetySummary.tsx — 安全检查摘要
// 职责：展示 SafetyReport 的阻塞原因列表

import type { SafetyReport } from '../../../api/session/types';
import { Badge } from '../../../shared/ui/Badge';

interface ScanSafetySummaryProps {
  safety: SafetyReport;
}

export function ScanSafetySummary({ safety }: ScanSafetySummaryProps) {
  if (safety.can_execute && safety.blocking_reasons.length === 0) {
    return (
      <div className="rounded-lg border border-emerald-500/20 bg-emerald-500/5 p-4">
        <Badge variant="success">Safe for dry-run preview</Badge>
        <p className="mt-2 text-sm text-white/40">
          所有安全检查已通过。
          （当前 UI 不会真实修改文件）
        </p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border border-amber-500/20 bg-amber-500/5 p-4">
      <div className="mb-3 flex items-center gap-2">
        <Badge variant="warning">Blocked / Needs Review</Badge>
        <span className="text-xs text-white/40">
          （当前 UI 不会真实修改文件）
        </span>
      </div>
      <ul className="space-y-1">
        {safety.blocking_reasons.map((reason, i) => (
          <li key={i} className="text-sm text-amber-300">
            {reason}
          </li>
        ))}
      </ul>
    </div>
  );
}
