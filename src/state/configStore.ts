import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppConfig, RenameRule } from '../types';
import { useUiFeedbackStore } from './uiFeedbackStore';

interface ConfigState {
  config: AppConfig | null;
  templates: RenameRule[];
  confidenceThreshold: number;
  fetchAppConfig: () => Promise<void>;
  fetchAllTemplates: () => Promise<void>;
  setTemplate: (mediaType: string, template: string) => Promise<void>;
  fetchConfidenceThreshold: () => Promise<void>;
  setConfidenceThreshold: (value: number) => Promise<void>;
}

export const useConfigStore = create<ConfigState>((set, get) => ({
  config: null,
  templates: [],
  confidenceThreshold: 70,

  fetchAppConfig: async () => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      const config = await invoke<AppConfig>('get_app_config');
      set({ config });
      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },

  fetchAllTemplates: async () => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      const templates = await invoke<RenameRule[]>('get_all_templates');
      set({ templates });
      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },

  setTemplate: async (mediaType: string, template: string) => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      await invoke('set_template', { mediaType, template });
      await get().fetchAllTemplates();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },

  fetchConfidenceThreshold: async () => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      const threshold = await invoke<number>('get_confidence_threshold');
      set({ confidenceThreshold: threshold });
      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },

  setConfidenceThreshold: async (value: number) => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      await invoke('set_confidence_threshold', { value });
      set({ confidenceThreshold: value });
      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },
}));
