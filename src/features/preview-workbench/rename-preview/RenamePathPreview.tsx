// features/preview-workbench/rename-preview/RenamePathPreview.tsx — 路径预览
// 职责：展示原路径和目标路径对比

import type { RenamePreviewItem } from '../../../api/session/types';

interface RenamePathPreviewProps {
  item: RenamePreviewItem;
}

export function RenamePathPreview({ item }: RenamePathPreviewProps) {
  return (
    <div className="mb-3">
      <div className="mb-1 text-xs text-white/40">路径预览</div>
      <div className="rounded bg-white/5 p-3 text-xs">
        {/* 原路径 */}
        <div className="mb-2">
          <span className="text-white/30">原路径: </span>
          <span className="text-white/60 break-all">
            {item.source_path ?? '(未知)'}
          </span>
        </div>
        {/* 目标路径 */}
        <div>
          <span className="text-white/30">目标路径: </span>
          <span className="text-blue-300/80 break-all">
            {item.target_path ?? '(未生成)'}
          </span>
        </div>
      </div>
    </div>
  );
}
