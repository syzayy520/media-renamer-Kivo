import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  ArrowLeft,
  Check,
  Edit3,
  Film,
  Globe,
  Image,
  Play,
  Settings,
  Star,
  X,
} from 'lucide-react';
import { Badge, Button, Card } from '../../components/ui';
import { useExecutionStore } from '../../state/executionStore';
import { usePipelineStore } from '../../state/pipelineStore';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import type {
  FolderGroup,
  FolderPolicyConfig,
  NamingRule,
  SearchTmdbCandidatesInput,
  TitleStrategy,
  TmdbCandidate,
} from '../../types';
import { ExecutionConfirmPanel } from './ExecutionConfirmPanel';
import { ExecutionProgressPanel } from './ExecutionProgressPanel';
import { ExecutionResultPanel } from './ExecutionResultPanel';
import { buildPreviewGroups } from './model/buildPreviewGroups';
import { buildInitialNamingWorkbenchState, PreviewNamingWorkbenchPanel } from './naming/workbench';
import { ScrapeWriteActions } from './scrape/ScrapeWriteActions';
import { PreviewPlanTree } from './tree/PreviewPlanTree';

const TMDB_IMAGE_BASE = 'https://image.tmdb.org/t/p/w185';
type TmdbManualMediaType = SearchTmdbCandidatesInput['media_type'];

const INITIAL_NAMING_WORKBENCH = buildInitialNamingWorkbenchState();

function defaultTmdbMediaType(group: FolderGroup | null): TmdbManualMediaType {
  return group?.media_type === 'Tv' ? 'tv' : 'movie';
}

