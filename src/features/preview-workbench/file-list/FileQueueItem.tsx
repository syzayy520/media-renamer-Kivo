// features/preview-workbench/file-list/FileQueueItem.tsx — 单个文件行
// 职责：渲染文件名 + 状态标签 + 选中高亮

import type { RenamePreviewItem } from '../../../api/session/types';

interface FileQueueItemProps {
  item: RenamePreviewItem;
  isSelected: boolean;
  onSelect: () => void;
}

export function FileQueueItem({ item, isSelected, onSelect }: FileQueueItemProps) {
  // 判断状态
  const hasConflict = item.conflicts.length > 0;
  const needsReview = item.needs_manual_review;
  const isSafe = !hasConflict && !needsReview;

  // 状态标签配置
  let statusLabel = '';
  let statusColor = '';
  if (hasConflict) {
    statusLabel = '冲突';
    statusColor = 'text-red-400 bg-red-500/10';
  } else if (needsReview) {
    statusLabel = '待审';
    statusColor = 'text-amber-400 bg-amber-500/10';
  } else if (isSafe) {
    statusLabel = '安全';
    statusColor = 'text-green-400 bg-green-500/10';
  }

  // 媒体类型图标
  const mediaIcon = getMediaTypeIcon(item.media_type);

  return (
    <button
      onClick={onSelect}
      className={`w-full rounded-lg px-3 py-2 text-left transition-colors ${
        isSelected
          ? 'bg-blue-500/20 ring-1 ring-blue-500/50'
          : 'hover:bg-white/5'
      }`}
    >
      <div className="flex items-center gap-2">
        <span className="text-lg">{mediaIcon}</span>
        <div className="min-w-0 flex-1">
          <div className="truncate text-sm font-medium text-white/90">
            {item.original_name}
          </div>
          <div className="mt-0.5 flex items-center gap-2">
            <span className={`rounded-full px-1.5 py-0.5 text-xs ${statusColor}`}>
              {statusLabel}
            </span>
            {item.confidence > 0 && (
              <span className="text-xs text-white/30">
                置信度 {(item.confidence * 100).toFixed(0)}%
              </span>
            )}
          </div>
        </div>
      </div>
    </button>
  );
}

function getMediaTypeIcon(mediaType: string): string {
  switch (mediaType) {
    case 'Movie':
      return '🎬';
    case 'Series':
      return '📺';
    case 'Anime':
      return '🎭';
    case 'Special':
      return '⭐';
    case 'Ova':
      return '🎞️';
    case 'Ncop':
      return '🎵';
    case 'Nced':
      return '🎶';
    case 'Extras':
      return '📦';
    default:
      return '❓';
  }
}
