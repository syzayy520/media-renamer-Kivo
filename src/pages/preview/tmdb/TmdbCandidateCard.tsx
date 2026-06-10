import { Star, Calendar } from 'lucide-react';
import type { TmdbCandidate } from '../../../types';
import { Button, Badge } from '../../../components/ui';

export interface TmdbCandidateCardProps {
  candidate: TmdbCandidate;
  selected: boolean;
  onSelect: () => void;
  onApply: () => void;
  imageBaseUrl: string;
}

export function TmdbCandidateCard({ candidate, selected, onSelect, onApply, imageBaseUrl }: TmdbCandidateCardProps) {
  const posterUrl = candidate.poster_path ? `${imageBaseUrl}${candidate.poster_path}` : null;

  return (
    <div
      className={`flex gap-3 p-3 rounded-lg border transition-colors cursor-pointer mb-2 ${selected ? 'border-primary bg-primary/5' : 'border-border hover:border-primary/30 hover:bg-surface-hover'}`}
      onClick={onSelect}
    >
      {/* Poster */}
      <div className="w-16 h-24 shrink-0 rounded-md overflow-hidden bg-surface-subtle flex items-center justify-center">
        {posterUrl ? (
          <img
            src={posterUrl}
            alt={candidate.title}
            className="w-full h-full object-cover"
            loading="lazy"
            onError={(e) => {
              (e.target as HTMLImageElement).style.display = 'none';
              const placeholder = (e.target as HTMLImageElement).nextElementSibling;
              if (placeholder) (placeholder as HTMLElement).style.display = 'flex';
            }}
          />
        ) : null}
        <div className={`w-full h-full items-center justify-center text-text-tertiary ${posterUrl ? 'hidden' : 'flex'}`}>
          <Calendar className="h-6 w-6" />
        </div>
      </div>

      {/* Info */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <h4 className="text-sm font-medium text-text-primary truncate">{candidate.title}</h4>
          {candidate.year && (
            <span className="text-xs text-text-tertiary shrink-0">{candidate.year}</span>
          )}
        </div>

        {candidate.original_title && candidate.original_title !== candidate.title && (
          <p className="text-xs text-text-tertiary truncate mb-1">{candidate.original_title}</p>
        )}

        {candidate.overview && (
          <p className="text-xs text-text-secondary line-clamp-2 mb-2">{candidate.overview}</p>
        )}

        <div className="flex items-center gap-2">
          {candidate.vote_average ? (
            <span className="flex items-center gap-0.5 text-xs text-text-secondary">
              <Star className="h-3 w-3 text-warning" />
              {candidate.vote_average.toFixed(1)}
            </span>
          ) : null}
          <Badge variant="default" size="sm">
            {candidate.media_type === 'Movie' ? '电影' : candidate.media_type === 'Series' ? '电视剧' : candidate.media_type}
          </Badge>
          <span className="text-xs text-text-tertiary">
            ID: {candidate.tmdb_id}
          </span>
        </div>
      </div>

      {/* Actions */}
      <div className="flex flex-col justify-center shrink-0" onClick={(e) => e.stopPropagation()}>
        <Button variant="primary" size="sm" onClick={onApply}>
          应用
        </Button>
      </div>
    </div>
  );
}
