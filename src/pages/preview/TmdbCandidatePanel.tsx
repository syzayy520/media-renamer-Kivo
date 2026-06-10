import { useState, useCallback } from 'react';
import { X, Search } from 'lucide-react';
import { useTmdbSearchStore } from '../../state/tmdbSearchStore';
import { usePipelineStore } from '../../state/pipelineStore';
import { CandidateSearchPanel } from './CandidateSearchPanel';
import { CandidateResultList } from './CandidateResultList';
import { CandidateApplySummary } from './CandidateApplySummary';
import type { ApplyTmdbCandidateOutput, RenamePreviewItem, TmdbCandidate } from '../../types';

interface TmdbCandidatePanelProps {
  item: RenamePreviewItem;
  onClose: () => void;
  onApplied?: (updatedItem: RenamePreviewItem) => void;
}

export function TmdbCandidatePanel({ item, onClose, onApplied }: TmdbCandidatePanelProps) {
  const { getItemState, selectCandidate, clearSelection } = useTmdbSearchStore();
  const { applyTmdbCandidate, refreshSafetySummary } = usePipelineStore();

  const [isApplying, setIsApplying] = useState(false);
  const [applyResult, setApplyResult] = useState<ApplyTmdbCandidateOutput | null>(null);

  const itemState = getItemState(item.id);
  const { results, selectedCandidate } = itemState;

  const handleSelectCandidate = useCallback(
    (candidate: TmdbCandidate) => {
      selectCandidate(item.id, candidate);
      setApplyResult(null);
    },
    [item.id, selectCandidate],
  );

  const handleCancelSelection = useCallback(() => {
    clearSelection(item.id);
    setApplyResult(null);
  }, [item.id, clearSelection]);

  const handleApply = useCallback(async () => {
    if (!selectedCandidate) return;

    setIsApplying(true);
    setApplyResult(null);

    try {
      const output = await applyTmdbCandidate(item.id, selectedCandidate);
      setApplyResult(output);

      if (output.result?.success) {
        // 刷新安全摘要
        await refreshSafetySummary();
        // 通知父组件
        onApplied?.(output.result.updated_item);
      }
    } catch (err) {
      setApplyResult({
        result: null,
        error: err instanceof Error ? err.message : String(err),
        safety: null,
      });
    } finally {
      setIsApplying(false);
    }
  }, [selectedCandidate, item.id, applyTmdbCandidate, refreshSafetySummary, onApplied]);

  return (
    <div className="bg-bg-card rounded-lg border border-border shadow-lg overflow-hidden">
      {/* 头部 */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-border bg-bg-secondary">
        <div className="flex items-center gap-2 min-w-0">
          <Search className="w-4 h-4 text-accent shrink-0" />
          <span className="text-sm font-medium text-text-primary truncate">
            TMDb 搜索: {item.original_name}
          </span>
        </div>
        <button
          onClick={onClose}
          className="p-1 rounded hover:bg-bg-primary transition-colors"
        >
          <X className="w-4 h-4 text-text-secondary" />
        </button>
      </div>

      {/* 内容 */}
      <div className="p-4 space-y-4">
        {/* 搜索面板 */}
        <CandidateSearchPanel item={item} />

        {/* 结果列表 */}
        {results.length > 0 && !selectedCandidate && (
          <div>
            <p className="text-xs text-text-secondary mb-2">
              找到 {results.length} 个结果，点击选择一个候选
            </p>
            <CandidateResultList
              candidates={results}
              selectedCandidate={selectedCandidate}
              onSelect={handleSelectCandidate}
            />
          </div>
        )}

        {/* 已选择候选 → 应用摘要 */}
        {selectedCandidate && (
          <CandidateApplySummary
            candidate={selectedCandidate}
            applyResult={applyResult}
            isApplying={isApplying}
            onApply={handleApply}
            onCancel={handleCancelSelection}
          />
        )}
      </div>
    </div>
  );
}
