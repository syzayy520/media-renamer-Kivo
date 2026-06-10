import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type {
  SearchTmdbCandidatesInput,
  SearchTmdbCandidatesOutput,
  TmdbCandidate,
  TmdbSearchStatus,
} from '../types';

interface TmdbConfigStatus {
  api_key_configured: boolean;
  gate_enabled: boolean;
  status: string;
  message: string;
}

/** 单个预览项的搜索状态 */
export interface ItemSearchState {
  status: TmdbSearchStatus;
  results: TmdbCandidate[];
  selectedCandidate: TmdbCandidate | null;
  error: string | null;
}

interface TmdbSearchState {
  /** 功能状态：disabled = 后端未启用, idle = 可用未搜索 */
  tmdbSearchStatus: TmdbSearchStatus;
  /** 最近一次搜索结果 */
  lastResult: SearchTmdbCandidatesOutput | null;
  /** 禁用原因（仅 status=disabled 时有值） */
  disabledReason: string | null;
  /** 当前正在搜索的预览项 ID */
  activeItemId: string | null;
  /** 每个预览项的搜索状态 */
  itemStates: Record<string, ItemSearchState>;

  /** 检测 TMDb 搜索功能是否可用 */
  checkTmdbSearchAvailability: () => Promise<void>;
  /** 便捷搜索：返回候选列表 */
  searchTmdbCandidates: (query: string, mediaType: 'Movie' | 'Series') => Promise<TmdbCandidate[] | null>;
  /** 搜索 TMDb 候选（仅在功能可用时有效） */
  searchCandidates: (input: SearchTmdbCandidatesInput) => Promise<void>;
  /** 为特定预览项搜索候选 */
  searchForItem: (itemId: string, input: SearchTmdbCandidatesInput) => Promise<void>;
  /** 选择候选 */
  selectCandidate: (itemId: string, candidate: TmdbCandidate) => void;
  /** 清除选择 */
  clearSelection: (itemId: string) => void;
  /** 获取某项的搜索状态 */
  getItemState: (itemId: string) => ItemSearchState;
  /** 设置活跃项 */
  setActiveItem: (itemId: string | null) => void;
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

const DEFAULT_ITEM_STATE: ItemSearchState = {
  status: 'idle',
  results: [],
  selectedCandidate: null,
  error: null,
};

async function readConfigStatus(): Promise<TmdbConfigStatus> {
  return invoke<TmdbConfigStatus>('get_tmdb_config_status');
}

function assertTmdbEnabled(configStatus: TmdbConfigStatus) {
  if (!configStatus.api_key_configured || !configStatus.gate_enabled) {
    throw new Error(configStatus.message || 'TMDb 未配置或未启用。');
  }
}

export const useTmdbSearchStore = create<TmdbSearchState>((set, get) => ({
  tmdbSearchStatus: 'idle',
  lastResult: null,
  disabledReason: null,
  activeItemId: null,
  itemStates: {},

  checkTmdbSearchAvailability: async () => {
    try {
      const configStatus = await readConfigStatus();

      if (!configStatus.api_key_configured) {
        set({
          tmdbSearchStatus: 'disabled',
          disabledReason: configStatus.message,
          lastResult: null,
        });
      } else if (!configStatus.gate_enabled) {
        set({
          tmdbSearchStatus: 'disabled',
          disabledReason: configStatus.message,
          lastResult: null,
        });
      } else {
        set({
          tmdbSearchStatus: 'idle',
          disabledReason: null,
        });
      }
    } catch {
      set({
        tmdbSearchStatus: 'disabled',
        disabledReason: '无法获取 TMDb 配置状态。',
        lastResult: null,
      });
    }
  },

  searchTmdbCandidates: async (query: string, mediaType: 'Movie' | 'Series'): Promise<TmdbCandidate[] | null> => {
    const configStatus = await readConfigStatus();
    assertTmdbEnabled(configStatus);

    set({ tmdbSearchStatus: 'loading', disabledReason: null });

    const output = await invoke<SearchTmdbCandidatesOutput>(
      'search_tmdb_candidates',
      { input: { query, media_type: mediaType, language: 'zh-CN', year: null, page: 1 } },
    );

    if (isDisabledResponse(output)) {
      const message = output.error?.message ?? 'TMDb search is not enabled.';
      set({ tmdbSearchStatus: 'disabled', disabledReason: message, lastResult: null });
      throw new Error(message);
    }

    if (output.error !== null) {
      set({ tmdbSearchStatus: 'error', lastResult: output });
      throw new Error(output.error.message);
    }

    set({ tmdbSearchStatus: 'success', lastResult: output });
    return output.candidates;
  },

  searchCandidates: async (input: SearchTmdbCandidatesInput) => {
    const { tmdbSearchStatus } = get();
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

  searchForItem: async (itemId: string, input: SearchTmdbCandidatesInput) => {
    const { tmdbSearchStatus, itemStates } = get();
    if (tmdbSearchStatus === 'disabled') {
      return;
    }

    set({
      activeItemId: itemId,
      itemStates: {
        ...itemStates,
        [itemId]: { ...DEFAULT_ITEM_STATE, status: 'loading' },
      },
    });

    try {
      const output = await invoke<SearchTmdbCandidatesOutput>(
        'search_tmdb_candidates',
        { input },
      );

      const currentState = get().itemStates;

      if (isDisabledResponse(output)) {
        set({
          tmdbSearchStatus: 'disabled',
          disabledReason: output.error?.message ?? 'TMDb search is not enabled.',
          itemStates: {
            ...currentState,
            [itemId]: {
              ...DEFAULT_ITEM_STATE,
              status: 'disabled',
              error: output.error?.message ?? null,
            },
          },
        });
      } else if (output.error !== null) {
        set({
          itemStates: {
            ...currentState,
            [itemId]: {
              ...DEFAULT_ITEM_STATE,
              status: 'error',
              error: output.error.message,
            },
          },
        });
      } else {
        set({
          lastResult: output,
          itemStates: {
            ...currentState,
            [itemId]: {
              status: 'success',
              results: output.candidates,
              selectedCandidate: null,
              error: null,
            },
          },
        });
      }
    } catch {
      const currentState = get().itemStates;
      set({
        itemStates: {
          ...currentState,
          [itemId]: {
            ...DEFAULT_ITEM_STATE,
            status: 'error',
            error: 'TMDb search command failed.',
          },
        },
      });
    }
  },

  selectCandidate: (itemId: string, candidate: TmdbCandidate) => {
    const { itemStates } = get();
    const current = itemStates[itemId] ?? DEFAULT_ITEM_STATE;
    set({
      itemStates: {
        ...itemStates,
        [itemId]: { ...current, selectedCandidate: candidate },
      },
    });
  },

  clearSelection: (itemId: string) => {
    const { itemStates } = get();
    const current = itemStates[itemId] ?? DEFAULT_ITEM_STATE;
    set({
      itemStates: {
        ...itemStates,
        [itemId]: { ...current, selectedCandidate: null },
      },
    });
  },

  getItemState: (itemId: string) => {
    return get().itemStates[itemId] ?? DEFAULT_ITEM_STATE;
  },

  setActiveItem: (itemId: string | null) => {
    set({ activeItemId: itemId });
  },

  reset: () => {
    set({
      tmdbSearchStatus: 'idle',
      lastResult: null,
      disabledReason: null,
      activeItemId: null,
      itemStates: {},
    });
  },
}));
