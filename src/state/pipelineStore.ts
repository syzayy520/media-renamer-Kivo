import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { PipelineResult, TmdbCandidate } from '../types';
import { useUiFeedbackStore } from './uiFeedbackStore';

interface PipelineState {
  pipelineResult: PipelineResult | null;
  startRenameSession: (directory: string) => Promise<PipelineResult>;
  applyTmdbCandidate: (itemId: string, candidate: TmdbCandidate) => void;
}

export const usePipelineStore = create<PipelineState>((set, get) => ({
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

  applyTmdbCandidate: (itemId: string, candidate: TmdbCandidate) => {
    const { pipelineResult } = get();
    if (!pipelineResult) return;

    const updatedPreviews = pipelineResult.previews.map((item) => {
      if (item.id !== itemId) return item;

      // Update parsed_info with TMDb candidate information
      const updatedParsedInfo = {
        ...item.parsed_info,
        title: candidate.title,
        year: candidate.year,
        media_type: candidate.media_type,
      };

      // Generate new proposed name based on updated info
      // For now, use a simple format: "Title (Year)"
      const yearStr = candidate.year ? ` (${candidate.year})` : '';
      const newProposedName = `${candidate.title}${yearStr}${item.parsed_info.media_item.extension}`;

      return {
        ...item,
        parsed_info: updatedParsedInfo,
        proposed_name: newProposedName,
        original_name: item.original_name,
      };
    });

    set({
      pipelineResult: {
        ...pipelineResult,
        previews: updatedPreviews,
      },
    });
  },
}));
