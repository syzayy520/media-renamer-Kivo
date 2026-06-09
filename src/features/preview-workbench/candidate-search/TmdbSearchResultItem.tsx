// features/preview-workbench/candidate-search/TmdbSearchResultItem.tsx — TMDb 搜索结果项（Mock UI）
// 职责：展示单个 mock 候选卡片
// 安全边界：不调用 backend command，不 invoke，不 fetch，所有 action disabled

/**
 * TMDb 搜索结果项 Mock UI
 * 展示单个候选卡片，所有 action 均为 disabled
 */
export function TmdbSearchResultItem({ candidate }: { candidate: {
  id: string;
  tmdbId: number;
  title: string;
  originalTitle?: string;
  mediaType: 'Movie' | 'Tv';
  releaseYear?: number;
  overview?: string;
  posterPath?: string;
  backdropPath?: string;
  language?: string;
  popularity?: number;
  voteAverage?: number;
  confidenceHint?: number;
  matchReasons: string[];
} }) {
  return (
    <div className="rounded-lg border border-white/10 bg-white/[0.03] p-3 hover:bg-white/[0.06] transition-colors">
      {/* 卡片内容 */}
      <div className="flex gap-3">
        {/* Poster 占位 */}
        <div className="h-[120px] w-[80px] flex-shrink-0 rounded-md bg-white/5 flex items-center justify-center text-xs text-white/20">
          {candidate.posterPath ? 'Poster' : 'No Image'}
        </div>

        {/* 信息区 */}
        <div className="flex-1 flex flex-col gap-1">
          {/* 标题和年份 */}
          <div className="flex items-start justify-between gap-2">
            <div>
              <div className="text-sm font-medium text-white/80">{candidate.title}</div>
              {candidate.originalTitle && candidate.originalTitle !== candidate.title && (
                <div className="text-xs text-white/40">{candidate.originalTitle}</div>
              )}
            </div>
            <div className="flex items-center gap-2 flex-shrink-0">
              <span className="rounded-full bg-blue-500/20 px-2 py-0.5 text-xs text-blue-400/80">
                {candidate.mediaType}
              </span>
              {candidate.releaseYear && (
                <span className="text-xs text-white/40">{candidate.releaseYear}</span>
              )}
            </div>
          </div>

          {/* Overview */}
          {candidate.overview && (
            <div className="text-xs text-white/50 line-clamp-2">
              {candidate.overview}
            </div>
          )}

          {/* Match Reasons */}
          <div className="flex flex-wrap gap-1 mt-1">
            {candidate.matchReasons.map((reason, index) => (
              <span
                key={index}
                className="rounded-full bg-green-500/20 px-2 py-0.5 text-xs text-green-400/80"
              >
                {reason}
              </span>
            ))}
          </div>

          {/* 评分和信息 */}
          <div className="flex items-center gap-3 mt-auto text-xs text-white/40">
            {candidate.voteAverage && (
              <span>⭐ {candidate.voteAverage.toFixed(1)}</span>
            )}
            {candidate.popularity && (
              <span>Popular: {candidate.popularity.toFixed(0)}</span>
            )}
            {candidate.confidenceHint && (
              <span>Confidence: {(candidate.confidenceHint * 100).toFixed(0)}%</span>
            )}
          </div>
        </div>
      </div>

      {/* Action 按钮（disabled） */}
      <div className="mt-3 flex gap-2">
        <button
          disabled
          className="flex-1 rounded-md bg-blue-500/20 py-1.5 text-xs font-medium text-blue-400/50 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Select Candidate (disabled)
        </button>
        <button
          disabled
          className="flex-1 rounded-md bg-white/5 py-1.5 text-xs font-medium text-white/40 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Preview Rename (disabled)
        </button>
      </div>
    </div>
  );
}
