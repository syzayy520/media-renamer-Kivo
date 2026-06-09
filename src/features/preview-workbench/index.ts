// features/preview-workbench/index.ts — 模块入口
// 职责：re-export page / file-list / candidate-list / rename-review / candidate-search 组件，不做逻辑编排

export { PreviewWorkbenchPage } from './page/PreviewWorkbenchPage';
export { FileQueuePanel } from './file-list/FileQueuePanel';
export { FileQueueItem } from './file-list/FileQueueItem';
export { FileQueueEmptyState } from './file-list/FileQueueEmptyState';
export { CandidatePanel } from './candidate-list/CandidatePanel';
export { CandidateEmptyState } from './candidate-list/CandidateEmptyState';
export { CandidatePlaceholderCard } from './candidate-list/CandidatePlaceholderCard';
export { RenamePreviewPanel } from './rename-preview/RenamePreviewPanel';
export { RenamePreviewEmptyState } from './rename-preview/RenamePreviewEmptyState';
export { RenamePathPreview } from './rename-preview/RenamePathPreview';
export { RenameSafetyNotice } from './rename-preview/RenameSafetyNotice';
export { DryRunActionBar } from './action-bar/DryRunActionBar';
export { DryRunActionButton } from './action-bar/DryRunActionButton';
export { DryRunActionStatus } from './action-bar/DryRunActionStatus';

// 安全摘要
export { SafetySummaryPanel } from './safety-summary/SafetySummaryPanel';
export { SafetySummaryEmptyState } from './safety-summary/SafetySummaryEmptyState';
export { SafetySummaryItem } from './safety-summary/SafetySummaryItem';

// TMDb Search Mock UI (P2-010)
export { TmdbSearchPanel } from './candidate-search/TmdbSearchPanel';
export { TmdbSearchInput } from './candidate-search/TmdbSearchInput';
export { TmdbSearchResultList } from './candidate-search/TmdbSearchResultList';
export { TmdbSearchResultItem } from './candidate-search/TmdbSearchResultItem';
export { TmdbSearchEmptyState } from './candidate-search/TmdbSearchEmptyState';
export { TmdbSearchErrorState } from './candidate-search/TmdbSearchErrorState';
export { TmdbSearchLoadingState } from './candidate-search/TmdbSearchLoadingState';

// 状态管理
export { useWorkbenchSelectionStore } from './state/workbenchSelectionStore';
export {
  useHasAnySelection,
  useHasPreviewSelection,
  useHasCandidateSelection,
  useHasAppliedCandidate,
  useSelectionSummary,
} from './state/workbenchSelectionSelectors';

// 类型定义
export type {
  WorkbenchSelectionState,
  WorkbenchSelectionActions,
  WorkbenchSelectionStore,
  SelectionSummary,
} from './types/selection';

export type {
  SafetySummaryData,
  SafetySummaryPanelProps,
  SafetySummaryItemProps,
} from './types/safetySummary';

export type {
  MockTmdbCandidate,
  CandidateSearchMockState,
  CandidateSearchMockMode,
  CandidateSearchMockError,
} from './types/candidateSearch';
