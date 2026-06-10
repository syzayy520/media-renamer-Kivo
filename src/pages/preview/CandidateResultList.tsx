import { useCallback } from 'react';
import { Check, Film, Tv, Star, Calendar } from 'lucide-react';
import { Badge } from '../../components/ui';
import type { TmdbCandidate } from '../../types';

interface CandidateResultListProps {
  candidates: TmdbCandidate[];
  selectedCandidate: TmdbCandidate | null;
  onSelect: (candidate: TmdbCandidate) => void;
}

export function CandidateResultList({
  candidates,
  selectedCandidate,
  onSelect,
}: CandidateResultListProps) {
  const handleSelect = useCallback(
    (candidate: TmdbCandidate) => {
      onSelect(candidate);
    },
    [onSelect],
  );

  if (candidates.length === 0) {
    return (
      <div className="text-center py-8 text-text-secondary">
        <p>未找到匹配结果</p>
      </div>
    );
  }

  return (
    <div className="space-y-2 max-h-80 overflow-y-auto">
      {candidates.map((candidate) => {
        const isSelected = selectedCandidate?.tmdb_id === candidate.tmdb_id;
        const isMovie = candidate.media_type === 'Movie';
        const rating = candidate.vote_average ? candidate.vote_average.toFixed(1) : null;

        return (
          <button
            key={candidate.tmdb_id}
            onClick={() => handleSelect(candidate)}
            className={`
              w-full text-left p-3 rounded-lg border transition-all
              ${isSelected
                ? 'border-accent bg-accent/10 ring-1 ring-accent/30'
                : 'border-border bg-bg-primary hover:border-accent/50 hover:bg-bg-secondary'
              }
            `}
          >
            <div className="flex items-start gap-3">
              {/* 海报占位 */}
              <div className="w-12 h-16 rounded bg-bg-secondary flex items-center justify-center shrink-0 overflow-hidden">
                {candidate.poster_path ? (
                  <img
                    src={`https://image.tmdb.org/t/p/w92${candidate.poster_path}`}
                    alt={candidate.title}
                    className="w-full h-full object-cover"
                    loading="lazy"
                  />
                ) : (
                  isMovie ? (
                    <Film className="w-5 h-5 text-text-secondary" />
                  ) : (
                    <Tv className="w-5 h-5 text-text-secondary" />
                  )
                )}
              </div>

              {/* 信息 */}
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-1">
                  <span className="font-medium text-text-primary truncate">
                    {candidate.title}
                  </span>
                  {isSelected && (
                    <Check className="w-4 h-4 text-accent shrink-0" />
                  )}
                </div>

                <div className="flex items-center gap-2 text-xs text-text-secondary mb-1.5">
                  <Badge
                    variant={isMovie ? 'default' : 'info'}
                    size="sm"
                  >
                    {isMovie ? '电影' : '剧集'}
                  </Badge>

                  {candidate.year && (
                    <span className="flex items-center gap-1">
                      <Calendar className="w-3 h-3" />
                      {candidate.year}
                    </span>
                  )}

                  {rating && (
                    <span className="flex items-center gap-1">
                      <Star className="w-3 h-3 text-yellow-500" />
                      {rating}
                    </span>
                  )}
                </div>

                {candidate.overview && (
                  <p className="text-xs text-text-secondary line-clamp-2 leading-relaxed">
                    {candidate.overview}
                  </p>
                )}
              </div>
            </div>
          </button>
        );
      })}
    </div>
  );
}
