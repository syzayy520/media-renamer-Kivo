import { useState, useCallback } from 'react';
import { X, Search, Globe } from 'lucide-react';
import type { FolderGroup, TmdbCandidate } from '../../../types';
import { Button, Card, Badge } from '../../../components/ui';
import { useTmdbSearchStore } from '../../../state/tmdbSearchStore';
import { TmdbCandidateCard } from './TmdbCandidateCard';

export interface TmdbCandidatePanelProps {
  groupId: string;
  groups: FolderGroup[];
  onClose: () => void;
  onApply: () => void;
}

const TMDB_IMAGE_BASE = 'https://image.tmdb.org/t/p/w500';

export function TmdbCandidatePanel({ groupId, groups, onClose, onApply }: TmdbCandidatePanelProps) {
  const group = groups.find((g) => g.id === groupId);
  const { searchTmdbCandidates } = useTmdbSearchStore();
  const [candidates, setCandidates] = useState<TmdbCandidate[]>([]);
  const [selectedCandidate, setSelectedCandidate] = useState<TmdbCandidate | null>(null);
  const [searchLoading, setSearchLoading] = useState(false);
  const [searchError, setSearchError] = useState<string | null>(null);

  const handleSearch = useCallback(async () => {
    if (!group) return;
    setSearchLoading(true);
    setSearchError(null);
    try {
      const firstChild = group.children[0];
      const rawName = firstChild?.target_name || group.target_folder_name;
      const searchName = rawName.replace(/\.[^.]+$/, '');
      const result = await searchTmdbCandidates(searchName, group.media_type === 'Tv' ? 'Series' : 'Movie');
      if (result) {
        setCandidates(result);
      }
    } catch (err) {
      setSearchError(err instanceof Error ? err.message : '搜索失败');
    } finally {
      setSearchLoading(false);
    }
  }, [group, searchTmdbCandidates]);

  const handleApplyCandidate = useCallback(async (_candidate: TmdbCandidate) => {
    if (!group) return;
    onApply();
  }, [group, onApply]);

  if (!group) {
    return (
      <Card className="p-4 text-center text-sm text-text-secondary">
        未找到文件夹组信息
        <Button variant="ghost" size="sm" onClick={onClose} className="mt-2">关闭</Button>
      </Card>
    );
  }

  return (
    <Card className="p-0 overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-border bg-surface-subtle">
        <div className="flex items-center gap-2">
          <Globe className="h-4 w-4 text-primary" />
          <h3 className="text-sm font-semibold text-text-primary">TMDb 候选匹配</h3>
          <Badge variant="default" size="sm">{group.target_folder_name}</Badge>
        </div>
        <Button variant="ghost" size="sm" icon={<X className="h-4 w-4" />} onClick={onClose}>{''}</Button>
      </div>

      {/* Search Area */}
      <div className="p-4 border-b border-border">
        <div className="flex items-center gap-2">
          <Button
            variant="primary"
            size="sm"
            icon={<Search className="h-4 w-4" />}
            onClick={handleSearch}
            isLoading={searchLoading}
          >
            搜索 TMDb
          </Button>
          <span className="text-xs text-text-tertiary">
            搜索: {group.target_folder_name}
          </span>
        </div>
        {searchError && (
          <p className="mt-2 text-xs text-error">{searchError}</p>
        )}
      </div>

      {/* Candidate Cards */}
      <div className="p-4 max-h-96 overflow-y-auto">
        {searchLoading && (
          <div className="flex items-center gap-2 py-8 justify-center">
            <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
            <span className="text-sm text-text-secondary">搜索中...</span>
          </div>
        )}

        {!searchLoading && candidates.length === 0 && !searchError && (
          <div className="flex flex-col items-center gap-2 py-8 text-text-tertiary">
            <Search className="h-8 w-8" />
            <span className="text-sm">点击搜索按钮搜索 TMDb 候选</span>
          </div>
        )}

        {candidates.map((candidate) => (
          <TmdbCandidateCard
            key={candidate.tmdb_id}
            candidate={candidate}
            selected={selectedCandidate?.tmdb_id === candidate.tmdb_id}
            onSelect={() => setSelectedCandidate(candidate)}
            onApply={() => handleApplyCandidate(candidate)}
            imageBaseUrl={TMDB_IMAGE_BASE}
          />
        ))}
      </div>

      {/* Apply Button */}
      {selectedCandidate && (
        <div className="border-t border-border px-4 py-3 bg-surface-subtle">
          <div className="flex items-center justify-between">
            <span className="text-xs text-text-secondary">
              已选择: {selectedCandidate.title} ({selectedCandidate.year || '未知年份'})
            </span>
            <Button variant="primary" size="sm" onClick={() => handleApplyCandidate(selectedCandidate)}>
              应用到 "{group.target_folder_name}"
            </Button>
          </div>
        </div>
      )}
    </Card>
  );
}
