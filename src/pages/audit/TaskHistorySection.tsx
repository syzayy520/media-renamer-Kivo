import { Clock, FileText, CheckCircle, XCircle, RefreshCw } from 'lucide-react';
import { Button, Card, CardHeader, CardTitle, CardDescription, CardContent, Badge, Table, TableHeader, TableBody, TableRow, TableHead, TableCell, EmptyState } from '../../components/ui';
import type { RenameTask } from '../../types';

interface TaskHistorySectionProps {
  tasks: RenameTask[];
  onViewLogs: (taskId: string) => void;
}

function getStatusIcon(status: string) {
  switch (status) {
    case 'Completed':
      return <CheckCircle className="w-5 h-5 text-success" />;
    case 'Failed':
      return <XCircle className="w-5 h-5 text-danger" />;
    case 'Executing':
      return <RefreshCw className="w-5 h-5 text-accent animate-spin" />;
    case 'Previewing':
      return <Clock className="w-5 h-5 text-warning" />;
    default:
      return <Clock className="w-5 h-5 text-text-secondary" />;
  }
}

function getStatusBadge(status: string) {
  switch (status) {
    case 'Completed':
      return <Badge variant="success">已完成</Badge>;
    case 'Failed':
      return <Badge variant="danger">失败</Badge>;
    case 'Executing':
      return <Badge variant="info">执行中</Badge>;
    case 'Previewing':
      return <Badge variant="warning">预览中</Badge>;
    case 'Ready':
      return <Badge variant="info">就绪</Badge>;
    case 'Cancelled':
      return <Badge variant="default">已取消</Badge>;
    default:
      return <Badge variant="default">{status}</Badge>;
  }
}

export function TaskHistorySection({ tasks, onViewLogs }: TaskHistorySectionProps) {
  return (
    <Card variant="elevated">
      <CardHeader>
        <CardTitle>任务历史</CardTitle>
        <CardDescription>
          查看历史重命名任务和审计日志
        </CardDescription>
      </CardHeader>
      <CardContent>
        {tasks.length === 0 ? (
          <EmptyState
            icon={<Clock className="w-12 h-12" />}
            title="暂无任务历史"
            description="执行重命名操作后，任务历史将显示在这里"
          />
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>任务ID</TableHead>
                <TableHead>状态</TableHead>
                <TableHead>文件数</TableHead>
                <TableHead>创建时间</TableHead>
                <TableHead>操作</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {tasks.map((task) => (
                <TableRow key={task.id}>
                  <TableCell>
                    <div className="font-mono text-sm truncate max-w-[120px]">
                      {task.id}
                    </div>
                  </TableCell>
                  <TableCell>
                    <div className="flex items-center gap-2">
                      {getStatusIcon(task.status)}
                      {getStatusBadge(task.status)}
                    </div>
                  </TableCell>
                  <TableCell>{task.total_files}</TableCell>
                  <TableCell>
                    {new Date(task.created_at).toLocaleString()}
                  </TableCell>
                  <TableCell>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => onViewLogs(task.id)}
                      icon={<FileText className="w-4 h-4" />}
                    >
                      日志
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </CardContent>
    </Card>
  );
}