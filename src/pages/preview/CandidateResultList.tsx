// pages/preview/CandidateResultList 组件
// 职责：展示 TMDb 搜索结果列表，支持选择

import { useCallback } from 'react';
import { Check, Film, Tv, Star, Calendar, Globe } from 'lucide-react';
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
      <div className="text-center py-8 text-text-secondary space-y-2">
        <Film className="w-8 h-8 mx-auto opacity-40" />
        <p className="text-sm">未找到匹配结果</p>
        <p className="text-xs opacity-70">尝试修改搜索关键词或更换类型</p>
      </div>
    );
  }

  return (
    <div className="space-y-2 max-h-96 overflow-y-auto">
      <p className="text-xs text-text-secondary px-1">
        找到 {candidates.length} 个结果，点击选择一个候选
      </p>
      {candidates.map((candidate, index) => {
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
              {/* 序号 + 海报 */}
              <div className="relative shrink-0">
                <div className="w-12 h-16 rounded bg-bg-secondary flex items-center justify-center overflow-hidden">
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
                <span className="absolute -top-1 -left-1 w-5 h-5 rounded-full bg-bg-card border border-border text-xs flex items-center justify-center text-text-secondary font-medium">
                  {index + 1}
                </span>
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

                {/* 原始标题（如果不同） */}
                {candidate.original_title && candidate.original_title !== candidate.title && (
                  <p className="text-xs text-text-secondary/60 truncate mb-1">
                    {candidate.original_title}
                  </p>
                )}

                <div className="flex items-center gap-2 text-xs text-text-secondary mb-1.5 flex-wrap">
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

                  {candidate.original_language && (
                    <span className="flex items-center gap-1">
                      <Globe className="w-3 h-3" />
                      {candidate.original_language.toUpperCase()}
                    </span>
                  )}

                  {candidate.vote_count && candidate.vote_count > 0 && (
                    <span className="opacity-50">
                      ({candidate.vote_count.toLocaleString()} 票)
                    </span>
                  )}
                </div>

                {candidate.overview && (
                  <p className="text-xs text-text-secondary line-clamp-2 leading-relaxed">
                    {candidate.overview}
                  </p>
                )}
              </div>

              {/* 选择指示 */}
              {isSelected && (
                <div className="shrink-0 w-6 h-6 rounded-full bg-accent flex items-center justify-center">
                  <Check className="w-3.5 h-3.5 text-white" />
                </div>
              )}
            </div>
          </button>
        );
      })}
    </div>
  );
}
