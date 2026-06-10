import { Edit, Search, SkipForward } from 'lucide-react';
import { Button, Tooltip } from '../../components/ui';
import type { RenamePreviewItem } from '../../types';

interface PreviewRowActionsProps {
  item: RenamePreviewItem;
  tmdbEnabled: boolean;
  onTmdbSearch?: (item: RenamePreviewItem) => void;
  onManualEdit?: (item: RenamePreviewItem) => void;
  onSkipToggle?: (item: RenamePreviewItem) => void;
}

export function PreviewRowActions({
  item,
  tmdbEnabled,
  onTmdbSearch,
  onManualEdit,
  onSkipToggle,
}: PreviewRowActionsProps) {
  return (
    <div className="flex justify-end gap-2">
      {tmdbEnabled && (
        <Tooltip content="TMDb 搜索">
          <Button variant="ghost" size="sm" onClick={() => onTmdbSearch?.(item)}>
            <Search className="h-4 w-4" />
          </Button>
        </Tooltip>
      )}
      <Tooltip content="编辑文件名">
        <Button variant="ghost" size="sm" onClick={() => onManualEdit?.(item)}>
          <Edit className="h-4 w-4" />
        </Button>
      </Tooltip>
      {onSkipToggle && (
        <Tooltip content={item.should_skip ? '取消跳过' : '跳过此文件'}>
          <Button variant="ghost" size="sm" onClick={() => onSkipToggle(item)}>
            <SkipForward className="h-4 w-4" />
          </Button>
        </Tooltip>
      )}
    </div>
  );
}
