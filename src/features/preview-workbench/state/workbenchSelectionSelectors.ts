// features/preview-workbench/state/workbenchSelectionSelectors.ts — 工作台选择选择器
// 职责：提供便捷的状态选择器，避免在组件中重复编写选择逻辑

import { useWorkbenchSelectionStore } from './workbenchSelectionStore';

// 检查是否有任何选择
export const useHasAnySelection = () =>
  useWorkbenchSelectionStore(
    (s) =>
      s.selectedPreviewId !== null ||
      s.selectedCandidateId !== null ||
      s.appliedCandidateId !== null,
  );

// 检查是否有预览选择
export const useHasPreviewSelection = () =>
  useWorkbenchSelectionStore((s) => s.selectedPreviewId !== null);

// 检查是否有候选选择
export const useHasCandidateSelection = () =>
  useWorkbenchSelectionStore((s) => s.selectedCandidateId !== null);

// 检查是否有已应用候选
export const useHasAppliedCandidate = () =>
  useWorkbenchSelectionStore((s) => s.appliedCandidateId !== null);

// 获取选择状态摘要（用于状态显示）
export const useSelectionSummary = () =>
  useWorkbenchSelectionStore((s) => ({
    hasPreview: s.selectedPreviewId !== null,
    hasCandidate: s.selectedCandidateId !== null,
    hasApplied: s.appliedCandidateId !== null,
    previewId: s.selectedPreviewId,
    candidateId: s.selectedCandidateId,
    appliedId: s.appliedCandidateId,
  }));
