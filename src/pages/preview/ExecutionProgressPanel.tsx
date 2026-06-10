import { Loader2, CheckCircle, XCircle, SkipForward, Ban } from 'lucide-react';
import { Card, Badge } from '../../components/ui';
import type { ExecutionProgressState } from '../../types';

interface ExecutionProgressPanelProps {
  state: ExecutionProgressState;
}

export function ExecutionProgressPanel({ state }: ExecutionProgressPanelProps) {
  const { total, completed, success, failed, skipped, blocked, current_item } = state;
  
  const progress = total > 0 ? (completed / total) * 100 : 0;
  const isComplete = completed === total;

  return (
    <Card variant="elevated" className="mb-6">
      <div className="p-6">
        <div className="flex items-center gap-3 mb-4">
          {isComplete ? (
            <CheckCircle className="w-6 h-6 text-success shrink-0" />
          ) : (
            <Loader2 className="w-6 h-6 text-accent animate-spin shrink-0" />
          )}
          <div>
            <h3 className="text-lg font-semibold text-text-primary">
              {isComplete ? '执行完成' : '执行中...'}
            </h3>
            <p className="text-sm text-text-secondary">
              {isComplete
                ? `已处理 ${total} 个文件`
                : `正在处理 ${completed + 1}/${total} 个文件`}
            </p>
          </div>
        </div>

        {/* 进度条 */}
        <div className="mb-4">
          <div className="flex items-center justify-between text-sm mb-1">
            <span className="text-text-secondary">进度</span>
            <span className="text-text-primary">{Math.round(progress)}%</span>
          </div>
          <div className="w-full bg-bg-secondary rounded-full h-2">
            <div
              className="bg-accent h-2 rounded-full transition-all duration-300"
              style={{ width: `${progress}%` }}
            />
          </div>
        </div>

        {/* 统计信息 */}
        <div className="grid grid-cols-4 gap-2 mb-4">
          <div className="flex items-center gap-2 p-2 rounded bg-success/10">
            <CheckCircle className="w-4 h-4 text-success" />
            <div>
              <div className="text-lg font-semibold text-success">{success}</div>
              <div className="text-xs text-text-secondary">成功</div>
            </div>
          </div>
          <div className="flex items-center gap-2 p-2 rounded bg-error/10">
            <XCircle className="w-4 h-4 text-error" />
            <div>
              <div className="text-lg font-semibold text-error">{failed}</div>
              <div className="text-xs text-text-secondary">失败</div>
            </div>
          </div>
          <div className="flex items-center gap-2 p-2 rounded bg-warning/10">
            <SkipForward className="w-4 h-4 text-warning" />
            <div>
              <div className="text-lg font-semibold text-warning">{skipped}</div>
              <div className="text-xs text-text-secondary">跳过</div>
            </div>
          </div>
          <div className="flex items-center gap-2 p-2 rounded bg-text-secondary/10">
            <Ban className="w-4 h-4 text-text-secondary" />
            <div>
              <div className="text-lg font-semibold text-text-secondary">{blocked}</div>
              <div className="text-xs text-text-secondary">阻塞</div>
            </div>
          </div>
        </div>

        {/* 当前处理项 */}
        {current_item && (
          <div className="p-3 rounded-lg bg-bg-secondary">
            <div className="text-sm text-text-secondary mb-1">当前处理:</div>
            <div className="text-sm text-text-primary truncate">{current_item}</div>
          </div>
        )}
      </div>
    </Card>
  );
}