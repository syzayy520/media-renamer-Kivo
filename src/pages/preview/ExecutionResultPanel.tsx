import { CheckCircle, XCircle, AlertTriangle, RotateCcw, Download } from 'lucide-react';
import { Button, Card, Badge } from '../../components/ui';
import type { ExecutionResultState } from '../../types';

interface ExecutionResultPanelProps {
  state: ExecutionResultState;
  onRollback: () => void;
  onExport: () => void;
  onReset: () => void;
}

export function ExecutionResultPanel({
  state,
  onRollback,
  onExport,
  onReset,
}: ExecutionResultPanelProps) {
  const { output, error, has_rollback_plan, can_rollback } = state;

  if (error) {
    return (
      <Card variant="elevated" className="mb-6">
        <div className="p-6">
          <div className="flex items-center gap-3 mb-4">
            <XCircle className="w-6 h-6 text-error shrink-0" />
            <div>
              <h3 className="text-lg font-semibold text-text-primary">
                执行失败
              </h3>
              <p className="text-sm text-error">{error}</p>
            </div>
          </div>
          <div className="flex items-center justify-end gap-3">
            <Button variant="secondary" onClick={onReset}>
              重新开始
            </Button>
          </div>
        </div>
      </Card>
    );
  }

  if (!output) {
    return null;
  }

  const { summary, item_results, rollback_plan } = output;
  const successRate = summary ? (summary.success / summary.total) * 100 : 0;

  return (
    <Card variant="elevated" className="mb-6">
      <div className="p-6">
        <div className="flex items-center gap-3 mb-4">
          {summary?.failed === 0 && summary?.blocked === 0 ? (
            <CheckCircle className="w-6 h-6 text-success shrink-0" />
          ) : (
            <AlertTriangle className="w-6 h-6 text-warning shrink-0" />
          )}
          <div>
            <h3 className="text-lg font-semibold text-text-primary">
              执行完成
            </h3>
            <p className="text-sm text-text-secondary">
              {summary?.success} 个成功，{summary?.failed} 个失败
              {summary?.blocked ? `，${summary.blocked} 个阻塞` : ''}
            </p>
          </div>
        </div>

        {/* 成功率 */}
        <div className="mb-4 p-4 rounded-lg bg-bg-secondary">
          <div className="flex items-center justify-between text-sm mb-2">
            <span className="text-text-secondary">成功率</span>
            <span className="text-text-primary">{successRate.toFixed(1)}%</span>
          </div>
          <div className="w-full bg-bg-primary rounded-full h-3">
            <div
              className={`h-3 rounded-full transition-all duration-300 ${
                successRate === 100 ? 'bg-success' : successRate >= 80 ? 'bg-warning' : 'bg-error'
              }`}
              style={{ width: `${successRate}%` }}
            />
          </div>
        </div>

        {/* 详细统计 */}
        <div className="grid grid-cols-4 gap-2 mb-4">
          <div className="text-center p-2 rounded bg-success/10">
            <div className="text-lg font-semibold text-success">{summary?.success || 0}</div>
            <div className="text-xs text-text-secondary">成功</div>
          </div>
          <div className="text-center p-2 rounded bg-error/10">
            <div className="text-lg font-semibold text-error">{summary?.failed || 0}</div>
            <div className="text-xs text-text-secondary">失败</div>
          </div>
          <div className="text-center p-2 rounded bg-warning/10">
            <div className="text-lg font-semibold text-warning">{summary?.skipped || 0}</div>
            <div className="text-xs text-text-secondary">跳过</div>
          </div>
          <div className="text-center p-2 rounded bg-text-secondary/10">
            <div className="text-lg font-semibold text-text-secondary">{summary?.blocked || 0}</div>
            <div className="text-xs text-text-secondary">阻塞</div>
          </div>
        </div>

        {/* 回滚计划信息 */}
        {has_rollback_plan && (
          <div className="mb-4 p-4 rounded-lg border border-accent/30 bg-accent/5">
            <div className="flex items-center gap-2 mb-2">
              <RotateCcw className="w-4 h-4 text-accent" />
              <h4 className="text-sm font-medium text-accent">回滚计划</h4>
            </div>
            <p className="text-sm text-text-secondary">
              已生成回滚计划，包含 {rollback_plan?.entries.length || 0} 个条目。
              {can_rollback
                ? '您可以回滚已执行的重命名操作。'
                : '没有已执行的条目，无需回滚。'}
            </p>
          </div>
        )}

        {/* 失败项列表 */}
        {item_results.some(r => r.status === 'Failed' || r.status === 'Blocked') && (
          <div className="mb-4">
            <h4 className="text-sm font-medium text-text-primary mb-2">失败项</h4>
            <div className="max-h-40 overflow-y-auto space-y-2">
              {item_results
                .filter(r => r.status === 'Failed' || r.status === 'Blocked')
                .map((result, index) => (
                  <div
                    key={index}
                    className="flex items-start gap-2 p-2 rounded bg-error/5 border border-error/20"
                  >
                    <XCircle className="w-4 h-4 text-error shrink-0 mt-0.5" />
                    <div className="min-w-0 flex-1">
                      <div className="text-sm text-text-primary truncate">
                        {result.source_path}
                      </div>
                      <div className="text-xs text-error">{result.error}</div>
                    </div>
                    <Badge variant="danger" size="sm">
                      {result.status}
                    </Badge>
                  </div>
                ))}
            </div>
          </div>
        )}

        {/* 操作按钮 */}
        <div className="flex items-center justify-end gap-3">
          <Button
            variant="secondary"
            icon={<Download className="w-4 h-4" />}
            onClick={onExport}
          >
            导出报告
          </Button>
          {can_rollback && (
            <Button
              variant="secondary"
              icon={<RotateCcw className="w-4 h-4" />}
              onClick={onRollback}
            >
              回滚操作
            </Button>
          )}
          <Button variant="secondary" onClick={onReset}>
            完成
          </Button>
        </div>
      </div>
    </Card>
  );
}