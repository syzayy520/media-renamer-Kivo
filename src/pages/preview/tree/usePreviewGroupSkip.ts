import { useCallback } from 'react';
import type { FolderGroup } from '../../../types';

export function usePreviewGroupSkip(groups: FolderGroup[], onFileSkip: (fileId: string) => void) {
  return useCallback((groupId: string) => {
    const group = groups.find((item) => item.id === groupId);
    if (!group) return;
    group.children.forEach((child) => onFileSkip(child.id));
  }, [groups, onFileSkip]);
}
