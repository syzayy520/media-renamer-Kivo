// features/scan/components/ScanResultSummary.tsx — 扫描结果摘要
// 职责：展示 PipelineResult 的扫描统计数据

import type { PipelineResult } from '../../../api/session/types';
import { Badge } from '../../../shared/ui/Badge';

interface ScanResultSummaryProps {
  result: PipelineResult;
}

export function ScanResultSummary({ result }: ScanResultSummaryProps) {
  const { scan, parsed_count, unknown_count, previews, task_id } = result;
  const safetyStatus = result.safety.can_execute ? 'success' : 'warning';

  return (
    <div className="space-y-4">
      {/* Task ID */}
      <div className="mb-4 text-[11px] text-white/40">
        Task ID: {task_id}
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        <StatCard label="视频文件" value={scan.video_count} />
        <StatCard label="已解析" value={parsed_count} variant="success" />
        <StatCard label="未识别" value={unknown_count} variant="warning" />
        <StatCard label="预览项" value={previews.length} variant="info" />
      </div>

      {/* Companion + Duration */}
      <div className="grid grid-cols-2 gap-3 md:grid-cols-2">
        <StatCard label="伴随文件" value={scan.companion_count} />
        <StatCard label="扫描耗时" value={`${scan.scan_duration_ms}ms`} />
      </div>

      {/* Safety Status */}
      <div className="flex items-center gap-2">
        <Badge variant={safetyStatus}>
          {result.safety.can_execute ? 'Safe for dry-run preview' : 'Blocked / Needs Review'}
        </Badge>
        <span className="text-xs text-white/40">
          （当前 UI 不会真实修改文件）
        </span>
      </div>
    </div>
  );
}

function StatCard({
  label,
  value,
  variant = 'info',
}: {
  label: string;
  value: string | number;
  variant?: 'success' | 'warning' | 'info' | 'danger';
}) {
  const colorClasses: Record<string, string> = {
    success: 'text-emerald-400',
    warning: 'text-amber-400',
    danger: 'text-red-400',
    info: 'text-blue-400',
  };

  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-3">
      <div className={`text-xl font-bold ${colorClasses[variant]}`}>
        {value}
      </div>
      <div className="mt-1 text-xs text-white/50">{label}</div>
    </div>
  );
}