function buildTmdbQuery(group: FolderGroup): string {
  const mainVideo = group.children.find((child) => child.file_role === 'MainVideo');
  const rawName = group.target_folder_name || mainVideo?.target_name || group.original_folder_name;
  const normalized = rawName
    .replace(/\.[a-z0-9]{2,5}$/i, '')
    .replace(/^\.+/, '')
    .replace(/[._]+/g, ' ')
    .replace(/\[[^\]]*\]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
  const beforeYear = normalized.match(/^(.*?)(?:\s*\(?((?:19|20)\d{2})\)?)(?:\s|$)/)?.[1]?.trim();

  if (beforeYear) return beforeYear;

  return normalized
    .replace(/\b(720p|1080p|2160p|4k|bluray|blu ray|web dl|web-dl|webrip|hdtv|remux|unrated|proper|repack|extended|gb|gbr|usa|x264|x265|hevc|h264|h265|vc1|vc|10bit|8bit|hdr|dv|truehd|dts|atmos)\b/gi, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

function isMovieCandidate(candidate: TmdbCandidate): boolean {
  const mediaType = candidate.media_type as string;
  return mediaType === 'Movie' || mediaType === 'movie';
}

export function PreviewPage() {
  const navigate = useNavigate();
  const {
    pipelineResult,
    updatePreviewProposedName,
    togglePreviewSkipped,
    refreshSafetySummary,
    applyNamingRule,
    applyFolderPolicy,
    applyTmdbCandidate,
  } = usePipelineStore();
  const { isLoading } = useUiFeedbackStore();
  const { tmdbSearchStatus, disabledReason, searchTmdbCandidates, checkTmdbSearchAvailability } = useTmdbSearchStore();
  const {
    uiState,
    confirmState,
    progressState,
    resultState,
    startExecution,
    confirmExecution,
    cancelExecution,
    resetExecution,
  } = useExecutionStore();

  const [expandedGroups, setExpandedGroups] = useState<Set<string>>(new Set());
  const [selectedGroupId, setSelectedGroupId] = useState<string | null>(null);
  const [currentNamingRule, setCurrentNamingRule] = useState<NamingRule>(INITIAL_NAMING_WORKBENCH.currentNamingRule);
  const [currentStrategy, setCurrentStrategy] = useState<TitleStrategy>(INITIAL_NAMING_WORKBENCH.currentStrategy);
  const [selectedPreset, setSelectedPreset] = useState(INITIAL_NAMING_WORKBENCH.selectedPreset);
  const [folderPolicyConfig, setFolderPolicyConfig] = useState<FolderPolicyConfig>({
    policy: 'KeepOriginalStructure',
    clean_empty_folders_after: false,
  });
  const [editModal, setEditModal] = useState<EditModalState | null>(null);
  const [tmdbMediaType, setTmdbMediaType] = useState<TmdbManualMediaType>('movie');
  const [tmdbQuery, setTmdbQuery] = useState('');
  const [tmdbCandidates, setTmdbCandidates] = useState<TmdbCandidate[]>([]);
  const [tmdbLoading, setTmdbLoading] = useState(false);
  const [tmdbError, setTmdbError] = useState<string | null>(null);
  const [selectedCandidate, setSelectedCandidate] = useState<TmdbCandidate | null>(null);
  const namingDebounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    checkTmdbSearchAvailability();
  }, [checkTmdbSearchAvailability]);

  const scanRoot = pipelineResult?.scan?.scan_path;
  const planTree = useMemo(() => {
    if (!pipelineResult) return null;
    return buildPreviewGroups(pipelineResult.previews, scanRoot);
  }, [pipelineResult, scanRoot]);
  const groups = planTree?.groups ?? [];
  const selectedGroup = groups.find((group) => group.id === selectedGroupId) ?? null;

  useEffect(() => {
    if (groups.length === 0) return;
    if (!selectedGroupId || !groups.some((group) => group.id === selectedGroupId)) {
      setSelectedGroupId(groups[0].id);
    }
  }, [groups, selectedGroupId]);

  useEffect(() => {
    setTmdbMediaType(defaultTmdbMediaType(selectedGroup));
    setTmdbQuery(selectedGroup ? buildTmdbQuery(selectedGroup) : '');
    setTmdbError(null);
    setTmdbCandidates([]);
    setSelectedCandidate(null);
  }, [selectedGroup?.id]);

  useEffect(() => {
    return () => {
      if (namingDebounceRef.current) clearTimeout(namingDebounceRef.current);
    };
  }, []);

  const triggerNamingApply = useCallback((rule: NamingRule, strategy: TitleStrategy) => {
    if (namingDebounceRef.current) clearTimeout(namingDebounceRef.current);
    namingDebounceRef.current = setTimeout(() => applyNamingRule(rule, strategy), 150);
  }, [applyNamingRule]);

  const handleFolderPolicyChange = useCallback((config: FolderPolicyConfig) => {
    setFolderPolicyConfig(config);
    applyFolderPolicy(config.policy);
  }, [applyFolderPolicy]);

  const handleToggleExpand = useCallback((groupId: string) => {
    setExpandedGroups((prev) => {
      const next = new Set(prev);
      if (next.has(groupId)) next.delete(groupId);
      else next.add(groupId);
      return next;
    });
    setSelectedGroupId(groupId);
  }, []);

  const searchTmdbForGroup = useCallback(async (
    group: FolderGroup,
    mediaType: TmdbManualMediaType,
    queryText: string,
  ) => {
    setSelectedGroupId(group.id);
    setTmdbError(null);
    setTmdbCandidates([]);
    setSelectedCandidate(null);

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
      const result = await searchTmdbCandidates(query, mediaType);
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
  }, [disabledReason, searchTmdbCandidates, tmdbSearchStatus]);

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
    await applyTmdbCandidate(targetId, candidate);
    await refreshSafetySummary();
  }, [selectedGroup, applyTmdbCandidate, refreshSafetySummary]);

  const openGroupEdit = useCallback((groupId: string) => {
    const group = groups.find((item) => item.id === groupId);
    if (!group || group.children.length === 0) return;
    const mainFile = group.children.find((child) => child.file_role === 'MainVideo') || group.children[0];
    setEditModal({
      mode: 'group',
      targetId: mainFile.id,
      currentName: group.target_folder_name,
      previewPath: group.target_path,
    });
  }, [groups]);

  const openFileEdit = useCallback((fileId: string) => {
    const item = pipelineResult?.previews.find((preview) => preview.id === fileId);
    if (!item) return;
    setEditModal({
      mode: 'file',
      targetId: fileId,
      currentName: item.proposed_name,
      previewPath: item.target_path,
    });
  }, [pipelineResult]);

  const confirmEdit = useCallback((newName: string) => {
    if (!editModal) return;
    updatePreviewProposedName(editModal.targetId, newName);
    setEditModal(null);
  }, [editModal, updatePreviewProposedName]);

  const handleGroupSkip = useCallback((groupId: string) => {
    const group = groups.find((item) => item.id === groupId);
    if (!group) return;
    group.children.forEach((child) => togglePreviewSkipped(child.id));
  }, [groups, togglePreviewSkipped]);

  if (isLoading) {
    return (
      <Card className="p-6">
        <div className="flex items-center gap-3">
          <div className="h-5 w-5 animate-spin rounded-full border-2 border-primary border-t-transparent" />
          <span className="text-sm text-text-secondary">加载中...</span>
        </div>
      </Card>
    );
  }

  if (!pipelineResult) {
    return (
      <Card className="p-6 text-center">
        <p className="mb-4 text-sm text-text-secondary">暂无预览结果，请先扫描文件</p>
        <Button onClick={() => navigate('/scan')}>去扫描</Button>
      </Card>
    );
  }

  return (
    <div className="flex h-[calc(100vh-6rem)] flex-col gap-3">
      <PreviewHeader
        groupCount={groups.length}
        fileCount={pipelineResult.previews.length}
        onBack={() => navigate('/scan')}
        onSettings={() => navigate('/settings')}
        onDryRun={() => startExecution('DryRun')}
        onExecute={() => startExecution('Confirmed')}
      />

      {tmdbSearchStatus === 'disabled' && (
        <TmdbDisabledBanner reason={disabledReason} onSettings={() => navigate('/settings')} />
      )}

      <div className="flex min-h-0 flex-1 gap-3">
        <div className="flex w-[260px] shrink-0 flex-col gap-3 overflow-y-auto">
          <PreviewNamingWorkbenchPanel
            selectedPreset={selectedPreset}
            currentNamingRule={currentNamingRule}
            currentStrategy={currentStrategy}
            folderPolicyConfig={folderPolicyConfig}
            setSelectedPreset={setSelectedPreset}
            setCurrentNamingRule={setCurrentNamingRule}
            setCurrentStrategy={setCurrentStrategy}
            triggerNamingApply={triggerNamingApply}
            onFolderPolicyChange={handleFolderPolicyChange}
          />
        </div>

        <div className="flex min-w-0 flex-1 flex-col">
          <Card className="min-h-0 flex-1 overflow-y-auto p-0">
            <PreviewPlanTree
              groups={groups}
              expandedGroups={expandedGroups}
              onGroupToggleExpand={handleToggleExpand}
              onGroupTmdbSearch={handleGroupTmdbSearch}
              onGroupEdit={openGroupEdit}
              onGroupSkip={handleGroupSkip}
              onFileEdit={openFileEdit}
              onFileSkip={togglePreviewSkipped}
            />
          </Card>
        </div>

        <div className="flex w-[320px] shrink-0 flex-col gap-3 overflow-y-auto">
          <TmdbInspector
            selectedGroup={selectedGroup}
            tmdbMediaType={tmdbMediaType}
            tmdbQuery={tmdbQuery}
            tmdbLoading={tmdbLoading}
            tmdbError={tmdbError}
            tmdbDisabled={tmdbSearchStatus === 'disabled'}
            candidates={tmdbCandidates}
            selectedCandidate={selectedCandidate}
            onQueryChange={setTmdbQuery}
            onMediaTypeChange={setTmdbMediaType}
            onSearch={handleTmdbSearch}
            onCandidateSelect={setSelectedCandidate}
            onCandidateApply={handleApplyCandidate}
            onClearCandidates={() => {
              setTmdbCandidates([]);
              setSelectedCandidate(null);
            }}
          />
        </div>
      </div>

      {editModal && (
        <EditNameModal
          mode={editModal.mode}
          currentName={editModal.currentName}
          previewPath={editModal.previewPath}
          onConfirm={confirmEdit}
          onCancel={() => setEditModal(null)}
        />
      )}

      {uiState === 'confirming' && confirmState && (
        <ExecutionConfirmPanel state={confirmState} onConfirm={() => confirmExecution()} onCancel={cancelExecution} isLoading={false} />
      )}
      {uiState === 'executing' && progressState && <ExecutionProgressPanel state={progressState} />}
      {uiState === 'completed' && resultState && (
        <ExecutionResultPanel state={resultState} onRollback={() => {}} onExport={() => {}} onReset={resetExecution} />
      )}
    </div>
  );
}

