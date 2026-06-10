import { useState, useMemo, useCallback, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  ArrowLeft,
  Globe,
  Play,
  Settings,
  X,
  Folder,
  Check,
  AlertTriangle,
  Star,
  Edit3,
  Image,
  Film,
  Info,
} from 'lucide-react';
import { Button, Card, Badge } from '../../components/ui';
import { usePipelineStore } from '../../state/pipelineStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import { useExecutionStore } from '../../state/executionStore';
import { PreviewPlanTree } from './tree/PreviewPlanTree';
import { NamingPresetList } from './naming/NamingPresetList';
import { NamingTokenOrderList } from './naming/NamingTokenOrderList';
import { FolderPolicySelector } from './folder-policy/FolderPolicySelector';
import { ExecutionConfirmPanel } from './ExecutionConfirmPanel';
import { ExecutionProgressPanel } from './ExecutionProgressPanel';
import { ExecutionResultPanel } from './ExecutionResultPanel';
import type {
  NamingRule,
  TitleStrategy,
  FolderPolicy,
  FolderPolicyConfig,
  FolderGroup,
  TmdbCandidate,
  SearchTmdbCandidatesInput,
} from '../../types';
import { buildPreviewGroups } from './model/buildPreviewGroups';

const PRESETS: Array<{ id: string; label: string; desc: string }> = [
  { id: 'clean-library', label: '清爽媒体库', desc: '中文标题 (年份).mkv' },
  { id: 'bilingual-library', label: '中英双语', desc: '中文 - 英文 (年份).mkv' },
  { id: 'pt-preserve', label: 'PT原样保留', desc: 'First.Blood.1982.1080p.BluRay.mkv' },
  { id: 'chinese-prefix-pt', label: '中文前缀+PT', desc: '中文.PT技术信息.mkv' },
  { id: 'bt-friendly', label: 'BT友好', desc: '中文.英文.年份.分辨率.mkv' },
  { id: 'jellyfin-emby', label: 'Jellyfin/Emby', desc: '中文标题 (年份).mkv' },
  { id: 'tmdb-id-friendly', label: 'TMDb ID友好', desc: '中文 (年份) [tmdb-xxx].mkv' },
];

const STRATEGIES: Array<{ id: TitleStrategy; label: string }> = [
  { id: 'ChineseOnly', label: '仅中文' },
  { id: 'EnglishOnly', label: '仅英文' },
  { id: 'Bilingual', label: '中英双语' },
  { id: 'ChinesePrefixPt', label: '中文前缀+PT' },
  { id: 'ChineseFolderPtFile', label: '中文文件夹+PT文件' },
  { id: 'ChineseFolderChinesePrefixPtFile', label: '中文文件夹+中文前缀PT' },
];

const TMDB_IMAGE_BASE = 'https://image.tmdb.org/t/p/w185';
type TmdbManualMediaType = SearchTmdbCandidatesInput['media_type'];

