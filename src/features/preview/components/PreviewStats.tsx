// features/preview/components/PreviewStats.tsx — 统计摘要
// 职责：展示预览统计数据

import type { PipelineResult } from '../../../api/session/types';

interface PreviewStatsProps {
  result: PipelineResult;
}

export function PreviewStats({ result }: PreviewStatsProps) {
  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-4">
      <div className="mb-2 text-[11px] text-white/40">
        Task ID: {result.task_id}
      </div>
      <div className="grid grid-cols-4 gap-3">
        <Stat label="预览项" value={result.previews.length} color="text-blue-400" />
        <Stat label="已解析" value={result.parsed_count} color="text-emerald-400" />
        <Stat label="未识别" value={result.unknown_count} color="text-amber-400" />
        <Stat label="阻塞原因" value={result.safety.blocking_reasons.length} color="text-red-400" />
      </div>
    </div>
  );
}

function Stat({
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
