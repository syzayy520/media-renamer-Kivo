// features/settings/components/ApiKeyStatusPanel.tsx — API Key 状态面板
// 职责：只读显示 API key 配置状态，不显示明文

import { Badge } from '../../../shared/ui/Badge';

export function ApiKeyStatusPanel() {
  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-4">
      <h3 className="text-sm font-medium text-white/70">API Key</h3>
      <p className="mt-0.5 mb-3 text-xs text-white/40">
        API key 用于获取在线元数据（例如 TMDb）。管理功能暂不在本 UI 开放。
      </p>
      <div className="flex items-center gap-3">
        <Badge variant="info">Not Configured</Badge>
        <span className="text-xs text-white/30">
          不显示明文 key
        </span>
      </div>
    </div>
  );
}
