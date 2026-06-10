import { useState, useCallback } from 'react';
import { Settings } from 'lucide-react';
import type { NamingRule, TitleStrategy } from '../../../types';
import { NamingPresetList } from './NamingPresetList';
import { NamingTokenOrderList } from './NamingTokenOrderList';
import { NamingTemplatePreview } from './NamingTemplatePreview';

export interface NamingRulePanelProps {
  currentRule: NamingRule;
  currentStrategy: TitleStrategy;
  onRuleChange: (rule: NamingRule) => void;
  onStrategyChange: (strategy: TitleStrategy) => void;
  previewName: string;
}

const PRESETS: Array<{ id: string; label: string; desc: string }> = [
  { id: 'clean-library', label: '清爽媒体库', desc: '中文标题 (年份).mkv' },
  { id: 'bilingual-library', label: '中英双语', desc: '中文 - 英文 (年份).mkv' },
  { id: 'pt-preserve', label: 'PT原样保留', desc: 'First.Blood.1982.1080p.BluRay.mkv' },
  { id: 'chinese-prefix-pt', label: '中文前缀+PT', desc: '中文.PT技术信息.mkv' },
  { id: 'bt-friendly', label: 'BT友好', desc: '中文.英文.年份.分辨率.mkv' },
  { id: 'jellyfin-emby', label: 'Jellyfin/Emby', desc: '中文标题 (年份).mkv' },
  { id: 'tmdb-id-friendly', label: 'TMDb ID友好', desc: '中文 (年份) [tmdb-xxx].mkv' },
];

const STRATEGIES: Array<{ id: TitleStrategy; label: string }> = [
  { id: 'ChineseOnly', label: '仅中文' },
  { id: 'EnglishOnly', label: '仅英文' },
  { id: 'Bilingual', label: '中英双语' },
  { id: 'ChinesePrefixPt', label: '中文前缀+PT' },
  { id: 'ChineseFolderPtFile', label: '中文文件夹+PT文件' },
  { id: 'ChineseFolderChinesePrefixPtFile', label: '中文文件夹+中文前缀PT' },
];

export function NamingRulePanel({ currentRule, currentStrategy, onRuleChange, onStrategyChange, previewName }: NamingRulePanelProps) {
  const [selectedPreset, setSelectedPreset] = useState('clean-library');

  const handlePresetSelect = useCallback((presetId: string) => {
    setSelectedPreset(presetId);
  }, []);

  return (
    <div className="rounded-xl border border-border bg-surface p-4">
      <div className="flex items-center gap-2 mb-3">
        <Settings className="h-4 w-4 text-text-secondary" />
        <h3 className="text-sm font-semibold text-text-primary">命名规则</h3>
      </div>

      {/* Three-column layout */}
      <div className="grid grid-cols-3 gap-4">
        {/* Left: Preset List */}
        <div className="border-r border-border pr-3">
          <label className="text-xs text-text-secondary mb-2 block">预设</label>
          <NamingPresetList
            presets={PRESETS}
            selectedId={selectedPreset}
            onSelect={handlePresetSelect}
          />
        </div>

        {/* Center: Token Order */}
        <div>
          <label className="text-xs text-text-secondary mb-2 block">Token 顺序</label>
          <NamingTokenOrderList
            tokens={currentRule.tokens}
            onReorder={(tokens) => onRuleChange({ ...currentRule, tokens })}
            onRemove={(idx) => {
              const newTokens = currentRule.tokens.filter((_, i) => i !== idx);
              onRuleChange({ ...currentRule, tokens: newTokens });
            }}
          />
        </div>

        {/* Right: Preview + Strategy */}
        <div className="border-l border-border pl-3">
          <label className="text-xs text-text-secondary mb-2 block">标题策略</label>
          <div className="flex flex-wrap gap-1 mb-3">
            {STRATEGIES.map((s) => (
              <button
                key={s.id}
                className={`text-xs px-2 py-1 rounded-full transition-colors ${currentStrategy === s.id ? 'bg-primary text-white' : 'bg-surface-hover text-text-secondary hover:text-text-primary'}`}
                onClick={() => onStrategyChange(s.id)}
              >
                {s.label}
              </button>
            ))}
          </div>

          <label className="text-xs text-text-secondary mb-1 block">预览</label>
          <NamingTemplatePreview name={previewName} rule={currentRule} />
        </div>
      </div>
    </div>
  );
}
