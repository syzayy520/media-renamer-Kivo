import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type {
  SearchTmdbCandidatesInput,
  SearchTmdbCandidatesOutput,
  TmdbSearchStatus,
} from '../types';

interface TmdbSearchState {
  /** 功能状态：disabled = 后端未启用, idle = 可用未搜索 */
  tmdbSearchStatus: TmdbSearchStatus;
  /** 最近一次搜索结果 */
  lastResult: SearchTmdbCandidatesOutput | null;
  /** 禁用原因（仅 status=disabled 时有值） */
  disabledReason: string | null;

  /** 检测 TMDb 搜索功能是否可用 */
  checkTmdbSearchAvailability: () => Promise<void>;
  /** 搜索 TMDb 候选（仅在功能可用时有效） */
  searchCandidates: (input: SearchTmdbCandidatesInput) => Promise<void>;
  /** 重置状态 */
  reset: () => void;
}

const DISABLED_GATE_MESSAGE = 'TMDb live search is not enabled.';

function isDisabledResponse(output: SearchTmdbCandidatesOutput): boolean {
  return (
    output.error !== null &&
    output.error.code === 'Unknown' &&
    output.error.message.includes(DISABLED_GATE_MESSAGE)
  );
}

export const useTmdbSearchStore = create<TmdbSearchState>((set) => ({
  tmdbSearchStatus: 'idle',
  lastResult: null,
  disabledReason: null,

  checkTmdbSearchAvailability: async () => {
    try {
      // 发送一个空查询来检测功能状态
      const output = await invoke<SearchTmdbCandidatesOutput>(
        'search_tmdb_candidates',
        {
          input: {
            media_type: 'Movie',
            title: '__availability_check__',
            year: null,
            language: null,
            page: null,
          },
        },
      );

      if (isDisabledResponse(output)) {
        set({
          tmdbSearchStatus: 'disabled',
          disabledReason: output.error?.message ?? 'TMDb search is not enabled.',
          lastResult: null,
        });
      } else {
        set({
          tmdbSearchStatus: 'idle',
          disabledReason: null,
        });
      }
    } catch {
      // 命令不存在或调用失败 — 视为禁用
      set({
        tmdbSearchStatus: 'disabled',
        disabledReason: 'TMDb search command is not available.',
        lastResult: null,
      });
    }
  },

  searchCandidates: async (input: SearchTmdbCandidatesInput) => {
    const { tmdbSearchStatus } = useTmdbSearchStore.getState();
    if (tmdbSearchStatus === 'disabled') {
      return;
    }

    set({ tmdbSearchStatus: 'loading' });

    try {
      const output = await invoke<SearchTmdbCandidatesOutput>(
        'search_tmdb_candidates',
        { input },
      );

      if (isDisabledResponse(output)) {
        set({
          tmdbSearchStatus: 'disabled',
          disabledReason: output.error?.message ?? 'TMDb search is not enabled.',
          lastResult: null,
        });
      } else if (output.error !== null) {
        set({
          tmdbSearchStatus: 'error',
          lastResult: output,
        });
      } else {
        set({
          tmdbSearchStatus: 'success',
          lastResult: output,
        });
      }
    } catch {
      set({
        tmdbSearchStatus: 'disabled',
        disabledReason: 'TMDb search command is not available.',
        lastResult: null,
      });
    }
  },

  reset: () => {
    set({
      tmdbSearchStatus: 'idle',
      lastResult: null,
      disabledReason: null,
    });
  },
}));
