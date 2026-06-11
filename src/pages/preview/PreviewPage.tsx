import type * as React from 'react';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  ArrowLeft,
  Check,
  Edit3,
  Globe,
  Play,
  Settings,
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
  TitleStrategy,
  TmdbCandidate,
} from '../../types';
import { ExecutionConfirmPanel } from './ExecutionConfirmPanel';
import { ExecutionProgressPanel } from './ExecutionProgressPanel';
import { ExecutionResultPanel } from './ExecutionResultPanel';
import { buildPreviewGroups } from './model/buildPreviewGroups';
import { buildInitialNamingWorkbenchState, PreviewNamingWorkbenchPanel } from './naming/workbench';
import { CandidateList, ScrapePreviewCard, TmdbErrorCard, TmdbLoadingCard, TmdbSearchCard } from './tmdb';
import type { TmdbManualMediaType } from './tmdb';
import { PreviewPlanTree } from './tree/PreviewPlanTree';

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
      <TmdbSearchCard
        selectedGroup={selectedGroup}
        tmdbMediaType={tmdbMediaType}
        tmdbQuery={tmdbQuery}
        tmdbLoading={tmdbLoading}
        tmdbDisabled={tmdbDisabled}
        onQueryChange={onQueryChange}
        onMediaTypeChange={onMediaTypeChange}
        onSearch={onSearch}
      />

      {candidates.length > 0 && (
        <CandidateList
          candidates={candidates}
          selectedCandidate={selectedCandidate}
          onSelect={onCandidateSelect}
          onApply={onCandidateApply}
          onClear={onClearCandidates}
        />
      )}

      {tmdbLoading && <TmdbLoadingCard />}
      {tmdbError && <TmdbErrorCard message={tmdbError} />}
      {selectedCandidate && <ScrapePreviewCard selectedCandidate={selectedCandidate} selectedGroup={selectedGroup} />}
    </>
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
