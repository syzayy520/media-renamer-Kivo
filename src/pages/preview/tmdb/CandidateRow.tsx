import { Film, Star } from 'lucide-react';
import { Badge, Button } from '../../../components/ui';
import type { CandidateRowProps } from './tmdbInspectorTypes';
import { TMDB_IMAGE_BASE, isMovieCandidate } from './tmdbInspectorTypes';

export function CandidateRow({ candidate, selected, onSelect, onApply }: CandidateRowProps) {
  return (
    <div
      className={`flex cursor-pointer gap-3 p-3 transition-colors hover:bg-surface-hover ${selected ? 'border-l-2 border-primary bg-primary/5' : ''}`}
      onClick={() => onSelect(candidate)}
    >
      <div className="h-20 w-12 shrink-0 overflow-hidden rounded bg-surface-hover">
        {candidate.poster_path ? (
          <img
            src={`${TMDB_IMAGE_BASE}${candidate.poster_path}`}
            alt={candidate.title}
            className="h-full w-full object-cover"
            loading="lazy"
          />
        ) : (
          <div className="flex h-full w-full items-center justify-center text-text-tertiary">
            <Film className="h-5 w-5" />
          </div>
        )}
      </div>

      <div className="min-w-0 flex-1">
        <div className="flex items-center gap-1.5">
          <span className="truncate text-xs font-medium text-text-primary">{candidate.title}</span>
          {candidate.year && <span className="shrink-0 text-xs text-text-tertiary">({candidate.year})</span>}
        </div>
        {candidate.original_title && candidate.original_title !== candidate.title && (
          <div className="truncate text-xs text-text-tertiary">{candidate.original_title}</div>
        )}
        <div className="mt-1 flex items-center gap-2">
          {candidate.vote_average != null && (
            <span className="flex items-center gap-0.5 text-xs text-warning">
              <Star className="h-3 w-3 fill-current" />
              {candidate.vote_average.toFixed(1)}
            </span>
          )}
          <Badge variant="default" size="sm">
            {isMovieCandidate(candidate) ? '🎬' : '📺'} TMDb {candidate.tmdb_id}
          </Badge>
        </div>
        {candidate.overview && <p className="mt-1 line-clamp-2 text-xs text-text-tertiary">{candidate.overview}</p>}
      </div>

      <div className="flex shrink-0 items-center">
        <Button
          variant="primary"
          size="sm"
          onClick={(event) => {
            event.stopPropagation();
            onApply(candidate);
          }}
        >
          应用
        </Button>
      </div>
    </div>
  );
}
