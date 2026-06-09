// features/preview-workbench/candidate-list/CandidatePanel.tsx — 中栏候选面板
// 职责：展示候选匹配列表（当前为静态骨架，后续票接入真实数据）
// 集成选择状态：显示是否有文件被选中

import { useWorkbenchSelectionStore } from '../state/workbenchSelectionStore';
import { CandidateEmptyState } from './CandidateEmptyState';
import { CandidatePlaceholderCard } from './CandidatePlaceholderCard';

export function CandidatePanel() {
  // 静态骨架：显示 3 个占位卡片，模拟未来候选列表
  const placeholderCount = 3;
  const selectedPreviewId = useWorkbenchSelectionStore((s) => s.selectedPreviewId);

  return (
    <div className="flex h-full flex-col">
      {/* 标题 */}
      <div className="mb-3 flex items-center justify-between">
        <span className="text-sm font-medium text-white/60">匹配候选</span>
        <span className="rounded-full bg-white/5 px-2 py-0.5 text-xs text-white/40">
          0/{placeholderCount}
        </span>
      </div>

      {/* 选择状态提示 */}
      <div className="mb-3 rounded-md bg-white/5 px-3 py-2 text-xs text-white/30">
        {selectedPreviewId ? (
          <span className="text-blue-400">已选择文件，候选匹配将在后续票中启用</span>
        ) : (
          <span>请先从左栏选择文件</span>
        )}
      </div>

      {/* 搜索栏占位（后续票启用） */}
      <div className="mb-3 rounded-md bg-white/5 px-3 py-2 text-xs text-white/30">
        搜索候选...（后续票启用）
      </div>

      {/* 候选列表占位 */}
      <div className="flex-1 overflow-y-auto">
        {Array.from({ length: placeholderCount }).map((_, index) => (
          <CandidatePlaceholderCard key={index} index={index} />
        ))}
      </div>

      {/* 空状态（当前显示，因为没有真实数据） */}
      <CandidateEmptyState />
    </div>
  );
}
