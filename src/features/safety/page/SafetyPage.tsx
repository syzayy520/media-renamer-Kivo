// features/safety/page/SafetyPage.tsx — 安全检查页面
// 职责：展示 SafetyReport、阻塞原因、预览统计

import { useScanStore } from '../../scan/state/scanStore';
import { SafetyEmptyState } from '../components/SafetyEmptyState';
import { SafetyOverview } from '../components/SafetyOverview';
import { BlockingReasonsList } from '../components/BlockingReasonsList';
import { SafetyCheckList } from '../components/SafetyCheckList';
import { RiskSummary } from '../components/RiskSummary';
import { SafetyPreviewBreakdown } from '../components/SafetyPreviewBreakdown';

export function SafetyPage() {
  const result = useScanStore((s) => s.result);

  if (!result) {
    return <SafetyEmptyState />;
  }

  return (
    <div className="space-y-4">
      {/* 标题 + Dry-run 安全说明 */}
      <div className="rounded-lg border border-white/10 bg-white/5 p-4">
        <h2 className="mb-1 text-lg font-semibold">安全检查</h2>
        <p className="text-sm text-white/50">
          当前仅展示安全检查结果，不会真实修改媒体文件。
        </p>
      </div>

      {/* 风险摘要 */}
      <RiskSummary canExecute={result.safety.can_execute} />

      {/* 概览统计 */}
      <SafetyOverview result={result} />

      {/* 阻塞原因 */}
      <BlockingReasonsList safety={result.safety} />

      {/* 检查项列表 */}
      <SafetyCheckList result={result} />

      {/* 预览项分类 */}
      <SafetyPreviewBreakdown result={result} />
    </div>
  );
}
