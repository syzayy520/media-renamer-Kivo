import { CheckCircle2, AlertTriangle, XCircle, ArrowRight, Loader2 } from 'lucide-react';
import { Badge, Button } from '../../components/ui';
import type { ApplyTmdbCandidateOutput, TmdbCandidate } from '../../types';

interface CandidateApplySummaryProps {
  candidate: TmdbCandidate;
  applyResult: ApplyTmdbCandidateOutput | null;
  isApplying: boolean;
  onApply: () => void;
  onCancel: () => void;
}

export function CandidateApplySummary({
  candidate,
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

  return (
    <div className="p-4 bg-bg-secondary rounded-lg border border-border space-y-3">
      {/* 标题 */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="text-sm font-medium text-text-primary">已选择候选</span>
          <Badge variant="default" size="sm">
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

      {/* 候选信息 */}
      <div className="flex items-center gap-3 p-3 bg-bg-primary rounded-md border border-border">
        <div className="flex-1 min-w-0">
          <p className="font-medium text-text-primary truncate">{candidate.title}</p>
          <div className="flex items-center gap-2 text-xs text-text-secondary mt-0.5">
            {candidate.year && <span>{candidate.year}</span>}
            {candidate.vote_average && (
              <span>评分 {candidate.vote_average.toFixed(1)}</span>
            )}
            <span>TMDb ID: {candidate.tmdb_id}</span>
          </div>
        </div>
        <ArrowRight className="w-4 h-4 text-text-secondary shrink-0" />
        <div className="text-right">
          <p className="text-sm text-accent font-medium">应用此候选</p>
        </div>
      </div>

      {/* 应用结果 */}
      {hasResult && (
        <div className="space-y-2">
          {isSuccess && (
            <div className="flex items-start gap-2 p-3 bg-success/10 rounded-md border border-success/30">
              <CheckCircle2 className="w-4 h-4 text-success shrink-0 mt-0.5" />
              <div className="flex-1 min-w-0">
                <p className="text-sm text-success font-medium">应用成功</p>
                <p className="text-xs text-text-secondary mt-1">
                  新文件名: <span className="font-mono text-text-primary">{applyResult.result?.updated_item.proposed_name}</span>
                </p>
              </div>
            </div>
          )}

          {hasError && (
            <div className="flex items-start gap-2 p-3 bg-error/10 rounded-md border border-error/30">
              <XCircle className="w-4 h-4 text-error shrink-0 mt-0.5" />
              <div className="flex-1 min-w-0">
                <p className="text-sm text-error font-medium">应用失败</p>
                <p className="text-xs text-text-secondary mt-1">{applyResult.error}</p>
              </div>
            </div>
          )}

          {warnings.length > 0 && (
            <div className="flex items-start gap-2 p-3 bg-warning/10 rounded-md border border-warning/30">
              <AlertTriangle className="w-4 h-4 text-warning shrink-0 mt-0.5" />
              <div className="flex-1 min-w-0">
                <p className="text-sm text-warning font-medium">警告</p>
                <ul className="text-xs text-text-secondary mt-1 space-y-0.5">
                  {warnings.map((w, i) => (
                    <li key={i}>- {w}</li>
                  ))}
                </ul>
              </div>
            </div>
          )}

          {safety && !safety.can_execute && (
            <div className="flex items-start gap-2 p-3 bg-warning/10 rounded-md border border-warning/30">
              <AlertTriangle className="w-4 h-4 text-warning shrink-0 mt-0.5" />
              <div className="flex-1 min-w-0">
                <p className="text-sm text-warning font-medium">安全检查未通过</p>
                <ul className="text-xs text-text-secondary mt-1 space-y-0.5">
                  {safety.checks
                    .filter((c) => !c.passed)
                    .map((c, i) => (
                      <li key={i}>- {c.message}</li>
                    ))}
                </ul>
              </div>
            </div>
          )}
        </div>
      )}

      {/* 操作按钮 */}
      {!hasResult && (
        <Button
          onClick={onApply}
          disabled={isApplying}
          icon={isApplying ? <Loader2 className="w-4 h-4 animate-spin" /> : undefined}
          className="w-full"
        >
          {isApplying ? '正在应用...' : '确认应用此候选'}
        </Button>
      )}
    </div>
  );
}
