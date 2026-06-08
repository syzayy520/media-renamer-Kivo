// app/shell/TopBar 模块 - 顶部栏组件
// 职责：顶部标题区域，显示当前页面标题和安全状态

import { Badge } from '../../shared/ui/Badge';
import { useLocation } from 'react-router-dom';

const pageLabels: Record<string, string> = {
  '/': '扫描',
  '/preview': '预览',
  '/safety': '安全检查',
  '/audit': '审计',
  '/settings': '设置',
};

export function TopBar() {
  const location = useLocation();
  const label = pageLabels[location.pathname] ?? '';

  return (
    <header className="flex h-14 shrink-0 items-center justify-between border-b border-white/10 px-6">
      <h1 className="text-lg font-semibold">{label}</h1>
      <div className="flex items-center gap-3">
        <Badge variant="info">Dry-run Only</Badge>
      </div>
    </header>
  );
}
