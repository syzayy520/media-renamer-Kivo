import { Globe } from 'lucide-react';
import { Badge, Button, Card } from '../../../components/ui';
import { SearchTypeButton } from './SearchTypeButton';
import type { TmdbSearchCardProps } from './tmdbInspectorTypes';

export function TmdbSearchCard({
  selectedGroup,
  tmdbMediaType,
  tmdbQuery,
  tmdbLoading,
  tmdbDisabled,
  onQueryChange,
  onMediaTypeChange,
  onSearch,
}: TmdbSearchCardProps) {
  return (
    <Card className="p-3">
      <div className="mb-2 flex items-center gap-2">
        <span className="text-sm">{selectedGroup.media_type === 'Movie' ? '🎬' : selectedGroup.media_type === 'Tv' ? '📺' : '📁'}</span>
        <h3 className="truncate text-sm font-semibold text-text-primary">{selectedGroup.target_folder_name}</h3>
        <Badge variant="default" size="sm">{selectedGroup.media_type} · {selectedGroup.file_count} 文件</Badge>
      </div>
      <div className="space-y-1 text-xs text-text-tertiary">
        <div>原路径：{selectedGroup.original_path}</div>
        <div>新路径：{selectedGroup.target_path}</div>
      </div>
      <div className="mt-3 space-y-3 border-t border-border pt-3">
        <div>
          <div className="mb-1 text-xs text-text-secondary">搜索词</div>
          <input
            className="w-full rounded-xl border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary focus:border-primary focus:outline-none"
            value={tmdbQuery}
            onChange={(event) => onQueryChange(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === 'Enter') onSearch();
            }}
            placeholder="可改成英文名，例如 First Blood"
          />
        </div>
        <div>
          <div className="mb-1 text-xs text-text-secondary">搜索类型</div>
          <div className="grid grid-cols-2 gap-2 rounded-xl bg-bg-primary p-1">
            <SearchTypeButton active={tmdbMediaType === 'movie'} onClick={() => onMediaTypeChange('movie')}>电影</SearchTypeButton>
            <SearchTypeButton active={tmdbMediaType === 'tv'} onClick={() => onMediaTypeChange('tv')}>电视剧</SearchTypeButton>
          </div>
        </div>
        <Button
          variant="primary"
          size="sm"
          icon={<Globe className="h-3.5 w-3.5" />}
          onClick={onSearch}
          isLoading={tmdbLoading}
          disabled={tmdbDisabled || !tmdbQuery.trim()}
          className="w-full"
        >
          {tmdbDisabled ? '先启用 TMDb' : '搜索 TMDb'}
        </Button>
      </div>
    </Card>
  );
}
