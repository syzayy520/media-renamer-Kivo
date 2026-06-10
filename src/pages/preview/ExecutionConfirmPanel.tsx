import { AlertTriangle, CheckCircle, XCircle, Loader2 } from 'lucide-react';
import { Button, Card, Badge } from '../../components/ui';
import type { ExecutionConfirmState } from '../../types';

interface ExecutionConfirmPanelProps {
  state: ExecutionConfirmState;
  onConfirm: () => void;
  onCancel: () => void;
  isLoading: boolean;
}

export function ExecutionConfirmPanel({
  state,
  onConfirm,
  onCancel,
  isLoading,
}: ExecutionConfirmPanelProps) {
  const { safety_report, preview_count, has_blockers, blocking_reasons } = state;

  return (
    <Card variant="elevated" className="mb-6">
      <div className="p-6">
        <div className="flex items-center gap-3 mb-4">
          {has_blockers ? (
            <XCircle className="w-6 h-6 text-error shrink-0" />
          ) : (
            <CheckCircle className="w-6 h-6 text-success shrink-0" />
          )}
          <div>
            <h3 className="text-lg font-semibold text-text-primary">
              执行确认
            </h3>
            <p className="text-sm text-text-secondary">
              {has_blockers
                ? '安全检查未通过，无法执行重命名'
                : `准备执行 ${preview_count} 个文件的重命名操作`}
            </p>
          </div>
        </div>

        {/* 安全检查摘要 */}
        {safety_report && (
          <div className="mb-4 p-4 rounded-lg bg-bg-secondary">
            <h4 className="text-sm font-medium text-text-primary mb-2">
              安全检查结果
            </h4>
            <div className="grid grid-cols-2 gap-2 text-sm">
              <div className="flex items-center gap-2">
                <span className="text-text-secondary">总检查项:</span>
                <span className="text-text-primary">{safety_report.checks.length}</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-text-secondary">通过:</span>
                <span className="text-success">
                  {safety_report.checks.filter(c => c.passed).length}
                </span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-text-secondary">失败:</span>
                <span className="text-error">
                  {safety_report.checks.filter(c => !c.passed).length}
                </span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-text-secondary">可执行:</span>
                <Badge variant={safety_report.can_execute ? 'success' : 'danger'} size="sm">
                  {safety_report.can_execute ? '是' : '否'}
                </Badge>
              </div>
            </div>
          </div>
        )}

        {/* 阻塞原因 */}
        {has_blockers && blocking_reasons.length > 0 && (
          <div className="mb-4 p-4 rounded-lg border border-error/30 bg-error/5">
            <div className="flex items-center gap-2 mb-2">
              <AlertTriangle className="w-4 h-4 text-error" />
              <h4 className="text-sm font-medium text-error">阻塞原因</h4>
            </div>
            <ul className="text-sm text-error space-y-1">
              {blocking_reasons.map((reason, index) => (
                <li key={index} className="flex items-start gap-2">
                  <span className="text-error">•</span>
                  <span>{reason}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        {/* 操作按钮 */}
        <div className="flex items-center justify-end gap-3">
          <Button
            variant="secondary"
            onClick={onCancel}
            disabled={isLoading}
          >
            取消
          </Button>
          <Button
            variant={has_blockers ? 'danger' : 'primary'}
            onClick={onConfirm}
            disabled={has_blockers || isLoading}
            icon={isLoading ? <Loader2 className="w-4 h-4 animate-spin" /> : undefined}
          >
            {isLoading ? '执行中...' : '确认执行'}
          </Button>
        </div>
      </div>
    </Card>
  );
}