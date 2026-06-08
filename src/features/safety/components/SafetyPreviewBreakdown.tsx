// features/safety/components/SafetyPreviewBreakdown.tsx — 预览统计
// 职责：从 previews 统计 conflict / needs review / safe / low confidence

import type { PipelineResult } from '../../../api/session/types';
import { Badge } from '../../../shared/ui/Badge';

interface SafetyPreviewBreakdownProps {
  result: PipelineResult;
}

export function SafetyPreviewBreakdown({ result }: SafetyPreviewBreakdownProps) {
  const { previews } = result;
  const conflictItems = previews.filter((p) => p.conflicts.length > 0);
  const needsReviewItems = previews.filter(
    (p) => p.needs_manual_review && p.conflicts.length === 0,
  );
  const lowConfidenceItems = previews.filter((p) => p.confidence < 70);
  const safeItems = previews.filter(
    (p) => p.conflicts.length === 0 && !p.needs_manual_review,
  );

  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-4">
      <h3 className="mb-3 text-sm font-medium text-white/70">预览项分类</h3>

      {previews.length === 0 ? (
        <p className="text-sm text-white/30">暂无预览数据</p>
      ) : (
        <div className="space-y-2">
          <Row label="有冲突" count={conflictItems.length} variant="danger">
            {conflictItems.length > 0 && (
              <ul className="mt-1 list-inside list-disc text-xs text-red-400/60">
                {conflictItems.slice(0, 3).map((p, i) => (
                  <li key={i} className="truncate font-mono">{p.original_name}</li>
                ))}
                {conflictItems.length > 3 && (
                  <li className="text-white/30">
                    ...还有 {conflictItems.length - 3} 项
                  </li>
                )}
              </ul>
            )}
          </Row>
          <Row label="需审核" count={needsReviewItems.length} variant="warning" />
          <Row label="低置信度 (&lt;70%)" count={lowConfidenceItems.length} variant="warning" />
          <Row label="安全" count={safeItems.length} variant="success" />
        </div>
      )}
    </div>
  );
}

function Row({
  label,
  count,
  variant,
  children,
}: {
  label: string;
  count: number;
  variant: 'success' | 'warning' | 'danger' | 'info';
  children?: React.ReactNode;
}) {
  return (
    <div className="flex items-start gap-3 rounded border border-white/5 px-3 py-2">
      <Badge variant={variant}>{count}</Badge>
      <div className="min-w-0 flex-1">
        <span className="text-sm text-white/70">{label}</span>
        {children}
      </div>
    </div>
  );
}
