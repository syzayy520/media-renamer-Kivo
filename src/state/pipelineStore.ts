import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { PipelineResult } from '../types';
import { useUiFeedbackStore } from './uiFeedbackStore';

interface PipelineState {
  pipelineResult: PipelineResult | null;
  startRenameSession: (directory: string) => Promise<PipelineResult>;
}

export const usePipelineStore = create<PipelineState>((set) => ({
  pipelineResult: null,

  startRenameSession: async (directory: string) => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<PipelineResult>('start_rename_session', { directory });
      set({ pipelineResult: result });
      setLoading(false);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
      throw err;
    }
  },
}));
