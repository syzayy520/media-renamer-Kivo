import { useState, useMemo, useEffect, useCallback } from 'react';
import { ArrowLeft, ArrowRight, Globe, Play } from 'lucide-react';
import { Button, Card, Badge } from '../../components/ui';
import { usePipelineStore } from '../../state/pipelineStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import { useExecutionStore } from '../../state/executionStore';
import { PreviewToolbar } from './PreviewToolbar';
import { PreviewTableEntry } from './PreviewTableEntry';
import { PreviewActionBar } from './PreviewActionBar';
import { PreviewEmptyState } from './PreviewEmptyState';
import { TmdbCandidatePanel } from './TmdbCandidatePanel';
import { PreviewReadinessSummary } from './PreviewReadinessSummary';
import { PreviewNameEditDialog } from './PreviewNameEditDialog';
import { ExecutionConfirmPanel } from './ExecutionConfirmPanel';
import { ExecutionProgressPanel } from './ExecutionProgressPanel';
import { ExecutionResultPanel } from './ExecutionResultPanel';
import type { RenamePreviewItem, ExecutionMode } from '../../types';

export function PreviewPage() {
  const { pipelineResult, updatePreviewProposedName, refreshSafetySummary } = usePipelineStore();
  const { isLoading } = useUiFeedbackStore();
  const { tmdbSearchStatus, disabledReason, itemStates, checkTmdbSearchAvailability } = useTmdbSearchStore();
  const { uiState, confirmState, progressState, resultState, startExecution, confirmExecution, cancelExecution, resetExecution } = useExecutionStore();

  useEffect(() => {
    checkTmdbSearchAvailability();
  }, [checkTmdbSearchAvailability]);

  const [selectedItems, setSelectedItems] = useState<Set<string>>(new Set());
  const [filterText, setFilterText] = useState('');
  const [showOnlySelected, setShowOnlySelected] = useState(false);
  const [activeTmdbItem, setActiveTmdbItem] = useState<RenamePreviewItem | null>(null);
  const [editingItem, setEditingItem] = useState<RenamePreviewItem | null>(null);

  const previews = useMemo(() => pipelineResult?.previews ?? [], [pipelineResult]);
  const safety = pipelineResult?.safety ?? null;
  const tmdbEnabled = tmdbSearchStatus !== 'disabled';

  const filteredItems = useMemo(() => previews.filter((item) => {
    const query = filterText.toLowerCase();
    const matchesFilter = query === '' || item.original_name.toLowerCase().includes(query) || item.proposed_name.toLowerCase().includes(query);
    const matchesSelection = !showOnlySelected || selectedItems.has(item.id);
    return matchesFilter && matchesSelection;
  }), [previews, filterText, showOnlySelected, selectedItems]);

  if (!pipelineResult) return <PreviewEmptyState />;

  const handleSelectItem = (id: string) => {
    const nextSelected = new Set(selectedItems);
    if (nextSelected.has(id)) nextSelected.delete(id);
    else nextSelected.add(id);
    setSelectedItems(nextSelected);
  };

  const handleSelectAll = () => {
    if (selectedItems.size === filteredItems.length) setSelectedItems(new Set());
    else setSelectedItems(new Set(filteredItems.map((item) => item.id)));
  };

  const handleSaveManualEdit = useCallback(async (itemId: string, proposedName: string) => {
    updatePreviewProposedName(itemId, proposedName);
    setEditingItem(null);
    await refreshSafetySummary();
  }, [refreshSafetySummary, updatePreviewProposedName]);

  const selectedCount = selectedItems.size;
  const handleStartExecution = useCallback((mode: ExecutionMode) => startExecution(mode), [startExecution]);
  const handleConfirmExecution = useCallback(() => confirmExecution(), [confirmExecution]);
  const handleCancelExecution = useCallback(() => cancelExecution(), [cancelExecution]);
  const handleResetExecution = useCallback(() => resetExecution(), [resetExecution]);

  return (
    <div className="w-full px-6 py-8">
      <div className="mx-auto w-full max-w-7xl space-y-6">
        <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
          <div className="min-w-0">
            <h1 className="text-3xl font-bold leading-tight text-text-primary">重命名预览</h1>
            <p className="mt-2 text-sm leading-6 text-text-secondary">共 {previews.length} 个文件，已选择 {selectedCount} 个。</p>
          </div>
          <div className="flex flex-wrap gap-3 lg:justify-end">
            <Button variant="secondary" icon={<ArrowLeft className="h-4 w-4" />}>返回</Button>
            <Button icon={<Play className="h-4 w-4" />} onClick={() => handleStartExecution('DryRun')} disabled={uiState !== 'idle'}>模拟执行</Button>
            <Button variant="primary" icon={<ArrowRight className="h-4 w-4" />} onClick={() => handleStartExecution('Confirmed')} disabled={uiState !== 'idle' || !safety?.can_execute}>执行重命名</Button>
          </div>
        </div>

        {tmdbSearchStatus === 'disabled' && (
          <div className="flex items-center gap-3 rounded-2xl border border-warning/30 bg-warning/5 px-4 py-3">
            <Globe className="h-4 w-4 shrink-0 text-warning" />
            <div className="min-w-0 flex-1">
              <Badge variant="warning" size="sm">TMDb 搜索</Badge>
              <span className="ml-2 text-sm text-text-secondary">元数据在线搜索功能暂未启用</span>
              {disabledReason && <p className="mt-1 truncate text-xs text-text-secondary">{disabledReason}</p>}
            </div>
          </div>
        )}

        {activeTmdbItem && <TmdbCandidatePanel item={activeTmdbItem} onClose={() => setActiveTmdbItem(null)} onApplied={() => undefined} />}
        <PreviewNameEditDialog item={editingItem} onSave={handleSaveManualEdit} onClose={() => setEditingItem(null)} />
        {uiState === 'confirming' && <ExecutionConfirmPanel state={confirmState} onConfirm={handleConfirmExecution} onCancel={handleCancelExecution} isLoading={isLoading} />}
        {uiState === 'executing' && <ExecutionProgressPanel state={progressState} />}
        {uiState === 'completed' && <ExecutionResultPanel state={resultState} onRollback={() => undefined} onExport={() => undefined} onReset={handleResetExecution} />}
        <PreviewReadinessSummary previews={previews} itemStates={itemStates} safety={safety} tmdbEnabled={tmdbEnabled} />
        <PreviewToolbar filterText={filterText} onFilterTextChange={setFilterText} showOnlySelected={showOnlySelected} onToggleShowOnlySelected={() => setShowOnlySelected(!showOnlySelected)} selectedCount={selectedCount} filteredCount={filteredItems.length} onSelectAll={handleSelectAll} />
        <Card variant="elevated" padding="none" className="overflow-hidden">
          <PreviewTableEntry items={filteredItems} selectedItems={selectedItems} onSelectItem={handleSelectItem} onSelectAll={handleSelectAll} filteredCount={filteredItems.length} onTmdbSearch={setActiveTmdbItem} onManualEdit={setEditingItem} itemStates={itemStates} tmdbEnabled={tmdbEnabled} />
        </Card>
        <PreviewActionBar selectedCount={selectedCount} onClearSelection={() => setSelectedItems(new Set())} />
        {isLoading && <div className="fixed inset-0 z-50 flex items-center justify-center bg-bg-primary/70"><div className="rounded-2xl bg-bg-card p-6"><p className="text-text-primary">正在处理...</p></div></div>}
      </div>
    </div>
  );
}
