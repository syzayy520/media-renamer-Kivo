// features/audit/page/AuditPage 模块 - 审计页面 placeholder
// 职责：占位页面，后续实现任务列表、任务详情、审计日志

import { Card, CardTitle, CardDescription } from '../../../shared/ui/Card';

export function AuditPage() {
  return (
    <div className="space-y-6">
      <Card>
        <CardTitle>审计日志</CardTitle>
        <CardDescription>
          查看历史任务记录和审计日志，追踪重命名操作。
        </CardDescription>
      </Card>

      <div className="rounded-lg border border-amber-500/20 bg-amber-500/5 px-4 py-3">
        <p className="text-sm text-amber-400">
          当前仅为 Dry-run Preview，不会真实修改媒体文件。
        </p>
      </div>
    </div>
  );
}
