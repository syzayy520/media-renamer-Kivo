import type { NamingRule, TitleStrategy } from '../../../../types';
import { applyNamingPreset } from '../preset/NamingPresetApply';

interface PresetApplyInput {
  presetId: string;
  rule: NamingRule;
  strategy: TitleStrategy;
}

export interface NamingWorkbenchInitialState {
  selectedPreset: string;
  currentNamingRule: NamingRule;
  currentStrategy: TitleStrategy;
}

export interface NamingWorkbenchStateSetters {
  setSelectedPreset: (presetId: string) => void;
  setCurrentNamingRule: (rule: NamingRule) => void;
  setCurrentStrategy: (strategy: TitleStrategy) => void;
  triggerNamingApply: (rule: NamingRule, strategy: TitleStrategy) => void;
}

export function buildInitialNamingWorkbenchState(presetId = 'clean-library'): NamingWorkbenchInitialState {
  const applied = applyNamingPreset(presetId);

  return {
    selectedPreset: applied.presetId,
    currentNamingRule: applied.rule,
    currentStrategy: applied.strategy,
  };
}

export function applyPresetToWorkbench(
  input: PresetApplyInput,
  setters: NamingWorkbenchStateSetters,
): void {
  setters.setSelectedPreset(input.presetId);
  setters.setCurrentNamingRule(input.rule);
  setters.setCurrentStrategy(input.strategy);
  setters.triggerNamingApply(input.rule, input.strategy);
}

export function applyStrategyToWorkbench(
  strategy: TitleStrategy,
  currentRule: NamingRule,
  setters: NamingWorkbenchStateSetters,
): void {
  setters.setCurrentStrategy(strategy);
  setters.triggerNamingApply(currentRule, strategy);
}

export function applyRuleToWorkbench(
  rule: NamingRule,
  strategy: TitleStrategy,
  setters: Pick<NamingWorkbenchStateSetters, 'setCurrentNamingRule' | 'triggerNamingApply'>,
): void {
  setters.setCurrentNamingRule(rule);
  setters.triggerNamingApply(rule, strategy);
}
