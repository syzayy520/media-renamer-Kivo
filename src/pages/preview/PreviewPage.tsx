import { useState, useMemo, useCallback, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  ArrowLeft, Globe, Play, Settings, X, Folder, Check, AlertTriangle,
  Star, Edit3, Image, Film, Info,
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
  NamingRule, TitleStrategy, FolderPolicy, FolderPolicyConfig,
  FolderGroup, TmdbCandidate,
} from '../../types';
import { buildPreviewGroups } from './model/buildPreviewGroups';

// ─── Presets & Strategies (same definitions as NamingRulePanel) ──────────────

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

function buildCleanLibraryRule(): NamingRule {
  return {
    name: 'clean-library', description: '清爽媒体库',
    tokens: [
      { token: 'ZhTitle', prefix: '', suffix: '', separator: 'Space', empty_policy: 'Hide', case_strategy: 'AsIs', wrapper: 'None', enabled: true },
      { token: 'Year', prefix: '', suffix: '', separator: 'Space', empty_policy: 'Hide', case_strategy: 'AsIs', wrapper: 'Parentheses', enabled: true },
      { token: 'Ext', prefix: '', suffix: '', separator: 'None', empty_policy: 'Hide', case_strategy: 'AsIs', wrapper: 'None', enabled: true },
    ],
  };
}

const TMDB_IMAGE_BASE = 'https://image.tmdb.org/t/p/w185';

// ─── Main Page ─────────────────────────────────────────────────────────────────

