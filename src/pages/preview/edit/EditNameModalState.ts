export type EditNameModalState = {
  mode: 'file' | 'group';
  targetId: string;
  currentName: string;
  previewPath: string;
};
