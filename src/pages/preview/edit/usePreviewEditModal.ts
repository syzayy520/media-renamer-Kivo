import { useCallback, useState } from 'react';
import type { FolderGroup, RenamePreviewItem } from '../../../types';
import type { EditNameModalState } from './EditNameModalState';

interface UsePreviewEditModalParams {
  groups: FolderGroup[];
  previews: RenamePreviewItem[];
  onNameChange: (targetId: string, newName: string) => void;
}

export function usePreviewEditModal({ groups, previews, onNameChange }: UsePreviewEditModalParams) {
  const [editModal, setEditModal] = useState<EditNameModalState | null>(null);

  const openGroupEdit = useCallback((groupId: string) => {
    const group = groups.find((item) => item.id === groupId);
    if (!group || group.children.length === 0) return;
    const mainFile = group.children.find((child) => child.file_role === 'MainVideo') || group.children[0];
    setEditModal({
      mode: 'group',
      targetId: mainFile.id,
      currentName: group.target_folder_name,
      previewPath: group.target_path,
    });
  }, [groups]);

  const openFileEdit = useCallback((fileId: string) => {
    const item = previews.find((preview) => preview.id === fileId);
    if (!item) return;
    setEditModal({
      mode: 'file',
      targetId: fileId,
      currentName: item.proposed_name,
      previewPath: item.target_path,
    });
  }, [previews]);

  const confirmEdit = useCallback((newName: string) => {
    if (!editModal) return;
    onNameChange(editModal.targetId, newName);
    setEditModal(null);
  }, [editModal, onNameChange]);

  const cancelEdit = useCallback(() => {
    setEditModal(null);
  }, []);

  return {
    editModal,
    openGroupEdit,
    openFileEdit,
    confirmEdit,
    cancelEdit,
  };
}