type EditModalState = {
  mode: 'file' | 'group';
  targetId: string;
  currentName: string;
  previewPath: string;
};

function PreviewHeader({
  groupCount,
  fileCount,
  onBack,
  onSettings,
  onDryRun,
  onExecute,
}: {
  groupCount: number;
  fileCount: number;
  onBack: () => void;
  onSettings: () => void;
  onDryRun: () => void;
  onExecute: () => void;
}) {
  return (
    <div className="flex shrink-0 items-center justify-between">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="sm" icon={<ArrowLeft className="h-4 w-4" />} onClick={onBack}>返回</Button>
        <h2 className="text-lg font-semibold text-text-primary">预览重命名</h2>
        <Badge variant="default" size="sm">{groupCount} 组 · {fileCount} 文件</Badge>
      </div>
      <div className="flex items-center gap-2">
        <Button variant="secondary" size="sm" icon={<Settings className="h-4 w-4" />} onClick={onSettings}>设置</Button>
        <Button variant="secondary" size="sm" icon={<Play className="h-4 w-4" />} onClick={onDryRun}>模拟执行</Button>
        <Button variant="primary" size="sm" icon={<Play className="h-4 w-4" />} onClick={onExecute}>执行重命名</Button>
      </div>
    </div>
  );
}

function TmdbDisabledBanner({ reason, onSettings }: { reason: string | null; onSettings: () => void }) {
  return (
    <div className="flex shrink-0 items-center gap-3 rounded-lg border border-warning/30 bg-warning/5 px-3 py-2">
      <Globe className="h-4 w-4 shrink-0 text-warning" />
      <span className="flex-1 text-xs text-text-secondary">TMDb 搜索暂未启用</span>
      {reason && <span className="max-w-xs truncate text-xs text-text-tertiary">{reason}</span>}
      <Button variant="secondary" size="sm" icon={<Settings className="h-3 w-3" />} onClick={onSettings}>去设置</Button>
    </div>
  );
}

