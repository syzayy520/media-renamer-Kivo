import { useState, useMemo, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { ArrowLeft, Globe, Play, Settings } from 'lucide-react';
import { Button, Card, Badge } from '../../components/ui';
import { usePipelineStore } from '../../state/pipelineStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import { useExecutionStore } from '../../state/executionStore';
import { PreviewPlanTree } from './tree/PreviewPlanTree';
import { NamingRulePanel } from './naming/NamingRulePanel';
import { FolderPolicyPanel } from './folder-policy/FolderPolicyPanel';
import { TmdbCandidatePanel } from './TmdbCandidatePanel';
import { ExecutionConfirmPanel } from './ExecutionConfirmPanel';
import { ExecutionProgressPanel } from './ExecutionProgressPanel';
import { ExecutionResultPanel } from './ExecutionResultPanel';
import type { NamingRule, TitleStrategy, FolderPolicyConfig, FolderGroup } from '../../types';
import { buildPreviewGroups } from './model/buildPreviewGroups';

function buildMockNamingRule(): NamingRule {
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

export function PreviewPage() {
  const navigate = useNavigate();
  const { pipelineResult, updatePreviewProposedName, togglePreviewSkipped, refreshSafetySummary } = usePipelineStore();
  const { isLoading } = useUiFeedbackStore();
  const { tmdbSearchStatus, disabledReason } = useTmdbSearchStore();
  const { uiState, confirmState, progressState, resultState, startExecution, confirmExecution, cancelExecution, resetExecution } = useExecutionStore();

  const [expandedGroups, setExpandedGroups] = useState<Set<string>>(new Set());
  const [selectedGroupId, setSelectedGroupId] = useState<string | null>(null);
  const [tmdbPanelOpen, setTmdbPanelOpen] = useState(false);
  const [currentNamingRule, setCurrentNamingRule] = useState<NamingRule>(buildMockNamingRule());
  const [currentStrategy, setCurrentStrategy] = useState<TitleStrategy>('ChineseOnly');
  const [folderPolicyConfig, setFolderPolicyConfig] = useState<FolderPolicyConfig>({ policy: 'KeepOriginalStructure', clean_empty_folders_after: false });

  const planTree = useMemo(() => {
    if (!pipelineResult) return null;
    return buildPreviewGroups(pipelineResult.previews);
  }, [pipelineResult]);

  const groups: FolderGroup[] = planTree?.groups ?? [];

  const handleToggleExpand = useCallback((groupId: string) => {
    setExpandedGroups((prev) => {
      const next = new Set(prev);
      if (next.has(groupId)) next.delete(groupId);
      else next.add(groupId);
      return next;
    });
  }, []);

  const handleGroupTmdbSearch = useCallback((groupId: string) => {
    setSelectedGroupId(groupId);
    setTmdbPanelOpen(true);
  }, []);

  const handleGroupEdit = useCallback((groupId: string) => {
    const group = groups.find((g) => g.id === groupId);
    if (!group || group.children.length === 0) return;
    updatePreviewProposedName(group.children[0].id, group.target_folder_name + group.children[0].extension);
  }, [groups, updatePreviewProposedName]);

  const handleGroupSkip = useCallback((groupId: string) => {
    const group = groups.find((g) => g.id === groupId);
    if (!group) return;
    group.children.forEach((child) => togglePreviewSkipped(child.id));
  }, [groups, togglePreviewSkipped]);

  const handleFileEdit = useCallback((fileId: string) => {
    const item = pipelineResult?.previews.find((p) => p.id === fileId);
    if (item) {
      const newName = prompt('编辑文件名', item.proposed_name);
      if (newName && newName !== item.proposed_name) {
        updatePreviewProposedName(fileId, newName);
      }
    }
  }, [pipelineResult, updatePreviewProposedName]);

  const handleFileSkip = useCallback((fileId: string) => {
    togglePreviewSkipped(fileId);
  }, [togglePreviewSkipped]);

  const previewSamples = useMemo(() => {
    if (!pipelineResult || pipelineResult.previews.length === 0) return { movie: '预览输出将显示在这里' };
    const first = pipelineResult.previews[0];
    return { movie: first.proposed_name };
  }, [pipelineResult]);

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
    <div className="flex flex-col gap-4">
      {/* Header */}
      <div className="flex items-center justify-between">
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
          <Button variant="primary" size="sm" icon={<Play className="h-4 w-4" />} onClick={() => startExecution('DryRun')}>
            执行重命名
          </Button>
        </div>
      </div>

      {/* TMDb Disabled Banner */}
      {tmdbSearchStatus === 'disabled' && (
        <div className="flex items-center gap-3 rounded-2xl border border-warning/30 bg-warning/5 px-4 py-3">
          <Globe className="h-4 w-4 shrink-0 text-warning" />
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-2">
              <Badge variant="warning" size="sm">TMDb 搜索</Badge>
              <span className="text-sm text-text-secondary">元数据在线搜索功能暂未启用</span>
            </div>
            {disabledReason && <p className="mt-1 truncate text-xs text-text-secondary">{disabledReason}</p>}
          </div>
          <Button variant="secondary" size="sm" icon={<Settings className="h-3.5 w-3.5" />} onClick={() => navigate('/settings')}>
            去设置
          </Button>
        </div>
      )}

      {/* Naming Rule + Folder Policy */}
      <div className="grid grid-cols-2 gap-4">
        <NamingRulePanel
          currentRule={currentNamingRule}
          currentStrategy={currentStrategy}
          onRuleChange={setCurrentNamingRule}
          onStrategyChange={setCurrentStrategy}
          previewName={previewSamples.movie}
        />
        <FolderPolicyPanel
          config={folderPolicyConfig}
          onConfigChange={setFolderPolicyConfig}
        />
      </div>

      {/* Folder Tree */}
      <Card className="p-0 overflow-hidden">
        <PreviewPlanTree
          groups={groups}
          expandedGroups={expandedGroups}
          onGroupToggleExpand={handleToggleExpand}
          onGroupTmdbSearch={handleGroupTmdbSearch}
          onGroupEdit={handleGroupEdit}
          onGroupSkip={handleGroupSkip}
          onFileEdit={handleFileEdit}
          onFileSkip={handleFileSkip}
        />
      </Card>

      {/* TMDb Candidate Panel */}
      {tmdbPanelOpen && selectedGroupId && (
        <TmdbCandidatePanel
          groupId={selectedGroupId}
          groups={groups}
          onClose={() => { setTmdbPanelOpen(false); setSelectedGroupId(null); }}
          onApply={() => {
            refreshSafetySummary();
            setTmdbPanelOpen(false);
          }}
        />
      )}

      {/* Execution Panels */}
      {uiState === 'confirming' && confirmState && (
        <ExecutionConfirmPanel
          state={confirmState}
          onConfirm={() => confirmExecution()}
          onCancel={cancelExecution}
          isLoading={false}
        />
      )}

      {uiState === 'executing' && progressState && (
        <ExecutionProgressPanel
          state={progressState}
        />
      )}

      {uiState === 'completed' && resultState && (
        <ExecutionResultPanel
          state={resultState}
          onRollback={() => {}}
          onExport={() => {}}
          onReset={resetExecution}
        />
      )}
    </div>
  );
}
