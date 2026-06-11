import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card } from '../../components/ui';
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
import { EditNameModal } from './edit';
import { PreviewExecutionPanels } from './execution';
import { PreviewHeader } from './header';
import { buildPreviewGroups } from './model/buildPreviewGroups';
import { buildInitialNamingWorkbenchState, PreviewNamingWorkbenchPanel } from './naming/workbench';
import { PreviewEmptyState, PreviewLoadingState } from './state';
import { TmdbDisabledBanner, TmdbInspector } from './tmdb';
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
    return <PreviewLoadingState />;
  }

  if (!pipelineResult) {
    return <PreviewEmptyState onScan={() => navigate('/scan')} />;
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

      <PreviewExecutionPanels
        uiState={uiState}
        confirmState={confirmState}
        progressState={progressState}
        resultState={resultState}
        onConfirm={confirmExecution}
        onCancel={cancelExecution}
        onReset={resetExecution}
      />
    </div>
  );
}

type EditModalState = {
  mode: 'file' | 'group';
  targetId: string;
  currentName: string;
  previewPath: string;
};
