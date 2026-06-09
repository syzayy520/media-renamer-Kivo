// features/settings/state/settingsPageStore.ts — 设置页面状态
// 职责：管理配置加载、编辑、保存状态

import { create } from 'zustand';
import type { AppConfig, RenameRule } from '../../../api/config/types';

interface SettingsPageState {
  config: AppConfig | null;
  templates: RenameRule[];
  confidenceThreshold: number;
  isLoading: boolean;
  error: string | null;
  saveStatus: 'idle' | 'saving' | 'saved' | 'error';
  saveMessage: string;

  setConfig: (config: AppConfig) => void;
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
  config: null,
  templates: [],
  confidenceThreshold: 70,
  isLoading: false,
  error: null,
  saveStatus: 'idle',
  saveMessage: '',

  setConfig: (config) => set({ config }),
  setTemplates: (templates) => set({ templates }),
  setConfidenceThreshold: (confidenceThreshold) => set({ confidenceThreshold }),
  setIsLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error, isLoading: false }),
  setSaveStatus: (saveStatus, saveMessage = '') =>
    set({ saveStatus, saveMessage }),
  clearError: () => set({ error: null, saveStatus: 'idle' }),
}));