function buildCleanLibraryRule(): NamingRule {
  return {
    name: 'clean-library',
    description: '清爽媒体库',
    tokens: [
      { token: 'ZhTitle', prefix: '', suffix: '', separator: 'Space', empty_policy: 'Hide', case_strategy: 'AsIs', wrapper: 'None', enabled: true },
      { token: 'Year', prefix: '', suffix: '', separator: 'Space', empty_policy: 'Hide', case_strategy: 'AsIs', wrapper: 'Parentheses', enabled: true },
      { token: 'Ext', prefix: '', suffix: '', separator: 'None', empty_policy: 'Hide', case_strategy: 'AsIs', wrapper: 'None', enabled: true },
    ],
  };
}

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
  const [currentNamingRule, setCurrentNamingRule] = useState<NamingRule>(buildCleanLibraryRule());
  const [currentStrategy, setCurrentStrategy] = useState<TitleStrategy>('ChineseOnly');
  const [selectedPreset, setSelectedPreset] = useState('clean-library');
  const [folderPolicyConfig, setFolderPolicyConfig] = useState<FolderPolicyConfig>({
    policy: 'KeepOriginalStructure',
    clean_empty_folders_after: false,
  });
  const [editModal, setEditModal] = useState<{
    open: boolean;
    mode: 'file' | 'group';
    targetId: string;
    currentName: string;
    previewPath: string;
  } | null>(null);
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

  const groups: FolderGroup[] = planTree?.groups ?? [];
  const selectedGroup = groups.find((g) => g.id === selectedGroupId) ?? null;

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

  const triggerNamingApply = useCallback((rule: NamingRule, strategy: TitleStrategy) => {
    if (namingDebounceRef.current) clearTimeout(namingDebounceRef.current);
    namingDebounceRef.current = setTimeout(() => {
      applyNamingRule(rule, strategy);
    }, 150);
  }, [applyNamingRule]);

  useEffect(() => {
    return () => {
      if (namingDebounceRef.current) clearTimeout(namingDebounceRef.current);
    };
  }, []);

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

    setTmdbLoading(true);
    try {
      const query = queryText.trim() || buildTmdbQuery(group);
      if (!query) {
        setTmdbError('请输入 TMDb 搜索词');
        return;
      }

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
    try {
      const mainVideo = selectedGroup.children.find((c) => c.file_role === 'MainVideo');
      const targetId = mainVideo?.id || selectedGroup.children[0]?.id;
      if (!targetId) return;
      await applyTmdbCandidate(targetId, candidate);
      refreshSafetySummary();
    } catch (err) {
      console.error('Failed to apply candidate:', err);
    }
  }, [selectedGroup, applyTmdbCandidate, refreshSafetySummary]);

  const openGroupEdit = useCallback((groupId: string) => {
    const group = groups.find((g) => g.id === groupId);
    if (!group || group.children.length === 0) return;
    const mainFile = group.children.find((c) => c.file_role === 'MainVideo') || group.children[0];
    setEditModal({
      open: true,
      mode: 'group',
      targetId: mainFile.id,
      currentName: group.target_folder_name,
      previewPath: group.target_path,
    });
  }, [groups]);

  const openFileEdit = useCallback((fileId: string) => {
    const item = pipelineResult?.previews.find((p) => p.id === fileId);
    if (!item) return;
    setEditModal({
      open: true,
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
    const group = groups.find((g) => g.id === groupId);
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
        <p className="text-sm text-text-secondary mb-4">暂无预览结果，请先扫描文件</p>
        <Button onClick={() => navigate('/scan')}>去扫描</Button>
      </Card>
    );
  }

  return (
    <div className="flex flex-col gap-3 h-[calc(100vh-6rem)]">
      <div className="flex items-center justify-between shrink-0">
        <div className="flex items-center gap-3">
          <Button variant="ghost" size="sm" icon={<ArrowLeft className="h-4 w-4" />} onClick={() => navigate('/scan')}>
            返回
          </Button>
          <h2 className="text-lg font-semibold text-text-primary">预览重命名</h2>
          <Badge variant="default" size="sm">{groups.length} 组 · {pipelineResult.previews.length} 文件</Badge>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="secondary" size="sm" icon={<Settings className="h-4 w-4" />} onClick={() => navigate('/settings')}>
            设置
          </Button>
          <Button variant="secondary" size="sm" icon={<Play className="h-4 w-4" />} onClick={() => startExecution('DryRun')}>
            模拟执行
          </Button>
          <Button variant="primary" size="sm" icon={<Play className="h-4 w-4" />} onClick={() => startExecution('Confirmed')}>
            执行重命名
          </Button>
        </div>
      </div>

      {tmdbSearchStatus === 'disabled' && (
        <div className="flex items-center gap-3 rounded-lg border border-warning/30 bg-warning/5 px-3 py-2 shrink-0">
          <Globe className="h-4 w-4 shrink-0 text-warning" />
          <span className="text-xs text-text-secondary flex-1">TMDb 搜索暂未启用</span>
          {disabledReason && <span className="text-xs text-text-tertiary truncate max-w-xs">{disabledReason}</span>}
          <Button variant="secondary" size="sm" icon={<Settings className="h-3 w-3" />} onClick={() => navigate('/settings')}>
            去设置
          </Button>
        </div>
      )}

      <div className="flex-1 flex gap-3 min-h-0">
        <div className="w-[260px] shrink-0 flex flex-col gap-3 overflow-y-auto">
          <WorkbenchCard title="命名预设" icon={<Settings className="h-3.5 w-3.5 text-text-secondary" />}>
            <NamingPresetList
              presets={PRESETS}
              selectedId={selectedPreset}
              onSelect={(id) => {
                setSelectedPreset(id);
                const rule = buildCleanLibraryRule();
                rule.name = id;
                const preset = PRESETS.find((p) => p.id === id);
                if (preset) {
                  rule.description = preset.desc;
                  setCurrentNamingRule(rule);
                  triggerNamingApply(rule, currentStrategy);
                }
              }}
            />
          </WorkbenchCard>

          <WorkbenchCard title="文件夹策略" icon={<Folder className="h-3.5 w-3.5 text-text-secondary" />}>
            <FolderPolicySelector
              selected={folderPolicyConfig.policy}
              onSelect={(policy: FolderPolicy) => handleFolderPolicyChange({ ...folderPolicyConfig, policy })}
            />
            {folderPolicyConfig.policy === 'Flatten' && (
              <div className="mt-2 flex items-start gap-1.5 rounded-lg border border-error/30 bg-error/5 px-2 py-1.5">
                <AlertTriangle className="h-3 w-3 text-error shrink-0 mt-0.5" />
                <span className="text-xs text-error leading-relaxed">高风险：将文件移至上级目录，同名文件会冲突。</span>
              </div>
            )}
            <label className="flex items-center gap-1.5 mt-2 text-xs text-text-secondary cursor-pointer">
              <input
                type="checkbox"
                className="rounded border-border"
                checked={folderPolicyConfig.clean_empty_folders_after}
                onChange={(e) => handleFolderPolicyChange({ ...folderPolicyConfig, clean_empty_folders_after: e.target.checked })}
              />
              清理空文件夹
            </label>
          </WorkbenchCard>

          <WorkbenchCard title="标题策略" icon={<Info className="h-3.5 w-3.5 text-text-secondary" />}>
            <div className="flex flex-wrap gap-1">
              {STRATEGIES.map((s) => (
                <button
                  key={s.id}
                  className={`text-xs px-2 py-1 rounded-full transition-colors ${currentStrategy === s.id ? 'bg-primary text-white' : 'bg-surface-hover text-text-secondary hover:text-text-primary'}`}
                  onClick={() => {
                    setCurrentStrategy(s.id);
                    triggerNamingApply(currentNamingRule, s.id);
                  }}
                >
                  {s.label}
                </button>
              ))}
            </div>
          </WorkbenchCard>

          <WorkbenchCard title="Token 排序" icon={<Edit3 className="h-3.5 w-3.5 text-text-secondary" />}>
            <NamingTokenOrderList
              tokens={currentNamingRule.tokens}
              onReorder={(tokens) => {
                const newRule = { ...currentNamingRule, tokens };
                setCurrentNamingRule(newRule);
                triggerNamingApply(newRule, currentStrategy);
              }}
              onRemove={(idx) => {
                const newTokens = currentNamingRule.tokens.filter((_, i) => i !== idx);
                const newRule = { ...currentNamingRule, tokens: newTokens };
                setCurrentNamingRule(newRule);
                triggerNamingApply(newRule, currentStrategy);
              }}
            />
          </WorkbenchCard>
        </div>

        <div className="flex-1 min-w-0 flex flex-col min-h-0">
          <Card className="flex-1 min-h-0 overflow-y-auto p-0">
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

        <div className="w-[320px] shrink-0 flex flex-col gap-3 overflow-y-auto">
          {selectedGroup ? (
            <Card className="p-3">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-sm">{selectedGroup.media_type === 'Movie' ? '🎬' : selectedGroup.media_type === 'Tv' ? '📺' : '📁'}</span>
                <h3 className="text-sm font-semibold text-text-primary truncate">{selectedGroup.target_folder_name}</h3>
                <Badge variant="default" size="sm">{selectedGroup.media_type} · {selectedGroup.file_count} 文件</Badge>
              </div>
              <div className="text-xs text-text-tertiary space-y-1">
                <div>原路径：{selectedGroup.original_path}</div>
                <div>新路径：{selectedGroup.target_path}</div>
              </div>

              <div className="mt-3 pt-3 border-t border-border space-y-3">
                <div>
                  <div className="mb-1 text-xs text-text-secondary">搜索词</div>
                  <input
                    className="w-full rounded-xl border border-border bg-bg-primary px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-primary"
                    value={tmdbQuery}
                    onChange={(e) => setTmdbQuery(e.target.value)}
                    onKeyDown={(event) => {
                      if (event.key === 'Enter') {
                        handleTmdbSearch();
                      }
                    }}
                    placeholder="可改成英文名，例如 First Blood"
                  />
                </div>
                <div>
                  <div className="mb-1 text-xs text-text-secondary">搜索类型</div>
                  <div className="grid grid-cols-2 gap-2 rounded-xl bg-bg-primary p-1">
                    <button
                      className={`rounded-lg px-3 py-2 text-xs font-medium transition-colors ${tmdbMediaType === 'movie' ? 'bg-primary text-bg-primary' : 'text-text-secondary hover:text-text-primary'}`}
                      onClick={() => setTmdbMediaType('movie')}
                    >
                      电影
                    </button>
                    <button
                      className={`rounded-lg px-3 py-2 text-xs font-medium transition-colors ${tmdbMediaType === 'tv' ? 'bg-primary text-bg-primary' : 'text-text-secondary hover:text-text-primary'}`}
                      onClick={() => setTmdbMediaType('tv')}
                    >
                      电视剧
                    </button>
                  </div>
                </div>
                <Button
                  variant="primary"
                  size="sm"
                  icon={<Globe className="h-3.5 w-3.5" />}
                  onClick={() => handleTmdbSearch()}
                  isLoading={tmdbLoading}
                  disabled={tmdbSearchStatus === 'disabled' || !tmdbQuery.trim()}
                  className="w-full"
                >
                  {tmdbSearchStatus === 'disabled' ? '先启用 TMDb' : '搜索 TMDb'}
                </Button>
              </div>
            </Card>
          ) : (
            <Card className="p-4 text-center">
              <p className="text-xs text-text-tertiary">在左侧树中选择一个组查看详情</p>
            </Card>
          )}

          {tmdbCandidates.length > 0 && (
            <Card className="p-0 overflow-hidden">
              <div className="flex items-center justify-between px-3 py-2 border-b border-border bg-surface-subtle">
                <h3 className="text-xs font-semibold text-text-primary">TMDb 候选 ({tmdbCandidates.length})</h3>
                <Button variant="ghost" size="sm" icon={<X className="h-3 w-3" />} onClick={() => { setTmdbCandidates([]); setSelectedCandidate(null); }}>{''}</Button>
              </div>
              <div className="max-h-80 overflow-y-auto divide-y divide-border">
                {tmdbCandidates.map((candidate) => (
                  <CandidateRow
                    key={candidate.tmdb_id}
                    candidate={candidate}
                    selected={selectedCandidate?.tmdb_id === candidate.tmdb_id}
                    onSelect={setSelectedCandidate}
                    onApply={handleApplyCandidate}
                  />
                ))}
              </div>
            </Card>
          )}

          {tmdbLoading && (
            <Card className="p-4 flex items-center justify-center gap-2">
              <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
              <span className="text-xs text-text-secondary">搜索中...</span>
            </Card>
          )}
          {tmdbError && (
            <Card className="p-3 border-danger/30 bg-danger/10">
              <p className="text-xs text-danger">{tmdbError}</p>
            </Card>
          )}

          {selectedCandidate && (
            <ScrapePreviewCard selectedCandidate={selectedCandidate} selectedGroup={selectedGroup} />
          )}
        </div>
      </div>

      {editModal?.open && (
        <EditNameModal mode={editModal.mode} currentName={editModal.currentName} previewPath={editModal.previewPath} onConfirm={confirmEdit} onCancel={() => setEditModal(null)} />
      )}

      {uiState === 'confirming' && confirmState && (
        <div className="shrink-0">
          <ExecutionConfirmPanel state={confirmState} onConfirm={() => confirmExecution()} onCancel={cancelExecution} isLoading={false} />
        </div>
      )}
      {uiState === 'executing' && progressState && (
        <div className="shrink-0"><ExecutionProgressPanel state={progressState} /></div>
      )}
      {uiState === 'completed' && resultState && (
        <div className="shrink-0">
          <ExecutionResultPanel state={resultState} onRollback={() => {}} onExport={() => {}} onReset={resetExecution} />
        </div>
      )}
    </div>
  );
}

function WorkbenchCard({ title, icon, children }: { title: string; icon: React.ReactNode; children: React.ReactNode }) {
  return (
    <div className="rounded-xl border border-border bg-surface p-3">
      <div className="flex items-center gap-1.5 mb-2">
        {icon}
        <h3 className="text-xs font-semibold text-text-primary">{title}</h3>
      </div>
      {children}
    </div>
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
      className={`flex gap-3 p-3 cursor-pointer hover:bg-surface-hover transition-colors ${selected ? 'bg-primary/5 border-l-2 border-primary' : ''}`}
      onClick={() => onSelect(candidate)}
    >
      <div className="w-12 h-20 shrink-0 rounded bg-surface-hover overflow-hidden">
        {candidate.poster_path ? (
          <img src={`${TMDB_IMAGE_BASE}${candidate.poster_path}`} alt={candidate.title} className="w-full h-full object-cover" loading="lazy" />
        ) : (
          <div className="w-full h-full flex items-center justify-center text-text-tertiary"><Film className="h-5 w-5" /></div>
        )}
      </div>
      <div className="min-w-0 flex-1">
        <div className="flex items-center gap-1.5">
          <span className="text-xs font-medium text-text-primary truncate">{candidate.title}</span>
          {candidate.year && <span className="text-xs text-text-tertiary shrink-0">({candidate.year})</span>}
        </div>
        {candidate.original_title && candidate.original_title !== candidate.title && <div className="text-xs text-text-tertiary truncate">{candidate.original_title}</div>}
        <div className="flex items-center gap-2 mt-1">
          {candidate.vote_average != null && <span className="flex items-center gap-0.5 text-xs text-warning"><Star className="h-3 w-3 fill-current" />{candidate.vote_average.toFixed(1)}</span>}
          <Badge variant="default" size="sm">{candidate.media_type === 'Movie' ? '🎬' : '📺'} TMDb {candidate.tmdb_id}</Badge>
        </div>
        {candidate.overview && <p className="text-xs text-text-tertiary mt-1 line-clamp-2">{candidate.overview}</p>}
      </div>
      <div className="shrink-0 flex items-center">
        <Button variant="primary" size="sm" onClick={(event) => { event.stopPropagation(); onApply(candidate); }}>应用</Button>
      </div>
    </div>
  );
}

function ScrapePreviewCard({ selectedCandidate, selectedGroup }: { selectedCandidate: TmdbCandidate; selectedGroup: FolderGroup | null }) {
  return (
    <Card className="p-3">
      <div className="flex items-center gap-1.5 mb-2">
        <Image className="h-3.5 w-3.5 text-text-secondary" />
        <h3 className="text-xs font-semibold text-text-primary">刮削预览</h3>
      </div>
      <div className="space-y-2 text-xs text-text-secondary">
        <div><span className="text-text-tertiary">文件夹：</span>{selectedGroup?.target_folder_name}</div>
        <div><span className="text-text-tertiary">文件：</span>{selectedGroup?.children.find((child) => child.file_role === 'MainVideo')?.target_name || selectedGroup?.children[0]?.target_name}</div>
        <div className="flex gap-2">
          <span className="text-text-tertiary">Poster：</span>
          {selectedCandidate.poster_path ? (
            <img src={`${TMDB_IMAGE_BASE}${selectedCandidate.poster_path}`} alt="Poster" className="w-16 h-24 rounded object-cover border border-border" />
          ) : (
            <span className="text-text-tertiary italic">无</span>
          )}
        </div>
        <div><span className="text-text-tertiary">标题：</span>{selectedCandidate.title} {selectedCandidate.year ? `(${selectedCandidate.year})` : ''}</div>
        <div><span className="text-text-tertiary">评分：</span>{selectedCandidate.vote_average != null ? `${selectedCandidate.vote_average.toFixed(1)} / 10` : '无'}</div>
        <div><span className="text-text-tertiary">简介：</span><span className="line-clamp-3">{selectedCandidate.overview || '无'}</span></div>
        <div><span className="text-text-tertiary">TMDb ID：</span>{selectedCandidate.tmdb_id}</div>
        <div><span className="text-text-tertiary">写入模式：</span><Badge variant="default" size="sm">仅命名</Badge></div>
      </div>
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
      <div className="bg-bg-secondary rounded-2xl border border-accent/40 shadow-2xl w-[520px] max-w-[95vw] p-0 overflow-hidden" onClick={(event) => event.stopPropagation()}>
        <div className="flex items-center justify-between px-5 py-4 border-b border-accent/20 bg-bg-card">
          <div className="flex items-center gap-2">
            <Edit3 className="h-4 w-4 text-accent" />
            <h3 className="text-sm font-semibold text-text-primary">{mode === 'group' ? '编辑文件夹名' : '编辑文件名'}</h3>
          </div>
          <Button variant="ghost" size="sm" icon={<X className="h-4 w-4" />} onClick={onCancel}>{''}</Button>
        </div>

        <div className="p-5 space-y-4">
          <div>
            <label className="text-xs text-text-secondary mb-1 block">原名称</label>
            <div className="text-sm text-text-primary bg-bg-primary rounded-xl px-3 py-2 break-all border border-text-secondary/20">{currentName}</div>
          </div>
          <div>
            <label className="text-xs text-text-secondary mb-1 block">新名称</label>
            <input
              type="text"
              className="w-full rounded-xl border border-accent/50 bg-bg-primary px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent transition-colors"
              value={value}
              onChange={(event) => setValue(event.target.value)}
              onKeyDown={handleKeyDown}
              autoFocus
            />
          </div>
          <div>
            <label className="text-xs text-text-secondary mb-1 block">最终路径预览</label>
            <div className="text-xs text-text-secondary bg-bg-primary rounded-xl px-3 py-2 break-all font-mono border border-text-secondary/20">
              {previewPath.replace(currentName, value.trim() || currentName)}
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-2 px-5 py-4 border-t border-accent/20 bg-bg-card">
          <Button variant="secondary" size="sm" onClick={onCancel}>取消</Button>
          <Button variant="primary" size="sm" icon={<Check className="h-4 w-4" />} onClick={handleSubmit}>保存</Button>
        </div>
      </div>
    </div>
  );
}
