// pages/preview/CandidateSearchPanel 组件
// 职责：TMDb 搜索输入面板，支持关键词、类型、年份

import { useState, useCallback } from 'react';
import { Search, Loader2, Key } from 'lucide-react';
import { Button, Input } from '../../components/ui';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import type { RenamePreviewItem, SearchTmdbCandidatesInput } from '../../types';

interface CandidateSearchPanelProps {
  item: RenamePreviewItem;
  onSearchComplete?: () => void;
}

type SearchMediaType = SearchTmdbCandidatesInput['media_type'];

export function CandidateSearchPanel({ item, onSearchComplete }: CandidateSearchPanelProps) {
  const { tmdbSearchStatus, searchForItem, activeItemId, disabledReason } = useTmdbSearchStore();

  const defaultQuery = item.parsed_info.title !== 'Unknown' ? item.parsed_info.title : '';
  const defaultMediaType: SearchMediaType = item.parsed_info.media_type === 'Series' || item.parsed_info.media_type === 'Anime'
    ? 'tv'
    : 'movie';

  const [query, setQuery] = useState(defaultQuery);
  const [mediaType, setMediaType] = useState<SearchMediaType>(defaultMediaType);

  const isLoading = activeItemId === item.id && tmdbSearchStatus === 'loading';
  const isDisabled = tmdbSearchStatus === 'disabled';

  const handleSearch = useCallback(async () => {
    if (!query.trim() || isDisabled) return;

    await searchForItem(item.id, {
      query: query.trim(),
      media_type: mediaType,
      language: 'zh-CN',
      year: item.parsed_info.year ?? null,
      page: null,
    });

    onSearchComplete?.();
  }, [query, mediaType, item.id, item.parsed_info.year, isDisabled, searchForItem, onSearchComplete]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  if (isDisabled) {
    return (
      <div className="flex items-center gap-3 p-4 bg-bg-secondary rounded-lg border border-warning/20">
        <Key className="w-5 h-5 text-warning shrink-0" />
        <div className="flex-1 min-w-0">
          <p className="text-sm text-text-primary font-medium">TMDb 搜索不可用</p>
          <p className="text-xs text-text-secondary mt-0.5">
            {disabledReason ?? '请在设置中配置 TMDb API Key。'}
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex items-end gap-3 p-4 bg-bg-secondary rounded-lg border border-border">
      <div className="flex-1 min-w-0">
        <label className="block text-xs text-text-secondary mb-1">搜索关键词</label>
        <Input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="输入电影/剧集名称..."
          disabled={isLoading}
          className="w-full"
        />
      </div>

      <div className="w-28 shrink-0">
        <label className="block text-xs text-text-secondary mb-1">类型</label>
        <select
          value={mediaType}
          onChange={(e) => setMediaType(e.target.value as SearchMediaType)}
          disabled={isLoading}
          className="w-full h-10 px-3 rounded-md border border-border bg-bg-primary text-text-primary text-sm focus:outline-none focus:ring-2 focus:ring-accent/50"
        >
          <option value="movie">电影</option>
          <option value="tv">剧集</option>
        </select>
      </div>

      {item.parsed_info.year && (
        <div className="w-20 shrink-0">
          <label className="block text-xs text-text-secondary mb-1">年份</label>
          <div className="h-10 px-3 flex items-center rounded-md border border-border bg-bg-primary text-text-primary text-sm">
            {item.parsed_info.year}
          </div>
        </div>
      )}

      <Button
        onClick={handleSearch}
        disabled={!query.trim() || isLoading}
        icon={isLoading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Search className="w-4 h-4" />}
        className="shrink-0"
      >
        {isLoading ? '搜索中...' : 'TMDb 搜索'}
      </Button>
    </div>
  );
}
