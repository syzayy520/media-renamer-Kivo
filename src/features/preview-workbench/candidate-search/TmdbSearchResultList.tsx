// features/preview-workbench/candidate-search/TmdbSearchResultList.tsx — TMDb 搜索结果列表（Mock UI）
// 职责：展示 mock 搜索结果列表
// 安全边界：不调用 backend command，不 invoke，不 fetch，使用静态 mock 数据

import { TmdbSearchResultItem } from './TmdbSearchResultItem';

/**
 * Mock TMDb 搜索结果数据
 * 仅用于 P2-010 Mock UI 展示，不包含真实 TMDb API 响应
 */
const MOCK_CANDIDATES = [
  {
    id: 'mock-1',
    tmdbId: 550,
    title: 'Fight Club',
    originalTitle: 'Fight Club',
    mediaType: 'Movie' as const,
    releaseYear: 1999,
    overview: 'A ticking-Loss-Loss-Loss bomb insomniac and a slippery soap salesman channel primal male aggression into a shocking new form of therapy.',
    posterPath: '/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg',
    backdropPath: '/hZkgoQYus5dXo3H8T7Uef6DNknx.jpg',
    language: 'en',
    popularity: 61.4,
    voteAverage: 8.4,
    confidenceHint: 0.95,
    matchReasons: ['Title match', 'Year match', 'Popular on TMDb'],
  },
  {
    id: 'mock-2',
    tmdbId: 13,
    title: 'Forrest Gump',
    originalTitle: 'Forrest Gump',
    mediaType: 'Movie' as const,
    releaseYear: 1994,
    overview: 'A man with a low IQ has accomplished great things in his life and been present during significant historic events.',
    posterPath: '/arw2vcBveWOVZr6pxd9XTd1TdQa.jpg',
    backdropPath: '/yQa6Crb6hExzECIvKCMZM21cMvq.jpg',
    language: 'en',
    popularity: 55.2,
    voteAverage: 8.5,
    confidenceHint: 0.88,
    matchReasons: ['Title similarity', 'Year proximity'],
  },
  {
    id: 'mock-3',
    tmdbId: 155,
    title: 'The Dark Knight',
    originalTitle: 'The Dark Knight',
    mediaType: 'Movie' as const,
    releaseYear: 2008,
    overview: 'When the menace known as the Joker wreaks havoc and chaos on Gotham City, Batman must accept one of the greatest tests.',
    posterPath: '/qJ2tW6WMUDux911BTUgMe1SnRlk.jpg',
    backdropPath: '/hqkIePZ8Y8pPPbVXjHfXqRk5Llq.jpg',
    language: 'en',
    popularity: 72.3,
    voteAverage: 8.5,
    confidenceHint: 0.82,
    matchReasons: ['Genre match', 'Director match'],
  },
];

/**
 * TMDb 搜索结果列表 Mock UI
 * 展示静态 mock 候选卡片列表，所有 action 均为 disabled
 */
export function TmdbSearchResultList() {
  return (
    <div className="flex flex-col gap-2">
      <div className="text-xs text-white/40">
        Found {MOCK_CANDIDATES.length} mock candidates (P2-010)
      </div>
      <div className="flex flex-col gap-2 max-h-[400px] overflow-y-auto">
        {MOCK_CANDIDATES.map((candidate) => (
          <TmdbSearchResultItem key={candidate.id} candidate={candidate} />
        ))}
      </div>
    </div>
  );
}
