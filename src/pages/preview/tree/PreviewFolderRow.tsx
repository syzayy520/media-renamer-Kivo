import { ChevronRight, ChevronDown, Globe, Edit3, SkipForward } from 'lucide-react';
import type { FolderGroup, GroupStatus } from '../../../types';
import { Button, Badge } from '../../../components/ui';
import { PreviewFileRow } from './PreviewFileRow';

export interface PreviewFolderRowProps {
  group: FolderGroup;
  expanded: boolean;
  onToggleExpand: () => void;
  onTmdbSearch: () => void;
  onEdit: () => void;
  onSkip: () => void;
  onFileEdit: (fileId: string) => void;
  onFileSkip: (fileId: string) => void;
}

function statusVariant(status: GroupStatus): 'success' | 'warning' | 'danger' | 'default' {
  switch (status) {
    case 'Ready': return 'success';
    case 'PartialSkipped': return 'warning';
    case 'NeedsReview': return 'warning';
    case 'Conflict': return 'danger';
    case 'Skipped': return 'default';
    case 'Blocker': return 'danger';
    default: return 'default';
  }
}

function statusLabel(status: GroupStatus): string {
  const map: Record<GroupStatus, string> = {
    Ready: '就绪', NeedsReview: '需审核', Conflict: '冲突',
    Skipped: '已跳过', Blocker: '阻断', PartialSkipped: '部分跳过',
  };
  return map[status] || status;
}

function mediaTypeIcon(type: string): string {
  const map: Record<string, string> = { Movie: '🎬', Tv: '📺', Anime: '🌸', Mixed: '📦', Unknown: '📁' };
  return map[type] || '📁';
}

export function PreviewFolderRow({
  group,
  expanded,
  onToggleExpand,
  onTmdbSearch,
  onEdit,
  onSkip,
  onFileEdit,
  onFileSkip,
}: PreviewFolderRowProps) {
  const hasBlocker = group.status === 'Blocker';

  return (
    <div className="border border-border rounded-lg overflow-hidden">
      {/* Folder Row */}
      <div
        className={`flex items-center gap-2 px-3 py-2.5 cursor-pointer hover:bg-surface-hover transition-colors ${hasBlocker ? 'bg-error/5 border-b border-error/20' : ''} ${group.should_skip ? 'opacity-50' : ''}`}
        onClick={onToggleExpand}
      >
        <button className="shrink-0 text-text-secondary hover:text-text-primary transition-colors">
          {expanded ? <ChevronDown className="h-3.5 w-3.5" /> : <ChevronRight className="h-3.5 w-3.5" />}
        </button>

        <span className="text-sm shrink-0">{mediaTypeIcon(group.media_type)}</span>

        <div className="min-w-0 flex-1 flex items-center gap-2">
          <span className="text-sm font-medium text-text-primary truncate" title={group.target_folder_name}>
            {group.target_folder_name}
          </span>
          <span className="text-xs text-text-secondary shrink-0">
            {group.media_type === 'Movie' ? 'Movie' : group.media_type === 'Tv' ? 'TV' : group.media_type}
          </span>
          <span className="text-xs text-text-tertiary shrink-0">· {group.file_count} files</span>
        </div>

        <Badge variant={statusVariant(group.status)} size="sm">{statusLabel(group.status)}</Badge>

        <div className="flex items-center gap-1 shrink-0" onClick={(e) => e.stopPropagation()}>
          <Button variant="ghost" size="sm" onClick={onTmdbSearch} title="TMDb 搜索" disabled={group.should_skip}>
            <Globe className="h-3.5 w-3.5" />
          </Button>
          <Button variant="ghost" size="sm" onClick={onEdit} title="编辑文件夹名" disabled={group.should_skip}>
            <Edit3 className="h-3.5 w-3.5" />
          </Button>
          <Button variant="ghost" size="sm" onClick={onSkip} title={group.should_skip ? '取消跳过' : '跳过整组'}>
            <SkipForward className="h-3.5 w-3.5" />
          </Button>
        </div>
      </div>

      {/* Path info */}
      <div className="px-3 py-1.5 border-b border-border bg-surface-subtle">
        <div className="flex gap-3 text-xs text-text-tertiary">
          <span>原目录：{group.original_path}</span>
          <span className="text-text-secondary">→</span>
          <span>新目录：{group.target_path}</span>
        </div>
      </div>

      {/* Children */}
      {expanded && group.children.length > 0 && (
        <div className="border-t border-border bg-surface-subtler">
          {group.children.map((child) => (
            <PreviewFileRow
              key={child.id}
              file={child}
              onEdit={() => onFileEdit(child.id)}
              onSkip={() => onFileSkip(child.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
