// features/safety/page/SafetyPage 模块 - 安全检查页面 placeholder
// 职责：占位页面，后续实现安全检查报告

import { Card, CardTitle, CardDescription } from '../../../shared/ui/Card';

export function SafetyPage() {
  return (
    <div className="space-y-6">
      <Card>
        <CardTitle>安全检查</CardTitle>
        <CardDescription>
          查看安全检查结果：置信度、冲突检测、人工审核需求。
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
