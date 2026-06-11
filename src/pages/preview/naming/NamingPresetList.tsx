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

const COMMERCIAL_PT_PRESETS: Preset[] = [
  {
    id: 'pt-0day-movie',
    label: 'PT 0day 电影',
    desc: 'First.Blood.1982.1080p.BluRay.x265.DTS.5.1-PTer.mkv',
  },
  {
    id: 'pt-0day-video-postfix',
    label: 'PT 外站视频后置',
    desc: 'First.Blood.1982.1080p.BluRay.DTS.5.1.x265-PTer.mkv',
  },
  {
    id: 'pt-original-release',
    label: 'PT 原始发布名',
    desc: '完整保留原始 PT/BT 发布名.mkv',
  },
];

export function NamingPresetList({ presets, selectedId, onSelect }: NamingPresetListProps) {
  const mergedPresets = mergePresets(presets);

  return (
    <div className="flex max-h-64 flex-col gap-1 overflow-y-auto">
      {mergedPresets.map((p) => (
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

function mergePresets(presets: Preset[]): Preset[] {
  const existing = new Set(presets.map((preset) => preset.id));
  return [
    ...presets,
    ...COMMERCIAL_PT_PRESETS.filter((preset) => !existing.has(preset.id)),
  ];
}
