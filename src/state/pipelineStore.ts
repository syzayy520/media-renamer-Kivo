import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type {
  ApplyTmdbCandidateInput,
  ApplyTmdbCandidateOutput,
  FolderPolicy,
  NamingRule,
  PipelineResult,
  SafetyReport,
  SafetySummaryInput,
  TitleStrategy,
  TmdbCandidate,
} from '../types';
import { useUiFeedbackStore } from './uiFeedbackStore';

interface PipelineState {
  pipelineResult: PipelineResult | null;
  startRenameSession: (directory: string) => Promise<PipelineResult>;
  applyTmdbCandidate: (itemId: string, candidate: TmdbCandidate) => Promise<ApplyTmdbCandidateOutput>;
  applyNamingRule: (namingRule: NamingRule, titleStrategy: TitleStrategy) => Promise<void>;
  applyFolderPolicy: (folderPolicy: FolderPolicy) => Promise<void>;
  refreshSafetySummary: () => Promise<SafetyReport | null>;
  updatePreviewProposedName: (itemId: string, proposedName: string) => void;
  togglePreviewSkipped: (itemId: string) => void;
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

  applyNamingRule: async (namingRule: NamingRule, titleStrategy: TitleStrategy) => {
    const { pipelineResult } = get();
    if (!pipelineResult || pipelineResult.previews.length === 0) return;

    try {
      const { previews } = await invoke<{ previews: typeof pipelineResult.previews }>(
        'apply_naming_rule',
        {
          input: {
            previews: pipelineResult.previews,
            naming_rule: namingRule,
            title_strategy: titleStrategy,
          },
        },
      );
      set({ pipelineResult: { ...pipelineResult, previews } });
    } catch (err) {
      console.error('Failed to apply naming rule:', err);
    }
  },

  applyFolderPolicy: async (folderPolicy: FolderPolicy) => {
    const { pipelineResult } = get();
    if (!pipelineResult || pipelineResult.previews.length === 0) return;

    try {
      const { previews } = await invoke<{ previews: typeof pipelineResult.previews }>(
        'apply_folder_policy',
        {
          input: {
            previews: pipelineResult.previews,
            folder_policy: folderPolicy,
          },
        },
      );
      set({ pipelineResult: { ...pipelineResult, previews } });
    } catch (err) {
      console.error('Failed to apply folder policy:', err);
    }
  },

  applyTmdbCandidate: async (itemId: string, candidate: TmdbCandidate) => {
    const { pipelineResult } = get();
    if (!pipelineResult) {
      return { result: null, error: 'No pipeline result available', safety: null };
    }

    const item = pipelineResult.previews.find((p) => p.id === itemId);
    if (!item) {
      return { result: null, error: `Preview item not found: ${itemId}`, safety: null };
    }

    try {
      const input: ApplyTmdbCandidateInput = { item, candidate };
      const output = await invoke<ApplyTmdbCandidateOutput>('apply_tmdb_candidate', { input });

      if (output.result) {
        const updatedPreviews = pipelineResult.previews.map((p) => {
          if (p.id === itemId) {
            return output.result!.updated_item;
          }
          return p;
        });

        set({
          pipelineResult: {
            ...pipelineResult,
            previews: updatedPreviews,
          },
        });
      }

      return output;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      return { result: null, error: errorMessage, safety: null };
    }
  },

  refreshSafetySummary: async () => {
    const { pipelineResult } = get();
    if (!pipelineResult) {
      return null;
    }

    try {
      const input: SafetySummaryInput = { previews: pipelineResult.previews };
      const safety = await invoke<SafetyReport>('get_safety_summary', { input });

      set({
        pipelineResult: {
          ...pipelineResult,
          safety,
        },
      });

      return safety;
    } catch (err) {
      console.error('Failed to refresh safety summary:', err);
      return null;
    }
  },

  updatePreviewProposedName: (itemId: string, proposedName: string) => {
    const { pipelineResult } = get();
    if (!pipelineResult) {
      return;
    }

    const updatedPreviews = pipelineResult.previews.map((item) => {
      if (item.id !== itemId) {
        return item;
      }

      const lastSep = item.target_path.lastIndexOf('\\');
      const parentDir = lastSep >= 0 ? item.target_path.substring(0, lastSep + 1) : '';
      const targetPath = parentDir + proposedName;

      return {
        ...item,
        proposed_name: proposedName,
        target_path: targetPath,
        metadata_source: 'Manual' as const,
        needs_manual_review: false,
      };
    });

    set({
      pipelineResult: {
        ...pipelineResult,
        previews: updatedPreviews,
      },
    });
  },

  togglePreviewSkipped: (itemId: string) => {
    const { pipelineResult } = get();
    if (!pipelineResult) {
      return;
    }

    const updatedPreviews = pipelineResult.previews.map((item) => {
      if (item.id !== itemId) {
        return item;
      }

      return {
        ...item,
        should_skip: !item.should_skip,
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
