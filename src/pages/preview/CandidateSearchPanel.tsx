import { useState, useCallback } from 'react';
import { Search, Loader2 } from 'lucide-react';
import { Button, Input } from '../../components/ui';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import type { RenamePreviewItem } from '../../types';

interface CandidateSearchPanelProps {
  item: RenamePreviewItem;
  onSearchComplete?: () => void;
}

export function CandidateSearchPanel({ item, onSearchComplete }: CandidateSearchPanelProps) {
  const { tmdbSearchStatus, searchForItem, activeItemId } = useTmdbSearchStore();

  const defaultQuery = item.parsed_info.title !== 'Unknown' ? item.parsed_info.title : '';
  const defaultMediaType = item.parsed_info.media_type === 'Series' || item.parsed_info.media_type === 'Anime'
    ? 'Tv' as const
    : 'Movie' as const;

  const [query, setQuery] = useState(defaultQuery);
  const [mediaType, setMediaType] = useState<'Movie' | 'Tv'>(defaultMediaType);

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

  return (
    <div className="flex items-end gap-3 p-4 bg-bg-secondary rounded-lg border border-border">
      <div className="flex-1 min-w-0">
        <label className="block text-xs text-text-secondary mb-1">搜索关键词</label>
        <Input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="输入电影/剧集名称..."
          disabled={isDisabled}
          className="w-full"
        />
      </div>

      <div className="w-28 shrink-0">
        <label className="block text-xs text-text-secondary mb-1">类型</label>
        <select
          value={mediaType}
          onChange={(e) => setMediaType(e.target.value as 'Movie' | 'Tv')}
          disabled={isDisabled}
          className="w-full h-10 px-3 rounded-md border border-border bg-bg-primary text-text-primary text-sm focus:outline-none focus:ring-2 focus:ring-accent/50"
        >
          <option value="Movie">电影</option>
          <option value="Tv">剧集</option>
        </select>
      </div>

      <Button
        onClick={handleSearch}
        disabled={!query.trim() || isDisabled || isLoading}
        icon={isLoading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Search className="w-4 h-4" />}
        className="shrink-0"
      >
        {isLoading ? '搜索中...' : 'TMDb 搜索'}
      </Button>
    </div>
  );
}
