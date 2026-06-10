import { Folder, AlertTriangle } from 'lucide-react';
import type { FolderPolicyConfig } from '../../../types';
import { FolderPolicySelector } from './FolderPolicySelector';

export interface FolderPolicyPanelProps {
  config: FolderPolicyConfig;
  onConfigChange: (config: FolderPolicyConfig) => void;
}

export function FolderPolicyPanel({ config, onConfigChange }: FolderPolicyPanelProps) {
  const isHighRisk = config.policy === 'Flatten';

  return (
    <div className="rounded-xl border border-border bg-surface p-4">
      <div className="flex items-center gap-2 mb-3">
        <Folder className="h-4 w-4 text-text-secondary" />
        <h3 className="text-sm font-semibold text-text-primary">文件夹策略</h3>
      </div>

      <FolderPolicySelector
        selected={config.policy}
        onSelect={(policy) => onConfigChange({ ...config, policy })}
      />

      {isHighRisk && (
        <div className="mt-3 flex items-center gap-2 rounded-lg border border-error/30 bg-error/5 px-3 py-2">
          <AlertTriangle className="h-4 w-4 text-error shrink-0" />
          <div className="text-xs text-error">
            <strong>高风险操作：</strong>去除文件夹会将所有文件移动到上级目录。如有同名文件会产生冲突。请先在 Dry Run 模式下预览结果。
          </div>
        </div>
      )}

      <label className="flex items-center gap-2 mt-3 text-xs text-text-secondary cursor-pointer">
        <input
          type="checkbox"
          className="rounded border-border"
          checked={config.clean_empty_folders_after}
          onChange={(e) => onConfigChange({ ...config, clean_empty_folders_after: e.target.checked })}
        />
        执行后清理空文件夹（默认关闭）
      </label>
    </div>
  );
}