function TmdbInspector({
  selectedGroup,
  tmdbMediaType,
  tmdbQuery,
  tmdbLoading,
  tmdbError,
  tmdbDisabled,
  candidates,
  selectedCandidate,
  onQueryChange,
  onMediaTypeChange,
  onSearch,
  onCandidateSelect,
  onCandidateApply,
  onClearCandidates,
}: {
  selectedGroup: FolderGroup | null;
  tmdbMediaType: TmdbManualMediaType;
  tmdbQuery: string;
  tmdbLoading: boolean;
  tmdbError: string | null;
  tmdbDisabled: boolean;
  candidates: TmdbCandidate[];
  selectedCandidate: TmdbCandidate | null;
  onQueryChange: (value: string) => void;
  onMediaTypeChange: (value: TmdbManualMediaType) => void;
  onSearch: () => void;
  onCandidateSelect: (candidate: TmdbCandidate) => void;
  onCandidateApply: (candidate: TmdbCandidate) => void;
  onClearCandidates: () => void;
}) {
  if (!selectedGroup) {
    return (
      <Card className="p-4 text-center">
        <p className="text-xs text-text-tertiary">在左侧树中选择一个组查看详情</p>
      </Card>
    );
  }

  return (
    <>
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

      {candidates.length > 0 && (
        <CandidateList
          candidates={candidates}
          selectedCandidate={selectedCandidate}
          onSelect={onCandidateSelect}
          onApply={onCandidateApply}
          onClear={onClearCandidates}
        />
      )}

      {tmdbLoading && <LoadingCard />}
      {tmdbError && <ErrorCard message={tmdbError} />}
      {selectedCandidate && <ScrapePreviewCard selectedCandidate={selectedCandidate} selectedGroup={selectedGroup} />}
    </>
  );
}

function SearchTypeButton({ active, children, onClick }: { active: boolean; children: React.ReactNode; onClick: () => void }) {
  return (
    <button
      className={`rounded-lg px-3 py-2 text-xs font-medium transition-colors ${active ? 'bg-primary text-bg-primary' : 'text-text-secondary hover:text-text-primary'}`}
      onClick={onClick}
    >
      {children}
    </button>
  );
}

