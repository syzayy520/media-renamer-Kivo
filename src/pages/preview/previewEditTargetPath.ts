import type { RenamePreviewItem } from '../../types';

export function buildEditedTargetPath(item: RenamePreviewItem, proposedName: string): string {
  if (item.target_path.endsWith(item.proposed_name)) {
    return `${item.target_path.slice(0, -item.proposed_name.length)}${proposedName}`;
  }

  const separatorIndex = Math.max(
    item.target_path.lastIndexOf('\\'),
    item.target_path.lastIndexOf('/'),
  );

  if (separatorIndex < 0) {
    return proposedName;
  }

  return `${item.target_path.slice(0, separatorIndex + 1)}${proposedName}`;
}
