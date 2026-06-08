// features/settings/page/SettingsPage 模块 - 设置页面 placeholder
// 职责：占位页面，后续实现模板管理、阈值配置

import { Card, CardTitle, CardDescription } from '../../../shared/ui/Card';

export function SettingsPage() {
  return (
    <div className="space-y-6">
      <Card>
        <CardTitle>设置</CardTitle>
        <CardDescription>
          管理重命名模板、置信度阈值、API Key 配置。
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
