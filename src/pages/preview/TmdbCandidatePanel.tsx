// pages/preview/TmdbCandidatePanel 组件
// 职责：整合 TMDb 搜索、候选列表、应用摘要的完整工作流

import { useState, useCallback } from 'react';
import { X, Search, AlertCircle, WifiOff, Key, Clock } from 'lucide-react';
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
  const { getItemState, selectCandidate, clearSelection, tmdbSearchStatus, disabledReason } = useTmdbSearchStore();
  const { applyTmdbCandidate, refreshSafetySummary } = usePipelineStore();

  const [isApplying, setIsApplying] = useState(false);
  const [applyResult, setApplyResult] = useState<ApplyTmdbCandidateOutput | null>(null);

  const itemState = getItemState(item.id);
  const { results, selectedCandidate, status: itemStatus, error: itemError } = itemState;

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
        await refreshSafetySummary();
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

  // 错误状态展示
  const renderErrorState = () => {
    if (tmdbSearchStatus === 'disabled') {
      return (
        <div className="flex items-start gap-3 p-4 bg-warning/5 rounded-lg border border-warning/20">
          <Key className="w-5 h-5 text-warning shrink-0 mt-0.5" />
          <div>
            <p className="text-sm font-medium text-warning">TMDb 搜索未启用</p>
            <p className="text-xs text-text-secondary mt-1">
              {disabledReason ?? '请在设置中配置 TMDb API Key 并启用在线搜索功能。'}
            </p>
          </div>
        </div>
      );
    }

    if (itemStatus === 'error') {
      const isNetwork = itemError?.toLowerCase().includes('network') ?? false;
      const isRateLimit = itemError?.toLowerCase().includes('rate') ?? false;

      return (
        <div className="flex items-start gap-3 p-4 bg-error/5 rounded-lg border border-error/20">
          {isNetwork ? (
            <WifiOff className="w-5 h-5 text-error shrink-0 mt-0.5" />
          ) : isRateLimit ? (
            <Clock className="w-5 h-5 text-warning shrink-0 mt-0.5" />
          ) : (
            <AlertCircle className="w-5 h-5 text-error shrink-0 mt-0.5" />
          )}
          <div>
            <p className="text-sm font-medium text-error">
              {isNetwork ? '网络连接失败' : isRateLimit ? '请求过于频繁' : '搜索出错'}
            </p>
            <p className="text-xs text-text-secondary mt-1">
              {itemError ?? '请稍后重试。'}
            </p>
          </div>
        </div>
      );
    }

    return null;
  };

  const errorState = renderErrorState();

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
          className="p-1.5 rounded hover:bg-bg-primary transition-colors"
          aria-label="关闭"
        >
          <X className="w-4 h-4 text-text-secondary" />
        </button>
      </div>

      {/* 内容 */}
      <div className="p-4 space-y-4">
        {/* 搜索面板 */}
        <CandidateSearchPanel item={item} />

        {/* 错误状态 */}
        {errorState}

        {/* 加载状态 */}
        {itemStatus === 'loading' && (
          <div className="flex items-center justify-center py-6 text-text-secondary">
            <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-accent mr-3" />
            <span className="text-sm">正在搜索 TMDb...</span>
          </div>
        )}

        {/* 空结果 */}
        {itemStatus === 'success' && results.length === 0 && !selectedCandidate && (
          <div className="text-center py-6 text-text-secondary space-y-1">
            <Search className="w-6 h-6 mx-auto opacity-40" />
            <p className="text-sm">未找到匹配的候选</p>
            <p className="text-xs opacity-70">尝试修改搜索关键词或更换媒体类型</p>
          </div>
        )}

        {/* 结果列表 */}
        {results.length > 0 && !selectedCandidate && (
          <CandidateResultList
            candidates={results}
            selectedCandidate={selectedCandidate}
            onSelect={handleSelectCandidate}
          />
        )}

        {/* 已选择候选 → 应用摘要 */}
        {selectedCandidate && (
          <CandidateApplySummary
            candidate={selectedCandidate}
            originalItem={item}
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
