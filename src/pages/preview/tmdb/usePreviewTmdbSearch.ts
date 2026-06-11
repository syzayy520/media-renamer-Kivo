import { useCallback, useEffect, useState } from 'react';
import type { FolderGroup, TmdbCandidate, TmdbSearchStatus } from '../../../types';
import { buildTmdbQuery, defaultTmdbMediaType } from './query';
import type { TmdbManualMediaType } from './tmdbInspectorTypes';

interface UsePreviewTmdbSearchParams {
  groups: FolderGroup[];
  selectedGroup: FolderGroup | null;
  tmdbSearchStatus: TmdbSearchStatus;
  disabledReason?: string | null;
  onGroupSelect: (groupId: string) => void;
  onSearchCandidates: (query: string, mediaType: TmdbManualMediaType) => Promise<TmdbCandidate[]>;
  onApplyCandidate: (targetId: string, candidate: TmdbCandidate) => Promise<void> | void;
  onRefreshSafetySummary: () => Promise<void> | void;
}

export function usePreviewTmdbSearch({
  groups,
  selectedGroup,
  tmdbSearchStatus,
  disabledReason,
  onGroupSelect,
  onSearchCandidates,
  onApplyCandidate,
  onRefreshSafetySummary,
}: UsePreviewTmdbSearchParams) {
  const [tmdbMediaType, setTmdbMediaType] = useState<TmdbManualMediaType>('movie');
  const [tmdbQuery, setTmdbQuery] = useState('');
  const [tmdbCandidates, setTmdbCandidates] = useState<TmdbCandidate[]>([]);
  const [tmdbLoading, setTmdbLoading] = useState(false);
  const [tmdbError, setTmdbError] = useState<string | null>(null);
  const [selectedCandidate, setSelectedCandidate] = useState<TmdbCandidate | null>(null);

  const clearCandidates = useCallback(() => {
    setTmdbCandidates([]);
    setSelectedCandidate(null);
  }, []);

  useEffect(() => {
    setTmdbMediaType(defaultTmdbMediaType(selectedGroup));
    setTmdbQuery(selectedGroup ? buildTmdbQuery(selectedGroup) : '');
    setTmdbError(null);
    clearCandidates();
  }, [clearCandidates, selectedGroup?.id]);

  const searchTmdbForGroup = useCallback(async (
    group: FolderGroup,
    mediaType: TmdbManualMediaType,
    queryText: string,
  ) => {
    onGroupSelect(group.id);
    setTmdbError(null);
    clearCandidates();

    if (tmdbSearchStatus === 'disabled') {
      setTmdbError(disabledReason || 'TMDb 未启用，请先到设置配置 API key 并开启 Live Search。');
      return;
    }

    const query = queryText.trim() || buildTmdbQuery(group);
    if (!query) {
      setTmdbError('请输入 TMDb 搜索词');
      return;
    }

    setTmdbLoading(true);
    try {
      const result = await onSearchCandidates(query, mediaType);
      if (!result || result.length === 0) {
        setTmdbError(`没有找到候选：${query}。可以把搜索词改成英文名再试，例如 First Blood。`);
        return;
      }
      setTmdbCandidates(result);
    } catch (err) {
      setTmdbError(err instanceof Error ? err.message : '搜索失败');
    } finally {
      setTmdbLoading(false);
    }
  }, [clearCandidates, disabledReason, onGroupSelect, onSearchCandidates, tmdbSearchStatus]);

  const handleTmdbSearch = useCallback(async () => {
    if (!selectedGroup) return;
    await searchTmdbForGroup(selectedGroup, tmdbMediaType, tmdbQuery);
  }, [selectedGroup, searchTmdbForGroup, tmdbMediaType, tmdbQuery]);

  const handleGroupTmdbSearch = useCallback((groupId: string) => {
    const group = groups.find((item) => item.id === groupId);
    if (!group) return;
    const mediaType = defaultTmdbMediaType(group);
    const query = buildTmdbQuery(group);
    setTmdbMediaType(mediaType);
    setTmdbQuery(query);
    void searchTmdbForGroup(group, mediaType, query);
  }, [groups, searchTmdbForGroup]);

  const handleApplyCandidate = useCallback(async (candidate: TmdbCandidate) => {
    if (!selectedGroup) return;
    const mainVideo = selectedGroup.children.find((child) => child.file_role === 'MainVideo');
    const targetId = mainVideo?.id || selectedGroup.children[0]?.id;
    if (!targetId) return;
    await onApplyCandidate(targetId, candidate);
    await onRefreshSafetySummary();
  }, [selectedGroup, onApplyCandidate, onRefreshSafetySummary]);

  return {
    tmdbMediaType,
    tmdbQuery,
    tmdbLoading,
    tmdbError,
    tmdbCandidates,
    selectedCandidate,
    setTmdbMediaType,
    setTmdbQuery,
    setSelectedCandidate,
    handleTmdbSearch,
    handleGroupTmdbSearch,
    handleApplyCandidate,
    clearCandidates,
  };
}
