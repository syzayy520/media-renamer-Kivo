// features/preview-workbench/state/workbenchSelectionStore.ts — 工作台选择状态管理
// 职责：管理 preview/candidate/applied 的 ID 选择状态，不存储完整数据

import { create } from 'zustand';

interface WorkbenchSelectionState {
  // 选择状态（只保存 ID，不保存完整数据）
  selectedPreviewId: string | null;
  selectedCandidateId: string | null;
  appliedCandidateId: string | null;

  // 操作
  selectPreview: (id: string | null) => void;
  selectCandidate: (id: string | null) => void;
  applyCandidate: (id: string | null) => void;
  clearSelection: () => void;
}

export const useWorkbenchSelectionStore = create<WorkbenchSelectionState>((set) => ({
  // 初始状态
  selectedPreviewId: null,
  selectedCandidateId: null,
  appliedCandidateId: null,

  // 操作实现
  selectPreview: (id) => set({ selectedPreviewId: id }),
  selectCandidate: (id) => set({ selectedCandidateId: id }),
  applyCandidate: (id) => set({ appliedCandidateId: id }),
  clearSelection: () =>
    set({
      selectedPreviewId: null,
      selectedCandidateId: null,
      appliedCandidateId: null,
    }),
}));
