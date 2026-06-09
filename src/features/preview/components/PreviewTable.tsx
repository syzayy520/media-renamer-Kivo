// features/preview/components/PreviewTable.tsx — 预览表格
// 职责：展示过滤后的预览列表

import type { RenamePreviewItem } from '../../../api/session/types';
import { PreviewRow } from './PreviewRow';

interface PreviewTableProps {
  items: RenamePreviewItem[];
}

export function PreviewTable({ items }: PreviewTableProps) {
  if (items.length === 0) {
    return (
      <div className="rounded-lg border border-white/5 bg-white/[0.02] p-8 text-center">
        <p className="text-sm text-white/30">没有匹配的预览项</p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto rounded-lg border border-white/10">
      <table className="w-full table-auto border-collapse" aria-label="重命名预览列表">
        <thead>
          <tr className="border-b border-white/10 bg-white/[0.03]">
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              #
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              原文件名
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              新文件名
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              类型
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              置信度
            </th>
            <th className="whitespace-nowrap px-4 py-2 text-left text-[11px] font-medium uppercase tracking-wider text-white/30">
              状态
            </th>
          </tr>
        </thead>
        <tbody>
          {items.map((item, i) => (
            <PreviewRow key={item.id} index={i} item={item} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
