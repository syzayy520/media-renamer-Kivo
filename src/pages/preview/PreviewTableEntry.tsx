import type { RenamePreviewItem } from '../../types';
import type { ItemSearchState } from '../../state/tmdbSearchStore';
import { PreviewTableGrid } from './PreviewTableGrid';

interface PreviewTableEntryProps {
  items: RenamePreviewItem[];
  selectedItems: Set<string>;
  onSelectItem: (id: string) => void;
  onSelectAll: () => void;
  filteredCount: number;
  onTmdbSearch?: (item: RenamePreviewItem) => void;
  onManualEdit?: (item: RenamePreviewItem) => void;
  onSkipToggle?: (item: RenamePreviewItem) => void;
  itemStates?: Record<string, ItemSearchState>;
  tmdbEnabled?: boolean;
}

export function PreviewTableEntry(props: PreviewTableEntryProps) {
  return <PreviewTableGrid {...props} />;
}
