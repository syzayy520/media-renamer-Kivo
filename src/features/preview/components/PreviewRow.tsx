// features/preview/components/PreviewRow.tsx — 预览表格行
// 职责：单条预览记录展示

import type { RenamePreviewItem } from '../../../api/session/types';
import { PreviewStatusBadge } from './PreviewStatusBadge';

interface PreviewRowProps {
  index: number;
  item: RenamePreviewItem;
}

function confidenceColor(value: number): string {
  if (value >= 80) return 'text-emerald-400';
  if (value >= 60) return 'text-amber-400';
  return 'text-red-400';
}

const typeLabels: Record<string, string> = {
  Movie: '电影',
  Series: '剧集',
  Anime: '动漫',
  Special: '特别篇',
  Ova: 'OVA',
  Unknown: '未知',
};

export function PreviewRow({ index, item }: PreviewRowProps) {
  return (
    <tr className="border-b border-white/5 transition-colors hover:bg-white/[0.02]">
      <td className="whitespace-nowrap px-4 py-2 text-xs text-white/30">{index + 1}</td>
      <td className="max-w-[300px] truncate px-4 py-2 font-mono text-xs text-white/50">
        {item.original_name}
      </td>
      <td className="max-w-[300px] truncate px-4 py-2 font-mono text-xs text-blue-400">
        {item.proposed_name}
      </td>
      <td className="whitespace-nowrap px-4 py-2 text-xs text-white/50">
        {typeLabels[item.media_type] ?? item.media_type}
      </td>
      <td className="whitespace-nowrap px-4 py-2 text-xs">
        <span className={confidenceColor(item.confidence)}>{item.confidence}%</span>
      </td>
      <td className="whitespace-nowrap px-4 py-2">
        <PreviewStatusBadge item={item} />
      </td>
    </tr>
  );
}
