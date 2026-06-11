import { useCallback, useEffect, useRef, useState } from 'react';
import type { FolderPolicyConfig, NamingRule, TitleStrategy } from '../../../../types';
import { buildInitialNamingWorkbenchState } from './NamingWorkbenchBridge';

interface UsePreviewNamingWorkbenchParams {
  onApplyNamingRule: (rule: NamingRule, strategy: TitleStrategy) => void;
  onApplyFolderPolicy: (policy: FolderPolicyConfig['policy']) => void;
}

const INITIAL_NAMING_WORKBENCH = buildInitialNamingWorkbenchState();

export function usePreviewNamingWorkbench({
  onApplyNamingRule,
  onApplyFolderPolicy,
}: UsePreviewNamingWorkbenchParams) {
  const [currentNamingRule, setCurrentNamingRule] = useState<NamingRule>(INITIAL_NAMING_WORKBENCH.currentNamingRule);
  const [currentStrategy, setCurrentStrategy] = useState<TitleStrategy>(INITIAL_NAMING_WORKBENCH.currentStrategy);
  const [selectedPreset, setSelectedPreset] = useState(INITIAL_NAMING_WORKBENCH.selectedPreset);
  const [folderPolicyConfig, setFolderPolicyConfig] = useState<FolderPolicyConfig>({
    policy: 'KeepOriginalStructure',
    clean_empty_folders_after: false,
  });
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, []);

  const triggerNamingApply = useCallback((rule: NamingRule, strategy: TitleStrategy) => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => onApplyNamingRule(rule, strategy), 150);
  }, [onApplyNamingRule]);

  const handleFolderPolicyChange = useCallback((config: FolderPolicyConfig) => {
    setFolderPolicyConfig(config);
    onApplyFolderPolicy(config.policy);
  }, [onApplyFolderPolicy]);

  return {
    selectedPreset,
    currentNamingRule,
    currentStrategy,
    folderPolicyConfig,
    setSelectedPreset,
    setCurrentNamingRule,
    setCurrentStrategy,
    triggerNamingApply,
    handleFolderPolicyChange,
  };
}
