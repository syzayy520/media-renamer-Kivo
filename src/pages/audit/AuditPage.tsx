import { useState } from 'react';
import { Download, RefreshCw } from 'lucide-react';
import { Button, LoadingSpinner } from '../../components/ui';
import { usePipelineStore } from '../../state/pipelineStore';
import { useTaskStore } from '../../state/taskStore';
import { useAuditStore } from '../../state/auditStore';
import { useUiFeedbackStore } from '../../state/uiFeedbackStore';
import { SafetyOverviewSection } from './SafetyOverviewSection';
import { SafetyDetailsSection } from './SafetyDetailsSection';
import { TaskHistorySection } from './TaskHistorySection';
import { AuditLogSection } from './AuditLogSection';

export function AuditPage() {
  const { pipelineResult } = usePipelineStore();
  const { tasks, fetchAllTasks } = useTaskStore();
  const { auditLogs, fetchAuditLogs } = useAuditStore();
  const { isLoading } = useUiFeedbackStore();
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);

  const handleRefresh = async () => {
    await fetchAllTasks();
  };

  const handleViewLogs = async (taskId: string) => {
    setSelectedTaskId(taskId);
    await fetchAuditLogs(taskId);
  };

  const safetyChecks = pipelineResult?.safety.checks || [];
  const canExecute = pipelineResult?.safety.can_execute ?? false;

  return (
    <div className="w-full px-6 py-8">
      <div className="mx-auto w-full max-w-6xl space-y-8">
        <div className="flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
          <div className="min-w-0">
            <h1 className="text-3xl font-bold leading-tight text-text-primary">
              审计与安全检查
            </h1>
            <p className="mt-2 text-sm leading-6 text-text-secondary">
              查看安全检查结果和任务历史
            </p>
          </div>
          <div className="flex shrink-0 gap-3">
            <Button variant="secondary" onClick={handleRefresh} icon={<RefreshCw className="h-4 w-4" />}>
              刷新
            </Button>
            <Button variant="secondary" icon={<Download className="h-4 w-4" />}>
              导出报告
            </Button>
          </div>
        </div>

        <SafetyOverviewSection safetyChecks={safetyChecks} canExecute={canExecute} />
        <SafetyDetailsSection safetyChecks={safetyChecks} canExecute={canExecute} />
        <TaskHistorySection tasks={tasks} onViewLogs={handleViewLogs} />

        {selectedTaskId && (
          <AuditLogSection
            taskId={selectedTaskId}
            auditLogs={auditLogs}
            onClose={() => setSelectedTaskId(null)}
          />
        )}

        {isLoading && (
          <div className="flex justify-center py-12">
            <LoadingSpinner size="lg" text="加载中..." />
          </div>
        )}
      </div>
    </div>
  );
}
