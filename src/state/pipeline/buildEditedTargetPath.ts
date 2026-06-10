import type { RenamePreviewItem } from '../../types';

export function buildEditedTargetPath(item: RenamePreviewItem, proposedName: string): string {
  if (item.target_path.endsWith(item.proposed_name)) {
    return `${item.target_path.slice(0, -item.proposed_name.length)}${proposedName}`;
  }

  const windowsSeparatorIndex = item.target_path.lastIndexOf('\\');
  const unixSeparatorIndex = item.target_path.lastIndexOf('/');
  const separatorIndex = Math.max(windowsSeparatorIndex, unixSeparatorIndex);

  if (separatorIndex < 0) {
    return proposedName;
  }

  return `${item.target_path.slice(0, separatorIndex + 1)}${proposedName}`;
}
