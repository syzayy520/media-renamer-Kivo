// features/settings/components/SettingsSaveStatus.tsx — 保存状态
// 职责：展示保存成功/失败状态

import { Badge } from '../../../shared/ui/Badge';

interface SettingsSaveStatusProps {
  status: 'idle' | 'saving' | 'saved' | 'error';
  message: string;
  onDismiss: () => void;
}

export function SettingsSaveStatus({
  status,
  message,
  onDismiss,
}: SettingsSaveStatusProps) {
  if (status === 'idle') return null;

  if (status === 'saving') {
    return (
      <div className="rounded-lg border border-blue-500/20 bg-blue-500/5 p-3">
        <span className="text-sm text-blue-400">保存中...</span>
      </div>
    );
  }

  if (status === 'saved') {
    return (
      <div className="flex items-center justify-between rounded-lg border border-emerald-500/20 bg-emerald-500/5 p-3">
        <div className="flex items-center gap-2">
          <Badge variant="success">已保存</Badge>
          {message && <span className="text-xs text-emerald-400/80">{message}</span>}
        </div>
        <button
          type="button"
          onClick={onDismiss}
          className="text-xs text-emerald-400/60 hover:text-emerald-400"
        >
          关闭
        </button>
      </div>
    );
  }

  // error
  return (
    <div className="flex items-center justify-between rounded-lg border border-red-500/20 bg-red-500/5 p-3">
      <div className="flex items-center gap-2">
        <Badge variant="danger">保存失败</Badge>
        {message && <span className="text-xs text-red-400/80">{message}</span>}
      </div>
      <button
        type="button"
        onClick={onDismiss}
        className="text-xs text-red-400/60 hover:text-red-400"
      >
        关闭
      </button>
    </div>
  );
}
