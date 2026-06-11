import type { FolderPolicyConfig, NamingRule, TitleStrategy } from '../../../../types';
import { NamingWorkbench } from './NamingWorkbench';
import {
  applyPresetToWorkbench,
  applyRuleToWorkbench,
  applyStrategyToWorkbench,
  type NamingWorkbenchStateSetters,
} from './NamingWorkbenchBridge';

interface PreviewNamingWorkbenchPanelProps extends NamingWorkbenchStateSetters {
  selectedPreset: string;
  currentNamingRule: NamingRule;
  currentStrategy: TitleStrategy;
  folderPolicyConfig: FolderPolicyConfig;
  onFolderPolicyChange: (config: FolderPolicyConfig) => void;
}

export function PreviewNamingWorkbenchPanel({
  selectedPreset,
  currentNamingRule,
  currentStrategy,
  folderPolicyConfig,
  setSelectedPreset,
  setCurrentNamingRule,
  setCurrentStrategy,
  triggerNamingApply,
  onFolderPolicyChange,
}: PreviewNamingWorkbenchPanelProps) {
  const workbenchSetters: NamingWorkbenchStateSetters = {
    setSelectedPreset,
    setCurrentNamingRule,
    setCurrentStrategy,
    triggerNamingApply,
  };

  return (
    <NamingWorkbench
      selectedPreset={selectedPreset}
      currentNamingRule={currentNamingRule}
      currentStrategy={currentStrategy}
      folderPolicyConfig={folderPolicyConfig}
      onPresetApply={(presetId, rule, strategy) => applyPresetToWorkbench({ presetId, rule, strategy }, workbenchSetters)}
      onStrategyChange={(strategy) => applyStrategyToWorkbench(strategy, currentNamingRule, workbenchSetters)}
      onRuleChange={(rule) => applyRuleToWorkbench(rule, currentStrategy, workbenchSetters)}
      onFolderPolicyChange={onFolderPolicyChange}
    />
  );
}