function CandidateList({
  candidates,
  selectedCandidate,
  onSelect,
  onApply,
  onClear,
}: {
  candidates: TmdbCandidate[];
  selectedCandidate: TmdbCandidate | null;
  onSelect: (candidate: TmdbCandidate) => void;
  onApply: (candidate: TmdbCandidate) => void;
  onClear: () => void;
}) {
  return (
    <Card className="overflow-hidden p-0">
      <div className="flex items-center justify-between border-b border-border bg-surface-subtle px-3 py-2">
        <h3 className="text-xs font-semibold text-text-primary">TMDb 候选 ({candidates.length})</h3>
        <Button variant="ghost" size="sm" icon={<X className="h-3 w-3" />} onClick={onClear}>{''}</Button>
      </div>
      <div className="max-h-80 divide-y divide-border overflow-y-auto">
        {candidates.map((candidate) => (
          <CandidateRow
            key={`${candidate.tmdb_id}:${candidate.title}`}
            candidate={candidate}
            selected={selectedCandidate?.tmdb_id === candidate.tmdb_id}
            onSelect={onSelect}
            onApply={onApply}
          />
        ))}
      </div>
    </Card>
  );
}

function CandidateRow({
  candidate,
  selected,
  onSelect,
  onApply,
}: {
  candidate: TmdbCandidate;
  selected: boolean;
  onSelect: (candidate: TmdbCandidate) => void;
  onApply: (candidate: TmdbCandidate) => void;
}) {
  return (
    <div
      className={`flex cursor-pointer gap-3 p-3 transition-colors hover:bg-surface-hover ${selected ? 'border-l-2 border-primary bg-primary/5' : ''}`}
      onClick={() => onSelect(candidate)}
    >
      <div className="h-20 w-12 shrink-0 overflow-hidden rounded bg-surface-hover">
        {candidate.poster_path ? (
          <img src={`${TMDB_IMAGE_BASE}${candidate.poster_path}`} alt={candidate.title} className="h-full w-full object-cover" loading="lazy" />
        ) : (
          <div className="flex h-full w-full items-center justify-center text-text-tertiary"><Film className="h-5 w-5" /></div>
        )}
      </div>
      <div className="min-w-0 flex-1">
        <div className="flex items-center gap-1.5">
          <span className="truncate text-xs font-medium text-text-primary">{candidate.title}</span>
          {candidate.year && <span className="shrink-0 text-xs text-text-tertiary">({candidate.year})</span>}
        </div>
        {candidate.original_title && candidate.original_title !== candidate.title && (
          <div className="truncate text-xs text-text-tertiary">{candidate.original_title}</div>
        )}
        <div className="mt-1 flex items-center gap-2">
          {candidate.vote_average != null && (
            <span className="flex items-center gap-0.5 text-xs text-warning"><Star className="h-3 w-3 fill-current" />{candidate.vote_average.toFixed(1)}</span>
          )}
          <Badge variant="default" size="sm">{isMovieCandidate(candidate) ? '🎬' : '📺'} TMDb {candidate.tmdb_id}</Badge>
        </div>
        {candidate.overview && <p className="mt-1 line-clamp-2 text-xs text-text-tertiary">{candidate.overview}</p>}
      </div>
      <div className="flex shrink-0 items-center">
        <Button variant="primary" size="sm" onClick={(event) => { event.stopPropagation(); onApply(candidate); }}>应用</Button>
      </div>
    </div>
  );
}

function LoadingCard() {
  return (
    <Card className="flex items-center justify-center gap-2 p-4">
      <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
      <span className="text-xs text-text-secondary">搜索中...</span>
    </Card>
  );
}

function ErrorCard({ message }: { message: string }) {
  return (
    <Card className="border-danger/30 bg-danger/10 p-3">
      <p className="text-xs text-danger">{message}</p>
    </Card>
  );
}

