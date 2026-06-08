// features/preview/page/PreviewPage 模块 - 预览页面 placeholder
// 职责：占位页面，后续实现重命名预览表格

import { Card, CardTitle, CardDescription } from '../../../shared/ui/Card';

export function PreviewPage() {
  return (
    <div className="space-y-6">
      <Card>
        <CardTitle>重命名预览</CardTitle>
        <CardDescription>
          查看扫描结果，对比原文件名和新文件名，筛选需要处理的文件。
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
