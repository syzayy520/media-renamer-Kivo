import { Edit3, SkipForward } from 'lucide-react';
import type { PreviewFileItem } from '../../../types';
import { Button, Badge } from '../../../components/ui';

export interface PreviewFileRowProps {
  file: PreviewFileItem;
  onEdit: () => void;
  onSkip: () => void;
}

function fileIcon(role: string): string {
  const map: Record<string, string> = {
    MainVideo: '🎞', Subtitle: '📝', Image: '🖼', Nfo: '📄', Extra: '📦', Unknown: '❓',
  };
  return map[role] || '❓';
}

export function PreviewFileRow({ file, onEdit, onSkip }: PreviewFileRowProps) {
  return (
    <div className={`flex items-center gap-2 px-4 py-1.5 hover:bg-surface-hover transition-colors text-xs ${file.should_skip ? 'opacity-50' : ''}`}>
      <span className="shrink-0 ml-3 text-xs">{fileIcon(file.file_role)}</span>

      <div className="min-w-0 flex-1 flex items-center gap-2">
        <span className="text-text-primary truncate" title={file.target_name}>
          {file.target_name}
        </span>
        {file.subtitle_language && (
          <Badge variant="default" size="sm">{file.subtitle_language}</Badge>
        )}
      </div>

      <span className="text-text-tertiary shrink-0 w-12 text-right">{file.extension}</span>

      <div className="flex items-center gap-1 shrink-0">
        <Button variant="ghost" size="sm" onClick={onEdit} title="编辑文件名" disabled={file.should_skip}>
          <Edit3 className="h-3 w-3" />
        </Button>
        <Button variant="ghost" size="sm" onClick={onSkip} title={file.should_skip ? '取消跳过' : '跳过'}>
          <SkipForward className="h-3 w-3" />
        </Button>
      </div>
    </div>
  );
}
