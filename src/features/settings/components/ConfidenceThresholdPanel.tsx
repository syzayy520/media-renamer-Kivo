// features/settings/components/ConfidenceThresholdPanel.tsx — 置信度阈值面板
// 职责：展示和编辑置信度阈值

import { useState } from 'react';

interface ConfidenceThresholdPanelProps {
  value: number;
  onSave: (value: number) => Promise<void>;
}

export function ConfidenceThresholdPanel({
  value,
  onSave,
}: ConfidenceThresholdPanelProps) {
  const [threshold, setThreshold] = useState(value);
  const isDirty = threshold !== value;

  const handleSave = async () => {
    await onSave(threshold);
  };

  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-4">
      <div className="mb-1 flex items-center justify-between">
        <div>
          <h3 className="text-sm font-medium text-white/70">置信度阈值</h3>
          <p className="mt-0.5 text-xs text-white/40">
            低于该阈值的预览项需要人工检查
          </p>
        </div>
        <button
          type="button"
          disabled={!isDirty}
          onClick={handleSave}
          className={`shrink-0 rounded px-4 py-1.5 text-xs font-medium transition-colors ${
            isDirty
              ? 'bg-blue-600 text-white hover:bg-blue-500'
              : 'cursor-not-allowed bg-white/5 text-white/20'
          }`}
        >
          保存
        </button>
      </div>
      <div className="mt-3 flex items-center gap-4">
        <input
          type="range"
          aria-label="置信度阈值"
          min={0}
          max={100}
          value={threshold}
          onChange={(e) => setThreshold(Number(e.target.value))}
          className="flex-1 accent-blue-600"
        />
        <span className="w-12 text-center text-lg font-bold text-blue-400">
          {threshold}
        </span>
        <span className="text-xs text-white/30">%</span>
      </div>
    </div>
  );
}
