import { Check } from 'lucide-react';

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
  return (
    <div className="flex flex-col gap-1 max-h-48 overflow-y-auto">
      {presets.map((p) => (
        <button
          key={p.id}
          className={`flex items-center gap-2 px-2 py-1.5 rounded-md text-left text-xs transition-colors ${selectedId === p.id ? 'bg-primary/10 text-primary' : 'hover:bg-surface-hover text-text-secondary'}`}
          onClick={() => onSelect(p.id)}
        >
          {selectedId === p.id ? <Check className="h-3 w-3 shrink-0" /> : <span className="w-3 shrink-0" />}
          <div className="min-w-0">
            <div className="font-medium truncate">{p.label}</div>
            <div className="text-text-tertiary truncate">{p.desc}</div>
          </div>
        </button>
      ))}
    </div>
  );
}
