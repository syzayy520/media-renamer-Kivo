// features/preview-workbench/types/selection.ts — 工作台选择类型
// 职责：定义工作台选择相关的类型，确保类型安全

/**
 * 工作台选择状态接口
 * 只保存 ID，不保存完整数据
 */
export interface WorkbenchSelectionState {
  selectedPreviewId: string | null;
  selectedCandidateId: string | null;
  appliedCandidateId: string | null;
}

/**
 * 工作台选择操作接口
 */
export interface WorkbenchSelectionActions {
  selectPreview: (id: string | null) => void;
  selectCandidate: (id: string | null) => void;
  applyCandidate: (id: string | null) => void;
  clearSelection: () => void;
}

/**
 * 工作台完整状态类型
 */
export type WorkbenchSelectionStore = WorkbenchSelectionState & WorkbenchSelectionActions;

/**
 * 选择摘要接口（用于状态显示）
 */
export interface SelectionSummary {
  hasPreview: boolean;
  hasCandidate: boolean;
  hasApplied: boolean;
  previewId: string | null;
  candidateId: string | null;
  appliedId: string | null;
}
