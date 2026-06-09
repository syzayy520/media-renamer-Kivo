// features/settings/state/settingsPageStore.ts — 设置页面状态
// 职责：管理配置加载、编辑、保存状态

import { create } from 'zustand';
import type { RenameRule } from '../../../api/config/types';

interface SettingsPageState {
  templates: RenameRule[];
  confidenceThreshold: number;
  isLoading: boolean;
  error: string | null;
  saveStatus: 'idle' | 'saving' | 'saved' | 'error';
  saveMessage: string;

  setTemplates: (templates: RenameRule[]) => void;
  setConfidenceThreshold: (value: number) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setSaveStatus: (
    status: 'idle' | 'saving' | 'saved' | 'error',
    message?: string,
  ) => void;
  clearError: () => void;
}

export const useSettingsPageStore = create<SettingsPageState>((set) => ({
  templates: [],
  confidenceThreshold: 70,
  isLoading: false,
  error: null,
  saveStatus: 'idle',
  saveMessage: '',

  setTemplates: (templates) => set({ templates, isLoading: false }),
  setConfidenceThreshold: (confidenceThreshold) => set({ confidenceThreshold, isLoading: false }),
  setIsLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error, isLoading: false }),
  setSaveStatus: (saveStatus, saveMessage = '') =>
    set({ saveStatus, saveMessage }),
  clearError: () => set({ error: null, saveStatus: 'idle' }),
}));
