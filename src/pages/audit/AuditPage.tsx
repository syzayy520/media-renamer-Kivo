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
    <div className="w-full px-4 py-8">
      <div className="w-full max-w-6xl mx-auto">
        <div className="flex items-center justify-between mb-8">
          <div>
            <h1 className="text-3xl font-bold text-text-primary">
              审计与安全检查
            </h1>
            <p className="text-text-secondary mt-2">
              查看安全检查结果和任务历史
            </p>
          </div>
          <div className="flex gap-4">
            <Button variant="secondary" onClick={handleRefresh} icon={<RefreshCw className="w-4 h-4" />}>
              刷新
            </Button>
            <Button variant="secondary" icon={<Download className="w-4 h-4" />}>
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