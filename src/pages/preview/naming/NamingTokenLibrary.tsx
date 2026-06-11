import { Plus } from 'lucide-react';
import type { NamingToken } from '../../../types';
import { AVAILABLE_NAMING_TOKENS } from './namingPresetRules';
import { TOKEN_LABELS } from './namingTokenLabels';

interface NamingTokenLibraryProps {
  onAdd: (token: NamingToken) => void;
}

export function NamingTokenLibrary({ onAdd }: NamingTokenLibraryProps) {
  return (
    <div className="grid grid-cols-2 gap-1">
      {AVAILABLE_NAMING_TOKENS.map((token) => (
        <button
          key={token}
          type="button"
          className="flex items-center justify-between rounded-lg bg-surface-hover px-2 py-1.5 text-left text-xs text-text-secondary hover:text-text-primary"
          onClick={() => onAdd(token)}
        >
          <span className="truncate">{TOKEN_LABELS[token]}</span>
          <Plus className="h-3 w-3 shrink-0" />
        </button>
      ))}
    </div>
  );
}
