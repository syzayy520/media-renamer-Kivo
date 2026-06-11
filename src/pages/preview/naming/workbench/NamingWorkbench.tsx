import type { ReactNode } from 'react';
import { AlertTriangle, Edit3, Folder, Info, Settings } from 'lucide-react';
import type { FolderPolicy, FolderPolicyConfig, NamingRule, TitleStrategy } from '../../../../types';
import { FolderPolicySelector } from '../../folder-policy/FolderPolicySelector';
import { NamingPresetList } from '../NamingPresetList';
import { NamingTokenOrderList } from '../NamingTokenOrderList';
import { applyNamingPreset, namingPresetListItems } from '../preset/NamingPresetApply';

const STRATEGIES: Array<{ id: TitleStrategy; label: string }> = [
  { id: 'ChineseOnly', label: '仅中文' },
  { id: 'EnglishOnly', label: '仅英文' },
  { id: 'Bilingual', label: '中英双语' },
  { id: 'ChinesePrefixPt', label: '中文前缀+PT' },
  { id: 'ChineseFolderPtFile', label: '中文文件夹+PT文件' },
  { id: 'ChineseFolderChinesePrefixPtFile', label: '中文文件夹+中文前缀PT' },
];

interface NamingWorkbenchProps {
  selectedPreset: string;
  currentNamingRule: NamingRule;
  currentStrategy: TitleStrategy;
  folderPolicyConfig: FolderPolicyConfig;
  onPresetApply: (presetId: string, rule: NamingRule, strategy: TitleStrategy) => void;
  onStrategyChange: (strategy: TitleStrategy) => void;
  onRuleChange: (rule: NamingRule) => void;
  onFolderPolicyChange: (config: FolderPolicyConfig) => void;
}

export function NamingWorkbench({
  selectedPreset,
  currentNamingRule,
  currentStrategy,
  folderPolicyConfig,
  onPresetApply,
  onStrategyChange,
  onRuleChange,
  onFolderPolicyChange,
}: NamingWorkbenchProps) {
  const presets = namingPresetListItems();

  const handlePresetChange = (presetId: string) => {
    const applied = applyNamingPreset(presetId);
    onPresetApply(applied.presetId, applied.rule, applied.strategy);
  };

  const handleTokenReorder = (tokens: NamingRule['tokens']) => {
    onRuleChange({ ...currentNamingRule, tokens });
  };

  const handleTokenRemove = (idx: number) => {
    onRuleChange({
      ...currentNamingRule,
      tokens: currentNamingRule.tokens.filter((_, index) => index !== idx),
    });
  };

  return (
    <>
      <WorkbenchCard title="命名预设" icon={<Settings className="h-3.5 w-3.5 text-text-secondary" />}>
        <NamingPresetList presets={presets} selectedId={selectedPreset} onSelect={handlePresetChange} />
      </WorkbenchCard>

      <WorkbenchCard title="文件夹策略" icon={<Folder className="h-3.5 w-3.5 text-text-secondary" />}>
        <FolderPolicySelector
          selected={folderPolicyConfig.policy}
          onSelect={(policy: FolderPolicy) => onFolderPolicyChange({ ...folderPolicyConfig, policy })}
        />
        {folderPolicyConfig.policy === 'Flatten' && (
          <div className="mt-2 flex items-start gap-1.5 rounded-lg border border-error/30 bg-error/5 px-2 py-1.5">
            <AlertTriangle className="mt-0.5 h-3 w-3 shrink-0 text-error" />
            <span className="text-xs leading-relaxed text-error">高风险：将文件移至上级目录，同名文件会冲突。</span>
          </div>
        )}
        <label className="mt-2 flex cursor-pointer items-center gap-1.5 text-xs text-text-secondary">
          <input
            type="checkbox"
            className="rounded border-border"
            checked={folderPolicyConfig.clean_empty_folders_after}
            onChange={(event) => onFolderPolicyChange({ ...folderPolicyConfig, clean_empty_folders_after: event.target.checked })}
          />
          清理空文件夹
        </label>
      </WorkbenchCard>

      <WorkbenchCard title="标题策略" icon={<Info className="h-3.5 w-3.5 text-text-secondary" />}>
        <div className="flex flex-wrap gap-1">
          {STRATEGIES.map((strategy) => (
            <button
              key={strategy.id}
              className={`rounded-full px-2 py-1 text-xs transition-colors ${currentStrategy === strategy.id ? 'bg-primary text-white' : 'bg-surface-hover text-text-secondary hover:text-text-primary'}`}
              onClick={() => onStrategyChange(strategy.id)}
            >
              {strategy.label}
            </button>
          ))}
        </div>
      </WorkbenchCard>

      <WorkbenchCard title="Token 编辑" icon={<Edit3 className="h-3.5 w-3.5 text-text-secondary" />}>
        <NamingTokenOrderList tokens={currentNamingRule.tokens} onReorder={handleTokenReorder} onRemove={handleTokenRemove} />
      </WorkbenchCard>
    </>
  );
}

function WorkbenchCard({ title, icon, children }: { title: string; icon: ReactNode; children: ReactNode }) {
  return (
    <div className="rounded-xl border border-border bg-surface p-3">
      <div className="mb-2 flex items-center gap-1.5">
        {icon}
        <h3 className="text-xs font-semibold text-text-primary">{title}</h3>
      </div>
      {children}
    </div>
  );
}
