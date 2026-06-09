// features/settings/components/TemplateEditor.tsx — 模板编辑器
// 职责：展示和编辑单个重命名模板

import { useState } from 'react';
import type { RenameRule } from '../../../api/config/types';

interface TemplateEditorProps {
  rule: RenameRule;
  onSave: (mediaType: string, template: string) => Promise<void>;
}

const typeLabels: Record<string, string> = {
  Movie: '电影',
  Series: '剧集',
  Anime: '动漫',
  Special: '特别篇',
  Ova: 'OVA',
  Ncop: 'NCOP',
  Nced: 'NCED',
  Extras: '花絮',
  Unknown: '未识别',
};

export function TemplateEditor({ rule, onSave }: TemplateEditorProps) {
  const [value, setValue] = useState(rule.template);
  const isDirty = value !== rule.template;

  const handleSave = async () => {
    const key = rule.media_type.toLowerCase();
    await onSave(key, value);
  };

  return (
    <div className="rounded border border-white/5 bg-white/[0.02] p-3">
      <div className="mb-2 text-xs text-white/40">
        {typeLabels[rule.media_type] ?? rule.media_type}
      </div>
      <div className="flex gap-2">
        <input
          type="text"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          className="min-w-0 flex-1 rounded border border-white/10 bg-white/5 px-3 py-1.5 font-mono text-xs text-white outline-none transition-colors focus:border-blue-500/50"
        />
        <button
          type="button"
          disabled={!isDirty}
          onClick={handleSave}
          className={`shrink-0 rounded px-4 py-1.5 text-xs font-medium transition-colors ${
            isDirty
              ? 'bg-blue-600 text-white hover:bg-blue-500'
              : 'cursor-not-allowed bg-white/5 text-white/20'
          }`}
        >
          保存
        </button>
      </div>
    </div>
  );
}
