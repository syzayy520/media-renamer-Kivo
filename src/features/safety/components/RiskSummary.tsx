// features/safety/components/RiskSummary.tsx — 风险摘要
// 职责：展示 dry-run 安全声明和风险提示

import { Badge } from '../../../shared/ui/Badge';

interface RiskSummaryProps {
  canExecute: boolean;
}

export function RiskSummary({ canExecute }: RiskSummaryProps) {
  return (
    <div
      className={`rounded-lg border p-4 ${
        canExecute
          ? 'border-emerald-500/20 bg-emerald-500/5'
          : 'border-amber-500/20 bg-amber-500/5'
      }`}
    >
      <div className="mb-2 flex items-center gap-2">
        <Badge variant={canExecute ? 'success' : 'warning'}>
          {canExecute ? '安全检查通过' : '存在阻塞项'}
        </Badge>
      </div>
      <p className="text-sm text-white/50">
        {canExecute
          ? '当前安全检查已通过。但仍需注意：本 UI 仅支持 Dry-run Preview，不会真实修改媒体文件。真实重命名功能尚未开放。'
          : '当前预览存在阻塞项，仅供检查，不会执行真实重命名。请在扫描页面查看详情并手动处理阻塞项。'}
      </p>
    </div>
  );
}
