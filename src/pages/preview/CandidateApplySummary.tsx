// pages/preview/CandidateApplySummary 组件
// 职责：展示候选应用前后的对比、结果、安全检查

import { CheckCircle2, AlertTriangle, XCircle, Loader2, ArrowDown } from 'lucide-react';
import { Badge, Button } from '../../components/ui';
import type { ApplyTmdbCandidateOutput, TmdbCandidate, RenamePreviewItem } from '../../types';

interface CandidateApplySummaryProps {
  candidate: TmdbCandidate;
  originalItem: RenamePreviewItem;
  applyResult: ApplyTmdbCandidateOutput | null;
  isApplying: boolean;
  onApply: () => void;
  onCancel: () => void;
}

export function CandidateApplySummary({
  candidate,
  originalItem,
  applyResult,
  isApplying,
  onApply,
  onCancel,
}: CandidateApplySummaryProps) {
  const hasResult = applyResult !== null;
  const isSuccess = applyResult?.result?.success === true;
  const hasError = applyResult?.error !== null;
  const warnings = applyResult?.result?.warnings ?? [];
  const safety = applyResult?.safety;
  const updatedItem = applyResult?.result?.updated_item;

  return (
    <div className="p-4 bg-bg-secondary rounded-lg border border-border space-y-4">
      {/* 标题 */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="text-sm font-medium text-text-primary">候选应用预览</span>
          <Badge variant={candidate.media_type === 'Movie' ? 'default' : 'info'} size="sm">
            {candidate.media_type === 'Movie' ? '电影' : '剧集'}
          </Badge>
        </div>
        <Button
          variant="ghost"
          size="sm"
          onClick={onCancel}
          className="text-text-secondary"
        >
          取消选择
        </Button>
      </div>

      {/* 候选信息卡 */}
      <div className="flex items-center gap-3 p-3 bg-bg-primary rounded-md border border-border">
        {candidate.poster_path && (
          <img
            src={`https://image.tmdb.org/t/p/w92${candidate.poster_path}`}
            alt={candidate.title}
            className="w-10 h-14 rounded object-cover shrink-0"
            loading="lazy"
          />
        )}
        <div className="flex-1 min-w-0">
          <p className="font-medium text-text-primary truncate">{candidate.title}</p>
          <div className="flex items-center gap-2 text-xs text-text-secondary mt-0.5 flex-wrap">
            {candidate.year && <span>{candidate.year}</span>}
            {candidate.vote_average && (
              <span className="flex items-center gap-0.5">
                <span className="text-yellow-500">★</span> {candidate.vote_average.toFixed(1)}
              </span>
            )}
            {candidate.original_language && (
              <Badge variant="default" size="sm">{candidate.original_language.toUpperCase()}</Badge>
            )}
            <span className="opacity-50">ID: {candidate.tmdb_id}</span>
          </div>
        </div>
      </div>

      {/* 文件名变更预览 */}
      <div className="p-3 bg-bg-primary rounded-md border border-border space-y-2">
        <p className="text-xs font-medium text-text-secondary uppercase tracking-wide">文件名变更</p>
        <div className="space-y-1">
          <div className="flex items-center gap-2">
            <span className="text-xs text-text-secondary w-8 shrink-0">旧:</span>
            <span className="font-mono text-sm text-text-secondary truncate">{originalItem.proposed_name}</span>
          </div>
          <div className="flex items-center gap-2">
            <ArrowDown className="w-3 h-3 text-accent shrink-0 ml-2.5" />
          </div>
          <div className="flex items-center gap-2">
            <span className="text-xs text-accent w-8 shrink-0">新:</span>
            <span className="font-mono text-sm text-accent font-medium truncate">
              {updatedItem?.proposed_name ?? '(应用后显示)'}
            </span>
          </div>
        </div>
      </div>

      {/* 应用结果 */}
      {hasResult && (
        <div className="space-y-2">
          {/* 成功 */}
          {isSuccess && (
            <div className="flex items-start gap-2 p-3 bg-success/10 rounded-md border border-success/30">
              <CheckCircle2 className="w-4 h-4 text-success shrink-0 mt-0.5" />
              <div className="flex-1 min-w-0">
                <p className="text-sm text-success font-medium">应用成功</p>
                <div className="text-xs text-text-secondary mt-1 space-y-0.5">
                  {updatedItem && (
                    <>
                      <p>标题: {updatedItem.parsed_info.title}</p>
                      {updatedItem.parsed_info.year && <p>年份: {updatedItem.parsed_info.year}</p>}
                      {updatedItem.parsed_info.season && <p>季: S{String(updatedItem.parsed_info.season).padStart(2, '0')}</p>}
                      {updatedItem.parsed_info.episode && <p>集: E{String(updatedItem.parsed_info.episode).padStart(2, '0')}</p>}
                    </>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* 错误 */}
          {hasError && (
            <div className="flex items-start gap-2 p-3 bg-error/10 rounded-md border border-error/30">
              <XCircle className="w-4 h-4 text-error shrink-0 mt-0.5" />
              <div className="flex-1 min-w-0">
                <p className="text-sm text-error font-medium">应用失败</p>
                <p className="text-xs text-text-secondary mt-1">{applyResult.error}</p>
              </div>
            </div>
          )}

          {/* 警告 */}
          {warnings.length > 0 && (
            <div className="flex items-start gap-2 p-3 bg-warning/10 rounded-md border border-warning/30">
              <AlertTriangle className="w-4 h-4 text-warning shrink-0 mt-0.5" />
              <div className="flex-1 min-w-0">
                <p className="text-sm text-warning font-medium">注意事项</p>
                <ul className="text-xs text-text-secondary mt-1 space-y-0.5">
                  {warnings.map((w, i) => (
                    <li key={i} className="flex items-start gap-1">
                      <span className="text-warning">•</span> {w}
                    </li>
                  ))}
                </ul>
              </div>
            </div>
          )}

          {/* 安全检查 — 分层展示 */}
          {safety && (
            <SafetyCheckList safety={safety} />
          )}
        </div>
      )}

      {/* 操作按钮 */}
      {!hasResult && (
        <div className="space-y-2">
          <Button
            onClick={onApply}
            disabled={isApplying}
            icon={isApplying ? <Loader2 className="w-4 h-4 animate-spin" /> : undefined}
            className="w-full"
          >
            {isApplying ? '正在应用...' : '确认应用此候选'}
          </Button>
          <p className="text-xs text-text-secondary text-center">
            仅更新预览，不会修改原始文件
          </p>
        </div>
      )}
    </div>
  );
}

/** 安全检查分层展示：blocker / warning / info */
function SafetyCheckList({ safety }: { safety: NonNullable<ApplyTmdbCandidateOutput['safety']> }) {
  const blockers = safety.checks.filter((c) => !c.passed && safety.blocking_reasons.some((r) => c.message.includes(r)));
  const failedChecks = safety.checks.filter((c) => !c.passed && !blockers.includes(c));
  const passedChecks = safety.checks.filter((c) => c.passed);

  if (safety.can_execute && failedChecks.length === 0) {
    return (
      <div className="flex items-center gap-2 p-3 bg-success/10 rounded-md border border-success/30">
        <CheckCircle2 className="w-4 h-4 text-success shrink-0" />
        <p className="text-sm text-success font-medium">安全检查全部通过</p>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      {/* Blockers */}
      {blockers.length > 0 && (
        <div className="p-3 bg-error/10 rounded-md border border-error/30">
          <div className="flex items-center gap-2 mb-1.5">
            <XCircle className="w-4 h-4 text-error shrink-0" />
            <p className="text-sm text-error font-medium">阻断项 ({blockers.length})</p>
          </div>
          <ul className="text-xs text-text-secondary space-y-0.5 ml-6">
            {blockers.map((c, i) => (
              <li key={i} className="text-error/80">• {c.message}</li>
            ))}
          </ul>
        </div>
      )}

      {/* Warnings */}
      {failedChecks.length > 0 && (
        <div className="p-3 bg-warning/10 rounded-md border border-warning/30">
          <div className="flex items-center gap-2 mb-1.5">
            <AlertTriangle className="w-4 h-4 text-warning shrink-0" />
            <p className="text-sm text-warning font-medium">警告 ({failedChecks.length})</p>
          </div>
          <ul className="text-xs text-text-secondary space-y-0.5 ml-6">
            {failedChecks.map((c, i) => (
              <li key={i} className="text-warning/80">• {c.message}</li>
            ))}
          </ul>
        </div>
      )}

      {/* Passed info */}
      {passedChecks.length > 0 && (
        <p className="text-xs text-text-secondary">
          <span className="text-success">✓</span> {passedChecks.length} 项检查已通过
        </p>
      )}
    </div>
  );
}
