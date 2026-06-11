import { X } from 'lucide-react';
import { Button, Card } from '../../../components/ui';
import { CandidateRow } from './CandidateRow';
import type { CandidateListProps } from './tmdbInspectorTypes';

export function CandidateList({ candidates, selectedCandidate, onSelect, onApply, onClear }: CandidateListProps) {
  return (
    <Card className="overflow-hidden p-0">
      <div className="flex items-center justify-between border-b border-border bg-surface-subtle px-3 py-2">
        <h3 className="text-xs font-semibold text-text-primary">TMDb 候选 ({candidates.length})</h3>
        <Button variant="ghost" size="sm" icon={<X className="h-3 w-3" />} onClick={onClear}>{''}</Button>
      </div>
      <div className="max-h-80 divide-y divide-border overflow-y-auto">
        {candidates.map((candidate) => (
          <CandidateRow
            key={`${candidate.tmdb_id}:${candidate.title}`}
            candidate={candidate}
            selected={selectedCandidate?.tmdb_id === candidate.tmdb_id}
            onSelect={onSelect}
            onApply={onApply}
          />
        ))}
      </div>
    </Card>
  );
}
