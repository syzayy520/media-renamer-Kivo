import { Check } from 'lucide-react';
import { NAMING_PRESETS } from './namingPresetRules';

export interface Preset {
  id: string;
  label: string;
  desc: string;
}

export interface NamingPresetListProps {
  presets: Preset[];
  selectedId: string;
  onSelect: (id: string) => void;
}

export function NamingPresetList({ presets, selectedId, onSelect }: NamingPresetListProps) {
  const displayPresets = mergeWithCanonicalPresets(presets);

  return (
    <div className="flex max-h-64 flex-col gap-1 overflow-y-auto">
      {displayPresets.map((p) => (
        <button
          key={p.id}
          className={`flex items-center gap-2 rounded-md px-2 py-1.5 text-left text-xs transition-colors ${selectedId === p.id ? 'bg-primary/10 text-primary' : 'text-text-secondary hover:bg-surface-hover'}`}
          onClick={() => onSelect(p.id)}
        >
          {selectedId === p.id ? <Check className="h-3 w-3 shrink-0" /> : <span className="w-3 shrink-0" />}
          <div className="min-w-0">
            <div className="truncate font-medium">{p.label}</div>
            <div className="truncate text-text-tertiary">{p.desc}</div>
          </div>
        </button>
      ))}
    </div>
  );
}

function mergeWithCanonicalPresets(presets: Preset[]): Preset[] {
  const existing = new Set(presets.map((preset) => preset.id));
  const missingCanonical = NAMING_PRESETS
    .filter((preset) => !existing.has(preset.id))
    .map(({ id, label, desc }) => ({ id, label, desc }));

  return [...presets, ...missingCanonical];
}
