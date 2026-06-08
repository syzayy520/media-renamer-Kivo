// features/safety/components/SafetyCheckList.tsx — 检查项列表
// 职责：展示 SafetyCheck 项列表

import type { PipelineResult } from '../../../api/session/types';
import { Badge } from '../../../shared/ui/Badge';

interface SafetyCheckListProps {
  result: PipelineResult;
}

export function SafetyCheckList({ result }: SafetyCheckListProps) {
  const { safety, previews } = result;
  const hasConflict = previews.some((p) => p.conflicts.length > 0);
  const hasLowConfidence = previews.some((p) => p.confidence < 70);
  const hasManualReview = previews.some((p) => p.needs_manual_review);

  const checks = [
    {
      name: '冲突检测',
      passed: !hasConflict,
      detail: hasConflict ? '存在文件名冲突' : '无冲突',
    },
    {
      name: '低置信度',
      passed: !hasLowConfidence,
      detail: hasLowConfidence ? '存在低置信度项' : '置信度正常',
    },
    {
      name: '人工审核',
      passed: !hasManualReview,
      detail: hasManualReview ? '存在需要人工审核的项' : '无需人工审核',
    },
    {
      name: '真实执行',
      passed: false,
      detail: '当前 UI 不开放真实执行',
    },
    {
      name: 'Dry-run',
      passed: safety.dry_run,
      detail: '当前为 Dry-run 模式，不会修改媒体文件',
    },
  ];

  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-4">
      <h3 className="mb-3 text-sm font-medium text-white/70">检查项</h3>
      <div className="space-y-2">
        {checks.map((check, i) => (
          <div
            key={i}
            className="flex items-center gap-3 rounded border border-white/5 px-3 py-2"
          >
            <Badge variant={check.passed ? 'success' : 'warning'}>
              {check.passed ? '通过' : '警告'}
            </Badge>
            <span className="text-sm text-white/80">{check.name}</span>
            <span className="text-xs text-white/40">{check.detail}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