function ScrapePreviewCard({ selectedCandidate, selectedGroup }: { selectedCandidate: TmdbCandidate; selectedGroup: FolderGroup | null }) {
  const mainFile = selectedGroup?.children.find((child) => child.file_role === 'MainVideo') || selectedGroup?.children[0];

  return (
    <Card className="p-3">
      <div className="mb-2 flex items-center gap-1.5">
        <Image className="h-3.5 w-3.5 text-text-secondary" />
        <h3 className="text-xs font-semibold text-text-primary">刮削预览</h3>
      </div>
      <div className="space-y-2 text-xs text-text-secondary">
        <div><span className="text-text-tertiary">文件夹：</span>{selectedGroup?.target_folder_name}</div>
        <div><span className="text-text-tertiary">文件：</span>{mainFile?.target_name}</div>
        <div className="flex gap-2">
          <span className="text-text-tertiary">Poster：</span>
          {selectedCandidate.poster_path ? (
            <img src={`${TMDB_IMAGE_BASE}${selectedCandidate.poster_path}`} alt="Poster" className="h-24 w-16 rounded border border-border object-cover" />
          ) : (
            <span className="text-text-tertiary italic">无</span>
          )}
        </div>
        <div><span className="text-text-tertiary">标题：</span>{selectedCandidate.title} {selectedCandidate.year ? `(${selectedCandidate.year})` : ''}</div>
        <div><span className="text-text-tertiary">评分：</span>{selectedCandidate.vote_average != null ? `${selectedCandidate.vote_average.toFixed(1)} / 10` : '无'}</div>
        <div><span className="text-text-tertiary">简介：</span><span className="line-clamp-3">{selectedCandidate.overview || '无'}</span></div>
        <div><span className="text-text-tertiary">TMDb ID：</span>{selectedCandidate.tmdb_id}</div>
        <div><span className="text-text-tertiary">写入内容：</span><Badge variant="default" size="sm">NFO + 图片URL</Badge></div>
      </div>
      <ScrapeWriteActions candidate={selectedCandidate} group={selectedGroup} />
    </Card>
  );
}

function EditNameModal({
  mode,
  currentName,
  previewPath,
  onConfirm,
  onCancel,
}: {
  mode: 'file' | 'group';
  currentName: string;
  previewPath: string;
  onConfirm: (newName: string) => void;
  onCancel: () => void;
}) {
  const [value, setValue] = useState(currentName);

  const handleSubmit = useCallback(() => {
    const trimmed = value.trim();
    if (trimmed && trimmed !== currentName) onConfirm(trimmed);
    else onCancel();
  }, [value, currentName, onConfirm, onCancel]);

  const handleKeyDown = useCallback((event: React.KeyboardEvent) => {
    if (event.key === 'Enter') handleSubmit();
    if (event.key === 'Escape') onCancel();
  }, [handleSubmit, onCancel]);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70" onClick={onCancel}>
      <div className="w-[520px] max-w-[95vw] overflow-hidden rounded-2xl border border-accent/40 bg-bg-secondary p-0 shadow-2xl" onClick={(event) => event.stopPropagation()}>
        <div className="flex items-center justify-between border-b border-accent/20 bg-bg-card px-5 py-4">
          <div className="flex items-center gap-2">
            <Edit3 className="h-4 w-4 text-accent" />
            <h3 className="text-sm font-semibold text-text-primary">{mode === 'group' ? '编辑文件夹名' : '编辑文件名'}</h3>
          </div>
          <Button variant="ghost" size="sm" icon={<X className="h-4 w-4" />} onClick={onCancel}>{''}</Button>
        </div>
        <div className="space-y-4 p-5">
          <div>
            <label className="mb-1 block text-xs text-text-secondary">原名称</label>
            <div className="break-all rounded-xl border border-text-secondary/20 bg-bg-primary px-3 py-2 text-sm text-text-primary">{currentName}</div>
          </div>
          <div>
            <label className="mb-1 block text-xs text-text-secondary">新名称</label>
            <input
              type="text"
              className="w-full rounded-xl border border-accent/50 bg-bg-primary px-3 py-2 text-sm text-text-primary transition-colors focus:border-accent focus:outline-none"
              value={value}
              onChange={(event) => setValue(event.target.value)}
              onKeyDown={handleKeyDown}
              autoFocus
            />
          </div>
          <div>
            <label className="mb-1 block text-xs text-text-secondary">最终路径预览</label>
            <div className="break-all rounded-xl border border-text-secondary/20 bg-bg-primary px-3 py-2 font-mono text-xs text-text-secondary">
              {previewPath.replace(currentName, value.trim() || currentName)}
            </div>
          </div>
        </div>
        <div className="flex justify-end gap-2 border-t border-accent/20 bg-bg-card px-5 py-4">
          <Button variant="secondary" size="sm" onClick={onCancel}>取消</Button>
          <Button variant="primary" size="sm" icon={<Check className="h-4 w-4" />} onClick={handleSubmit}>保存</Button>
        </div>
      </div>
    </div>
  );
}
