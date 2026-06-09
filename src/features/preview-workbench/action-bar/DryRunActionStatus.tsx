// features/preview-workbench/action-bar/DryRunActionStatus.tsx — 状态说明区
// 职责：显示 dry-run 模式标识、安全边界说明、选择状态

import { useWorkbenchSelectionStore } from '../state/workbenchSelectionStore';

export function DryRunActionStatus() {
  const selectedPreviewId = useWorkbenchSelectionStore((s) => s.selectedPreviewId);
  const selectedCandidateId = useWorkbenchSelectionStore((s) => s.selectedCandidateId);
  const appliedCandidateId = useWorkbenchSelectionStore((s) => s.appliedCandidateId);

  return (
    <div className="flex items-center gap-3">
      <span className="rounded-full bg-blue-500/20 px-2.5 py-0.5 text-xs font-medium text-blue-400">
        Dry-run Only
      </span>
      <span className="text-xs text-white/40">
        No real file changes — actions are staged for future tickets
      </span>
      <div className="flex gap-2 text-xs">
        <span className={selectedPreviewId ? 'text-green-400' : 'text-white/20'}>
          Preview: {selectedPreviewId ? '✓' : '—'}
        </span>
        <span className={selectedCandidateId ? 'text-green-400' : 'text-white/20'}>
          Candidate: {selectedCandidateId ? '✓' : '—'}
        </span>
        <span className={appliedCandidateId ? 'text-green-400' : 'text-white/20'}>
          Applied: {appliedCandidateId ? '✓' : '—'}
        </span>
      </div>
    </div>
  );
}
