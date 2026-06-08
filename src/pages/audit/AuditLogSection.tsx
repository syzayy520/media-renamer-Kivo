import { FileText, XCircle } from 'lucide-react';
import { Button, Card, CardHeader, CardTitle, CardDescription, CardContent, Badge, EmptyState } from '../../components/ui';
import type { AuditLogEntry } from '../../types';

interface AuditLogSectionProps {
  taskId: string;
  auditLogs: AuditLogEntry[];
  onClose: () => void;
}

function getEventTypeBadge(eventType: string) {
  switch (eventType) {
    case 'task_created':
      return <Badge variant="info">任务创建</Badge>;
    case 'preview':
      return <Badge variant="warning">预览生成</Badge>;
    case 'execution_plan':
      return <Badge variant="info">执行计划</Badge>;
    case 'failure':
      return <Badge variant="danger">失败</Badge>;
    case 'system':
      return <Badge variant="default">系统</Badge>;
    default:
      return <Badge variant="default">{eventType}</Badge>;
  }
}

export function AuditLogSection({ taskId, auditLogs, onClose }: AuditLogSectionProps) {
  return (
    <Card variant="elevated" className="mt-8">
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>审计日志</CardTitle>
            <CardDescription>
              任务 {taskId} 的详细日志
            </CardDescription>
          </div>
          <Button
            variant="ghost"
            size="sm"
            onClick={onClose}
            icon={<XCircle className="w-4 h-4" />}
          >
            关闭
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        {auditLogs.length === 0 ? (
          <EmptyState
            icon={<FileText className="w-12 h-12" />}
            title="暂无日志"
            description="该任务暂无审计日志记录"
          />
        ) : (
          <div className="space-y-3">
            {auditLogs.map((log) => (
              <div
                key={log.id}
                className="p-3 bg-bg-secondary rounded-lg"
              >
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center gap-2">
                    {getEventTypeBadge(log.event_type)}
                    <span className="text-sm text-text-secondary">
                      {new Date(log.created_at).toLocaleString()}
                    </span>
                  </div>
                </div>
                <p className="text-sm text-text-primary">{log.message}</p>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  );
}