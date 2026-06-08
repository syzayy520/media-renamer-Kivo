// features/scan/page/ScanPage 模块 - 扫描页面 placeholder
// 职责：占位页面，后续实现选择目录 + 开始 dry-run 扫描

import { Card, CardTitle, CardDescription } from '../../../shared/ui/Card';

export function ScanPage() {
  return (
    <div className="space-y-6">
      <Card>
        <CardTitle>媒体文件扫描</CardTitle>
        <CardDescription>
          选择包含媒体文件的目录，系统将自动解析文件名并生成重命名预览。
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
