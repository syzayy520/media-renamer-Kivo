// features/audit/page/AuditPage.tsx — 审计页面
// 职责：展示任务列表、任务详情、审计日志

import { useEffect } from 'react';
import { useAuditPageStore } from '../state/auditPageStore';
import { getAllTasks } from '../../../api/audit/getAllTasks';
import { getTask } from '../../../api/audit/getTask';
import { getAuditLogs } from '../../../api/audit/getAuditLogs';
import { AuditEmptyState } from '../components/AuditEmptyState';
import { AuditToolbar } from '../components/AuditToolbar';
import { TaskList } from '../components/TaskList';
import { TaskDetail } from '../components/TaskDetail';
import { AuditLogList } from '../components/AuditLogList';
import { AuditErrorPanel } from '../components/AuditErrorPanel';

export function AuditPage() {
  const {
    tasks,
    selectedTaskId,
    selectedTask,
    logs,
    isLoadingTasks,
    isLoadingLogs,
    error,
    setTasks,
    setSelectedTaskId,
    setSelectedTask,
    setLogs,
    setIsLoadingTasks,
    setIsLoadingLogs,
    setError,
    clearError,
  } = useAuditPageStore();

  // 首次加载任务列表
  useEffect(() => {
    const doLoad = async () => {
      clearError();
      setIsLoadingTasks(true);
      try {
        const data = await getAllTasks();
        setTasks(data);
      } catch (err: unknown) {
        const msg = err instanceof Error ? err.message : String(err);
        setError(msg);
      }
    };
    doLoad();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const loadTasks = async () => {
    clearError();
    setIsLoadingTasks(true);
    try {
      const data = await getAllTasks();
      setTasks(data);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setError(msg);
    }
  };

  const handleSelectTask = async (id: string) => {
    setSelectedTaskId(id);
    clearError();
    setIsLoadingLogs(true);
    try {
      const [task, auditLogs] = await Promise.all([
        getTask(id),
        getAuditLogs(id),
      ]);
      setSelectedTask(task);
      setLogs(auditLogs);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setError(msg);
    }
  };

  // 无任务 → 空状态
  if (!isLoadingTasks && tasks.length === 0 && !error) {
    return <AuditEmptyState />;
  }

  return (
    <div className="space-y-4">
      {/* 标题 */}
      <div className="rounded-lg border border-white/10 bg-white/5 p-4">
        <h2 className="mb-1 text-lg font-semibold">审计记录</h2>
        <p className="text-sm text-white/50">
          当前仅展示任务和审计日志，不会真实修改媒体文件。
        </p>
      </div>

      {/* 错误 */}
      {error && <AuditErrorPanel message={error} onDismiss={clearError} />}

      {/* 工具栏 */}
      <AuditToolbar
        isLoading={isLoadingTasks}
        taskCount={tasks.length}
        onRefresh={loadTasks}
      />

      {/* 任务列表 */}
      {isLoadingTasks ? (
        <div className="py-8 text-center text-sm text-white/30">加载任务中...</div>
      ) : (
        <TaskList
          tasks={tasks}
          selectedTaskId={selectedTaskId}
          onSelect={handleSelectTask}
        />
      )}

      {/* 任务详情 */}
      {selectedTask && (
        <div>
          <h3 className="mb-2 text-sm font-medium text-white/70">任务详情</h3>
          <TaskDetail task={selectedTask} />
        </div>
      )}

      {/* 审计日志 */}
      {selectedTaskId && (
        <div>
          <h3 className="mb-2 text-sm font-medium text-white/70">
            审计日志
            {isLoadingLogs && (
              <span className="ml-2 font-normal text-white/30">加载中...</span>
            )}
          </h3>
          {isLoadingLogs ? (
            <div className="py-6 text-center text-sm text-white/30">
              加载审计日志中...
            </div>
          ) : (
            <AuditLogList logs={logs} />
          )}
        </div>
      )}
    </div>
  );
}
