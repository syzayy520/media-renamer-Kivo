import type { NamingRule, TitleStrategy } from '../../../../types';
import { buildNamingPresetRule, getNamingPresetStrategy, NAMING_PRESETS } from '../namingPresetRules';

export interface NamingPresetApplyResult {
  presetId: string;
  rule: NamingRule;
  strategy: TitleStrategy;
}

export function applyNamingPreset(presetId: string): NamingPresetApplyResult {
  return {
    presetId,
    rule: buildNamingPresetRule(presetId),
    strategy: getNamingPresetStrategy(presetId),
  };
}

export function namingPresetListItems() {
  return NAMING_PRESETS.map(({ id, label, desc }) => ({ id, label, desc }));
}
