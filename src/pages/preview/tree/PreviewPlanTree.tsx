import type { FolderGroup } from '../../../types';
import { PreviewFolderRow } from './PreviewFolderRow';

export interface PreviewPlanTreeProps {
  groups: FolderGroup[];
  onGroupToggleExpand: (groupId: string) => void;
  onGroupTmdbSearch: (groupId: string) => void;
  onGroupEdit: (groupId: string) => void;
  onGroupSkip: (groupId: string) => void;
  onFileEdit: (fileId: string) => void;
  onFileSkip: (fileId: string) => void;
  expandedGroups: Set<string>;
}

export function PreviewPlanTree({
  groups,
  onGroupToggleExpand,
  onGroupTmdbSearch,
  onGroupEdit,
  onGroupSkip,
  onFileEdit,
  onFileSkip,
  expandedGroups,
}: PreviewPlanTreeProps) {
  if (groups.length === 0) {
    return (
      <div className="flex items-center justify-center py-16 text-text-secondary text-sm">
        暂无预览项，请先执行扫描。
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-1">
      {groups.map((group) => (
        <PreviewFolderRow
          key={group.id}
          group={group}
          expanded={expandedGroups.has(group.id)}
          onToggleExpand={() => onGroupToggleExpand(group.id)}
          onTmdbSearch={() => onGroupTmdbSearch(group.id)}
          onEdit={() => onGroupEdit(group.id)}
          onSkip={() => onGroupSkip(group.id)}
          onFileEdit={onFileEdit}
          onFileSkip={onFileSkip}
        />
      ))}
    </div>
  );
}
