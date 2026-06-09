// features/preview-workbench/candidate-search/TmdbSearchPanel.tsx — TMDb 搜索面板（Mock UI）
// 职责：展示 Mock TMDb 搜索界面，所有输入和按钮均为 disabled 状态
// 安全边界：不调用 backend command，不 invoke，不 fetch，不读取 API key

import { useState } from 'react';
import { TmdbSearchInput } from './TmdbSearchInput';
import { TmdbSearchResultList } from './TmdbSearchResultList';
import { TmdbSearchEmptyState } from './TmdbSearchEmptyState';
import { TmdbSearchLoadingState } from './TmdbSearchLoadingState';
import { TmdbSearchErrorState } from './TmdbSearchErrorState';
import type { CandidateSearchMockMode } from '../types/candidateSearch';

/**
 * TMDb 搜索面板 Mock UI
 * 展示未来 TMDb 搜索界面的骨架，所有功能均为 disabled 状态
 */
export function TmdbSearchPanel() {
  // Mock 状态：默认为 idle，可通过按钮切换查看不同状态（仅用于 UI 展示）
  const [mockMode, setMockMode] = useState<CandidateSearchMockMode>('idle');

  return (
    <div className="flex flex-col gap-3">
      {/* Mock UI 标识 */}
      <div className="rounded-md bg-yellow-500/10 px-3 py-2 text-xs text-yellow-400/80">
        <div className="font-medium">TMDb search is mocked in P2-010</div>
        <div className="mt-1 text-yellow-400/60">
          No network request is sent. API key is not read by this UI.
        </div>
      </div>

      {/* 搜索输入区（disabled） */}
      <TmdbSearchInput />

      {/* Mock 状态切换按钮（仅用于展示不同 UI 状态） */}
      <div className="flex gap-2 text-xs">
        <button
          onClick={() => setMockMode('idle')}
          className="rounded bg-white/5 px-2 py-1 text-white/40 hover:bg-white/10"
        >
          Idle
        </button>
        <button
          onClick={() => setMockMode('loading')}
          className="rounded bg-white/5 px-2 py-1 text-white/40 hover:bg-white/10"
        >
          Loading
        </button>
        <button
          onClick={() => setMockMode('success')}
          className="rounded bg-white/5 px-2 py-1 text-white/40 hover:bg-white/10"
        >
          Success
        </button>
        <button
          onClick={() => setMockMode('error')}
          className="rounded bg-white/5 px-2 py-1 text-white/40 hover:bg-white/10"
        >
          Error
        </button>
        <button
          onClick={() => setMockMode('empty')}
          className="rounded bg-white/5 px-2 py-1 text-white/40 hover:bg-white/10"
        >
          Empty
        </button>
      </div>

      {/* 搜索结果区 */}
      <div className="min-h-[200px]">
        {mockMode === 'idle' && (
          <div className="flex items-center justify-center h-32 text-xs text-white/30">
            Enter search query to find candidates
          </div>
        )}
        {mockMode === 'loading' && <TmdbSearchLoadingState />}
        {mockMode === 'success' && <TmdbSearchResultList />}
        {mockMode === 'error' && <TmdbSearchErrorState />}
        {mockMode === 'empty' && <TmdbSearchEmptyState />}
      </div>
    </div>
  );
}
