import { useState, useMemo, useEffect, useCallback } from 'react';
import { ArrowLeft, ArrowRight, Globe, Play } from 'lucide-react';
import { Button, Card, Badge } from '../../components/ui';
import { usePipelineStore } from '../../state/pipelineStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import { useExecutionStore } from '../../state/executionStore';
import { PreviewToolbar } from './PreviewToolbar';
import { PreviewTable } from './PreviewTable';
import { PreviewActionBar } from './PreviewActionBar';
import { PreviewEmptyState } from './PreviewEmptyState';
import { TmdbCandidatePanel } from './TmdbCandidatePanel';
import { PreviewReadinessSummary } from './PreviewReadinessSummary';
import { ExecutionConfirmPanel } from './ExecutionConfirmPanel';
import { ExecutionProgressPanel } from './ExecutionProgressPanel';
import { ExecutionResultPanel } from './ExecutionResultPanel';
import type { RenamePreviewItem, ExecutionMode } from '../../types';

export function PreviewPage() {
  const { pipelineResult } = usePipelineStore();
  const { isLoading } = useUiFeedbackStore();
  const { tmdbSearchStatus, disabledReason, itemStates, checkTmdbSearchAvailability } = useTmdbSearchStore();
  const { 
    uiState, 
    confirmState, 
    progressState, 
    resultState, 
    startExecution, 
    confirmExecution, 
    cancelExecution, 
    resetExecution 
  } = useExecutionStore();

  useEffect(() => {
    checkTmdbSearchAvailability();
  }, [checkTmdbSearchAvailability]);

  const [selectedItems, setSelectedItems] = useState<Set<string>>(new Set());
  const [filterText, setFilterText] = useState('');
  const [showOnlySelected, setShowOnlySelected] = useState(false);
  const [activeTmdbItem, setActiveTmdbItem] = useState<RenamePreviewItem | null>(null);

  const previews = useMemo(() => pipelineResult?.previews ?? [], [pipelineResult]);
  const safety = pipelineResult?.safety ?? null;
  const tmdbEnabled = tmdbSearchStatus !== 'disabled';

  const filteredItems = useMemo(() => {
    return previews.filter((item) => {
      const matchesFilter = filterText === '' ||
        item.original_name.toLowerCase().includes(filterText.toLowerCase()) ||
        item.proposed_name.toLowerCase().includes(filterText.toLowerCase());
      const matchesSelection = !showOnlySelected || selectedItems.has(item.id);
      return matchesFilter && matchesSelection;
    });
  }, [previews, filterText, showOnlySelected, selectedItems]);

  if (!pipelineResult) {
    return <PreviewEmptyState />;
  }

  const handleSelectItem = (id: string) => {
    const newSelected = new Set(selectedItems);
    if (newSelected.has(id)) {
      newSelected.delete(id);
    } else {
      newSelected.add(id);
    }
    setSelectedItems(newSelected);
  };

  const handleSelectAll = () => {
    if (selectedItems.size === filteredItems.length) {
      setSelectedItems(new Set());
    } else {
      setSelectedItems(new Set(filteredItems.map(item => item.id)));
    }
  };

  const handleTmdbSearch = useCallback((item: RenamePreviewItem) => {
    setActiveTmdbItem(item);
  }, []);

  const handleCloseTmdbPanel = useCallback(() => {
    setActiveTmdbItem(null);
  }, []);

  const handleTmdbApplied = useCallback((_updatedItem: RenamePreviewItem) => {
    // 候选已应用，pipelineStore 已更新
    // 可以选择关闭面板或保持打开
  }, []);

  const selectedCount = selectedItems.size;

  // 执行相关处理函数
  const handleStartExecution = useCallback((mode: ExecutionMode) => {
    startExecution(mode);
  }, [startExecution]);

  const handleConfirmExecution = useCallback(() => {
    confirmExecution();
  }, [confirmExecution]);

  const handleCancelExecution = useCallback(() => {
    cancelExecution();
  }, [cancelExecution]);

  const handleResetExecution = useCallback(() => {
    resetExecution();
  }, [resetExecution]);

  const handleRollback = useCallback(() => {
    // TODO: 实现回滚功能
    console.log('Rollback requested');
  }, []);

  const handleExportReport = useCallback(() => {
    // TODO: 实现导出报告功能
    console.log('Export report requested');
  }, []);

  return (
    <div className="w-full px-6 py-8">
      <div className="w-full">
        <div className="flex items-center justify-between mb-8 gap-4">
          <div className="min-w-0 flex-1">
            <h1 className="text-3xl font-bold text-text-primary whitespace-nowrap">
              重命名预览
            </h1>
            <p className="text-text-secondary mt-2">
              共 {previews.length} 个文件，已选择 {selectedCount} 个
            </p>
          </div>
          <div className="flex gap-4 shrink-0">
            <Button variant="secondary" icon={<ArrowLeft className="w-4 h-4" />}>
              返回
            </Button>
            <Button
              icon={<Play className="w-4 h-4" />}
              onClick={() => handleStartExecution('DryRun')}
              disabled={uiState !== 'idle'}
            >
              模拟执行
            </Button>
            <Button
              variant="primary"
              icon={<ArrowRight className="w-4 h-4" />}
              onClick={() => handleStartExecution('Confirmed')}
              disabled={uiState !== 'idle' || !safety?.can_execute}
            >
              执行重命名
            </Button>
          </div>
        </div>

        {tmdbSearchStatus === 'disabled' && (
          <div className="mb-6 flex items-center gap-3 rounded-lg border border-warning/30 bg-warning/5 px-4 py-3">
            <Globe className="w-4 h-4 text-warning shrink-0" />
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2">
                <Badge variant="warning" size="sm">TMDb 搜索</Badge>
                <span className="text-sm text-text-secondary">
                  元数据在线搜索功能暂未启用
                </span>
              </div>
              {disabledReason && (
                <p className="text-xs text-text-secondary mt-1 truncate">
                  {disabledReason}
                </p>
              )}
            </div>
          </div>
        )}

        {/* TMDb 候选面板（当有活跃搜索项时显示） */}
        {activeTmdbItem && (
          <div className="mb-6">
            <TmdbCandidatePanel
              item={activeTmdbItem}
              onClose={handleCloseTmdbPanel}
              onApplied={handleTmdbApplied}
            />
          </div>
        )}

        {/* 执行确认面板 */}
        {uiState === 'confirming' && (
          <ExecutionConfirmPanel
            state={confirmState}
            onConfirm={handleConfirmExecution}
            onCancel={handleCancelExecution}
            isLoading={isLoading}
          />
        )}

        {/* 执行进度面板 */}
        {uiState === 'executing' && (
          <ExecutionProgressPanel
            state={progressState}
          />
        )}

        {/* 执行结果面板 */}
        {uiState === 'completed' && (
          <ExecutionResultPanel
            state={resultState}
            onRollback={handleRollback}
            onExport={handleExportReport}
            onReset={handleResetExecution}
          />
        )}

        <PreviewReadinessSummary
          previews={previews}
          itemStates={itemStates}
          safety={safety}
          tmdbEnabled={tmdbEnabled}
        />

        <PreviewToolbar
          filterText={filterText}
          onFilterTextChange={setFilterText}
          showOnlySelected={showOnlySelected}
          onToggleShowOnlySelected={() => setShowOnlySelected(!showOnlySelected)}
          selectedCount={selectedCount}
          filteredCount={filteredItems.length}
          onSelectAll={handleSelectAll}
        />

        <Card variant="elevated">
          <PreviewTable
            items={filteredItems}
            selectedItems={selectedItems}
            onSelectItem={handleSelectItem}
            onSelectAll={handleSelectAll}
            filteredCount={filteredItems.length}
            onTmdbSearch={handleTmdbSearch}
            itemStates={itemStates}
            tmdbEnabled={tmdbEnabled}
          />
        </Card>

        <PreviewActionBar
          selectedCount={selectedCount}
          onClearSelection={() => setSelectedItems(new Set())}
        />

        {isLoading && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
            <div className="bg-bg-card p-6 rounded-lg">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-accent mx-auto mb-4"></div>
              <p className="text-text-primary">正在处理...</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