export function PreviewPage() {
  const navigate = useNavigate();
  const {
    pipelineResult, updatePreviewProposedName, togglePreviewSkipped,
    refreshSafetySummary, applyNamingRule, applyFolderPolicy, applyTmdbCandidate,
  } = usePipelineStore();
  const { isLoading } = useUiFeedbackStore();
  const { tmdbSearchStatus, disabledReason, searchTmdbCandidates } = useTmdbSearchStore();
  const {
    uiState, confirmState, progressState, resultState,
    startExecution, confirmExecution, cancelExecution, resetExecution,
  } = useExecutionStore();

  // ── Local state ──────────────────────────────────────────────────────────
  const [expandedGroups, setExpandedGroups] = useState<Set<string>>(new Set());
  const [selectedGroupId, setSelectedGroupId] = useState<string | null>(null);
  const [currentNamingRule, setCurrentNamingRule] = useState<NamingRule>(buildCleanLibraryRule());
  const [currentStrategy, setCurrentStrategy] = useState<TitleStrategy>('ChineseOnly');
  const [selectedPreset, setSelectedPreset] = useState('clean-library');
  const [folderPolicyConfig, setFolderPolicyConfig] = useState<FolderPolicyConfig>({
    policy: 'KeepOriginalStructure',
    clean_empty_folders_after: false,
  });

  // Edit modal
  const [editModal, setEditModal] = useState<{ open: boolean; mode: 'file' | 'group'; targetId: string; currentName: string; previewPath: string } | null>(null);

  // TMDb
  const [tmdbCandidates, setTmdbCandidates] = useState<TmdbCandidate[]>([]);
  const [tmdbLoading, setTmdbLoading] = useState(false);
  const [tmdbError, setTmdbError] = useState<string | null>(null);
  const [selectedCandidate, setSelectedCandidate] = useState<TmdbCandidate | null>(null);

  // Debounce ref for naming rule changes
  const namingDebounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // ── Derived ──────────────────────────────────────────────────────────────
  const scanRoot = pipelineResult?.scan?.scan_path;
  const planTree = useMemo(() => {
    if (!pipelineResult) return null;
    return buildPreviewGroups(pipelineResult.previews, scanRoot);
  }, [pipelineResult, scanRoot]);

  const groups: FolderGroup[] = planTree?.groups ?? [];
  const selectedGroup = groups.find((g) => g.id === selectedGroupId) ?? null;

  // ── Naming Rule change → backend ─────────────────────────────────────────
  const triggerNamingApply = useCallback((rule: NamingRule, strategy: TitleStrategy) => {
    if (namingDebounceRef.current) clearTimeout(namingDebounceRef.current);
    namingDebounceRef.current = setTimeout(() => {
      applyNamingRule(rule, strategy);
    }, 150);
  }, [applyNamingRule]);

  useEffect(() => {
    return () => { if (namingDebounceRef.current) clearTimeout(namingDebounceRef.current); };
  }, []);

  // ── Folder Policy change → backend ───────────────────────────────────────
  const handleFolderPolicyChange = useCallback((config: FolderPolicyConfig) => {
    setFolderPolicyConfig(config);
    applyFolderPolicy(config.policy);
  }, [applyFolderPolicy]);

  // ── Tree actions ─────────────────────────────────────────────────────────
  const handleToggleExpand = useCallback((groupId: string) => {
    setExpandedGroups((prev) => {
      const next = new Set(prev);
      if (next.has(groupId)) next.delete(groupId);
      else next.add(groupId);
      return next;
    });
    setSelectedGroupId(groupId);
  }, []);

  const handleSelectGroup = useCallback((groupId: string) => {
    setSelectedGroupId(groupId);
  }, []);

  // ── TMDb search ──────────────────────────────────────────────────────────
  const handleTmdbSearch = useCallback(async () => {
    if (!selectedGroup) return;
    setTmdbLoading(true);
    setTmdbError(null);
    setTmdbCandidates([]);
    setSelectedCandidate(null);
    try {
      const firstChild = selectedGroup.children[0];
      const rawName = firstChild?.target_name || selectedGroup.target_folder_name;
      const searchName = rawName.replace(/\.[^.]+$/, '');
      const result = await searchTmdbCandidates(
        searchName,
        selectedGroup.media_type === 'Tv' ? 'Series' : 'Movie',
      );
      if (result) setTmdbCandidates(result);
    } catch (err) {
      setTmdbError(err instanceof Error ? err.message : '搜索失败');
    } finally {
      setTmdbLoading(false);
    }
  }, [selectedGroup, searchTmdbCandidates]);

  const handleApplyCandidate = useCallback(async (candidate: TmdbCandidate) => {
    if (!selectedGroup) return;
    try {
      // Apply to the main video file in the group
      const mainVideo = selectedGroup.children.find((c) => c.file_role === 'MainVideo');
      const targetId = mainVideo?.id || selectedGroup.children[0]?.id;
      if (!targetId) return;

      await applyTmdbCandidate(targetId, candidate);
      refreshSafetySummary();
    } catch (err) {
      console.error('Failed to apply candidate:', err);
    }
  }, [selectedGroup, applyTmdbCandidate, refreshSafetySummary]);

  // ── Edit modal ───────────────────────────────────────────────────────────
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

  // ── Render ───────────────────────────────────────────────────────────────
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
      {/* ═══ Header ═══ */}
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

      {/* TMDb Disabled Banner */}
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

      {/* ═══ Three-Column Body ═══ */}
      <div className="flex-1 flex gap-3 min-h-0">
        {/* --- Left Sidebar (260px) --- */}
        <div className="w-[260px] shrink-0 flex flex-col gap-3 overflow-y-auto">
          {/* Naming Presets */}
          <div className="rounded-xl border border-border bg-surface p-3">
            <div className="flex items-center gap-1.5 mb-2">
              <Settings className="h-3.5 w-3.5 text-text-secondary" />
              <h3 className="text-xs font-semibold text-text-primary">命名预设</h3>
            </div>
            <NamingPresetList
              presets={PRESETS}
              selectedId={selectedPreset}
              onSelect={(id) => {
                setSelectedPreset(id);
                // Apply preset's NamingRule
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
          </div>

          {/* Folder Policy */}
          <div className="rounded-xl border border-border bg-surface p-3">
            <div className="flex items-center gap-1.5 mb-2">
              <Folder className="h-3.5 w-3.5 text-text-secondary" />
              <h3 className="text-xs font-semibold text-text-primary">文件夹策略</h3>
            </div>
            <FolderPolicySelector
              selected={folderPolicyConfig.policy}
              onSelect={(policy: FolderPolicy) => handleFolderPolicyChange({ ...folderPolicyConfig, policy })}
            />
            {folderPolicyConfig.policy === 'Flatten' && (
              <div className="mt-2 flex items-start gap-1.5 rounded-lg border border-error/30 bg-error/5 px-2 py-1.5">
                <AlertTriangle className="h-3 w-3 text-error shrink-0 mt-0.5" />
                <span className="text-xs text-error leading-relaxed">
                  高风险：将文件移至上级目录，同名文件会冲突。
                </span>
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
          </div>

          {/* Title Strategy */}
          <div className="rounded-xl border border-border bg-surface p-3">
            <div className="flex items-center gap-1.5 mb-2">
              <Info className="h-3.5 w-3.5 text-text-secondary" />
              <h3 className="text-xs font-semibold text-text-primary">标题策略</h3>
            </div>
            <div className="flex flex-wrap gap-1">
              {STRATEGIES.map((s) => (
                <button
                  key={s.id}
                  className={`text-xs px-2 py-1 rounded-full transition-colors ${
                    currentStrategy === s.id
                      ? 'bg-primary text-white'
                      : 'bg-surface-hover text-text-secondary hover:text-text-primary'
                  }`}
                  onClick={() => {
                    setCurrentStrategy(s.id);
                    triggerNamingApply(currentNamingRule, s.id);
                  }}
                >
                  {s.label}
                </button>
              ))}
            </div>
          </div>

          {/* Token Order */}
          <div className="rounded-xl border border-border bg-surface p-3">
            <div className="flex items-center gap-1.5 mb-2">
              <Edit3 className="h-3.5 w-3.5 text-text-secondary" />
              <h3 className="text-xs font-semibold text-text-primary">Token 排序</h3>
            </div>
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
          </div>
        </div>

        {/* --- Center: Folder Tree (flex-1) --- */}
        <div className="flex-1 min-w-0 flex flex-col min-h-0">
          <Card className="flex-1 min-h-0 overflow-y-auto p-0">
            <PreviewPlanTree
              groups={groups}
              expandedGroups={expandedGroups}
              onGroupToggleExpand={handleToggleExpand}
              onGroupTmdbSearch={handleSelectGroup}
              onGroupEdit={openGroupEdit}
              onGroupSkip={handleGroupSkip}
              onFileEdit={openFileEdit}
              onFileSkip={togglePreviewSkipped}
            />
          </Card>
        </div>

        {/* --- Right Inspector (320px) --- */}
        <div className="w-[320px] shrink-0 flex flex-col gap-3 overflow-y-auto">
          {/* Group Info */}
          {selectedGroup && (
            <Card className="p-3">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-sm">
                  {selectedGroup.media_type === 'Movie' ? '🎬' : selectedGroup.media_type === 'Tv' ? '📺' : '📁'}
                </span>
                <h3 className="text-sm font-semibold text-text-primary truncate">
                  {selectedGroup.target_folder_name}
                </h3>
                <Badge variant="default" size="sm">
                  {selectedGroup.media_type} · {selectedGroup.file_count} 文件
                </Badge>
              </div>
              <div className="text-xs text-text-tertiary space-y-1">
                <div>原路径：{selectedGroup.original_path}</div>
                <div>新路径：{selectedGroup.target_path}</div>
              </div>

              {/* TMDb Search */}
              <div className="mt-3 pt-3 border-t border-border">
                <Button
                  variant="primary"
                  size="sm"
                  icon={<Globe className="h-3.5 w-3.5" />}
                  onClick={handleTmdbSearch}
                  isLoading={tmdbLoading}
                  className="w-full"
                >
                  搜索 TMDb
                </Button>
              </div>
            </Card>
          )}

          {!selectedGroup && (
            <Card className="p-4 text-center">
              <p className="text-xs text-text-tertiary">在左侧树中选择一个组查看详情</p>
            </Card>
          )}

          {/* TMDb Candidates */}
          {tmdbCandidates.length > 0 && (
            <Card className="p-0 overflow-hidden">
              <div className="flex items-center justify-between px-3 py-2 border-b border-border bg-surface-subtle">
                <h3 className="text-xs font-semibold text-text-primary">
                  TMDb 候选 ({tmdbCandidates.length})
                </h3>
                <Button variant="ghost" size="sm" icon={<X className="h-3 w-3" />} onClick={() => { setTmdbCandidates([]); setSelectedCandidate(null); }}>
                  {''}
                </Button>
              </div>
              <div className="max-h-80 overflow-y-auto divide-y divide-border">
                {tmdbCandidates.map((c) => (
                  <div
                    key={c.tmdb_id}
                    className={`flex gap-3 p-3 cursor-pointer hover:bg-surface-hover transition-colors ${
                      selectedCandidate?.tmdb_id === c.tmdb_id ? 'bg-primary/5 border-l-2 border-primary' : ''
                    }`}
                    onClick={() => setSelectedCandidate(c)}
                  >
                    {/* Poster */}
                    <div className="w-12 h-18 shrink-0 rounded bg-surface-hover overflow-hidden">
                      {c.poster_path ? (
                        <img
                          src={`${TMDB_IMAGE_BASE}${c.poster_path}`}
                          alt={c.title}
                          className="w-full h-full object-cover"
                          loading="lazy"
                        />
                      ) : (
                        <div className="w-full h-full flex items-center justify-center text-text-tertiary">
                          <Film className="h-5 w-5" />
                        </div>
                      )}
                    </div>
                    {/* Info */}
                    <div className="min-w-0 flex-1">
                      <div className="flex items-center gap-1.5">
                        <span className="text-xs font-medium text-text-primary truncate">{c.title}</span>
                        {c.year && <span className="text-xs text-text-tertiary shrink-0">({c.year})</span>}
                      </div>
                      {c.original_title && c.original_title !== c.title && (
                        <div className="text-xs text-text-tertiary truncate">{c.original_title}</div>
                      )}
                      <div className="flex items-center gap-2 mt-1">
                        {c.vote_average != null && (
                          <span className="flex items-center gap-0.5 text-xs text-warning">
                            <Star className="h-3 w-3 fill-current" />{c.vote_average.toFixed(1)}
                          </span>
                        )}
                        <Badge variant="default" size="sm">
                          {c.media_type === 'Movie' ? '🎬' : '📺'} TMDb {c.tmdb_id}
                        </Badge>
                      </div>
                      {c.overview && (
                        <p className="text-xs text-text-tertiary mt-1 line-clamp-2">{c.overview}</p>
                      )}
                    </div>
                    {/* Apply button */}
                    <div className="shrink-0 flex items-center">
                      <Button
                        variant="primary"
                        size="sm"
                        onClick={(e) => { e.stopPropagation(); handleApplyCandidate(c); }}
                      >
                        应用
                      </Button>
                    </div>
                  </div>
                ))}
              </div>
            </Card>
          )}

          {/* TMDb Error / Loading */}
          {tmdbLoading && (
            <Card className="p-4 flex items-center justify-center gap-2">
              <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
              <span className="text-xs text-text-secondary">搜索中...</span>
            </Card>
          )}
          {tmdbError && (
            <Card className="p-3 border-error/30 bg-error/5">
              <p className="text-xs text-error">{tmdbError}</p>
            </Card>
          )}

          {/* Scrape Preview (when candidate selected) */}
          {selectedCandidate && (
            <Card className="p-3">
              <div className="flex items-center gap-1.5 mb-2">
                <Image className="h-3.5 w-3.5 text-text-secondary" />
                <h3 className="text-xs font-semibold text-text-primary">刮削预览</h3>
              </div>
              <div className="space-y-2 text-xs text-text-secondary">
                <div>
                  <span className="text-text-tertiary">文件夹：</span>
                  {selectedGroup?.target_folder_name}
                </div>
                <div>
                  <span className="text-text-tertiary">文件：</span>
                  {selectedGroup?.children.find((c) => c.file_role === 'MainVideo')?.target_name || selectedGroup?.children[0]?.target_name}
                </div>
                <div className="flex gap-2">
                  <span className="text-text-tertiary">Poster：</span>
                  {selectedCandidate.poster_path ? (
                    <img
                      src={`${TMDB_IMAGE_BASE}${selectedCandidate.poster_path}`}
                      alt="Poster"
                      className="w-16 h-24 rounded object-cover border border-border"
                    />
                  ) : (
                    <span className="text-text-tertiary italic">无</span>
                  )}
                </div>
                <div>
                  <span className="text-text-tertiary">标题：</span>
                  {selectedCandidate.title} {selectedCandidate.year ? `(${selectedCandidate.year})` : ''}
                </div>
                <div>
                  <span className="text-text-tertiary">评分：</span>
                  {selectedCandidate.vote_average != null ? `${selectedCandidate.vote_average.toFixed(1)} / 10` : '无'}
                </div>
                <div>
                  <span className="text-text-tertiary">简介：</span>
                  <span className="line-clamp-3">{selectedCandidate.overview || '无'}</span>
                </div>
                <div>
                  <span className="text-text-tertiary">TMDb ID：</span>
                  {selectedCandidate.tmdb_id}
                </div>
                <div>
                  <span className="text-text-tertiary">写入模式：</span>
                  <Badge variant="default" size="sm">仅命名</Badge>
                </div>
              </div>
            </Card>
          )}
        </div>
      </div>

      {/* ═══ Edit Modal (replaces window.prompt) ═══ */}
      {editModal?.open && (
        <EditNameModal
          mode={editModal.mode}
          currentName={editModal.currentName}
          previewPath={editModal.previewPath}
          onConfirm={confirmEdit}
          onCancel={() => setEditModal(null)}
        />
      )}

      {/* ═══ Execution Panels ═══ */}
      {uiState === 'confirming' && confirmState && (
        <div className="shrink-0">
          <ExecutionConfirmPanel
            state={confirmState}
            onConfirm={() => confirmExecution()}
            onCancel={cancelExecution}
            isLoading={false}
          />
        </div>
      )}
      {uiState === 'executing' && progressState && (
        <div className="shrink-0">
          <ExecutionProgressPanel state={progressState} />
        </div>
      )}
      {uiState === 'completed' && resultState && (
        <div className="shrink-0">
          <ExecutionResultPanel
            state={resultState}
            onRollback={() => {}}
            onExport={() => {}}
            onReset={resetExecution}
          />
        </div>
      )}
    </div>
  );
}

// ─── Edit Name Modal ──────────────────────────────────────────────────────────

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
    if (trimmed && trimmed !== currentName) {
      onConfirm(trimmed);
    } else {
      onCancel();
    }
  }, [value, currentName, onConfirm, onCancel]);

  // Handle Enter key
  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Enter') handleSubmit();
    if (e.key === 'Escape') onCancel();
  }, [handleSubmit, onCancel]);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40" onClick={onCancel}>
      <div
        className="bg-surface rounded-xl border border-border shadow-xl w-[480px] max-w-[95vw] p-0 overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-border bg-surface-subtle">
          <div className="flex items-center gap-2">
            <Edit3 className="h-4 w-4 text-text-secondary" />
            <h3 className="text-sm font-semibold text-text-primary">
              {mode === 'group' ? '编辑文件夹名' : '编辑文件名'}
            </h3>
          </div>
          <Button variant="ghost" size="sm" icon={<X className="h-4 w-4" />} onClick={onCancel}>
            {''}
          </Button>
        </div>

        {/* Body */}
        <div className="p-4 space-y-3">
          <div>
            <label className="text-xs text-text-secondary mb-1 block">原名称</label>
            <div className="text-sm text-text-primary bg-surface-subtle rounded px-3 py-2 break-all">
              {currentName}
            </div>
          </div>
          <div>
            <label className="text-xs text-text-secondary mb-1 block">新名称</label>
            <input
              type="text"
              className="w-full rounded-lg border border-border bg-surface-subtle px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-primary transition-colors"
              value={value}
              onChange={(e) => setValue(e.target.value)}
              onKeyDown={handleKeyDown}
              autoFocus
            />
          </div>
          <div>
            <label className="text-xs text-text-secondary mb-1 block">最终路径预览</label>
            <div className="text-xs text-text-tertiary bg-surface-subtle rounded px-3 py-2 break-all font-mono">
              {previewPath.replace(currentName, value.trim() || currentName)}
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-2 px-4 py-3 border-t border-border bg-surface-subtle">
          <Button variant="secondary" size="sm" onClick={onCancel}>取消</Button>
          <Button variant="primary" size="sm" icon={<Check className="h-4 w-4" />} onClick={handleSubmit}>
            保存
          </Button>
        </div>
      </div>
    </div>
  );
}
