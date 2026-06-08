// features/preview/state/previewFilterStore.ts — 预览筛选状态
// 职责：管理搜索文本和筛选条件

import { create } from 'zustand';

interface PreviewFilterState {
  searchText: string;
  setSearchText: (text: string) => void;
  activeFilter: 'all' | 'conflict' | 'needs_review' | 'safe';
  setActiveFilter: (filter: 'all' | 'conflict' | 'needs_review' | 'safe') => void;
}

export const usePreviewFilterStore = create<PreviewFilterState>((set) => ({
  searchText: '',
  setSearchText: (text) => set({ searchText: text }),
  activeFilter: 'all',
  setActiveFilter: (filter) => set({ activeFilter: filter }),
}));
