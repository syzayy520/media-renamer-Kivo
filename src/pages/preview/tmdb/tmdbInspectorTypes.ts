import type { FolderGroup, SearchTmdbCandidatesInput, TmdbCandidate } from '../../../types';

export const TMDB_IMAGE_BASE = 'https://image.tmdb.org/t/p/w185';

export type TmdbManualMediaType = SearchTmdbCandidatesInput['media_type'];

export interface TmdbInspectorPanelProps {
  selectedGroup: FolderGroup | null;
  tmdbMediaType: TmdbManualMediaType;
  tmdbQuery: string;
  tmdbLoading: boolean;
  tmdbError: string | null;
  tmdbDisabled: boolean;
  candidates: TmdbCandidate[];
  selectedCandidate: TmdbCandidate | null;
  onQueryChange: (value: string) => void;
  onMediaTypeChange: (value: TmdbManualMediaType) => void;
  onSearch: () => void;
  onCandidateSelect: (candidate: TmdbCandidate) => void;
  onCandidateApply: (candidate: TmdbCandidate) => void;
  onClearCandidates: () => void;
}

export interface CandidateListProps {
  candidates: TmdbCandidate[];
  selectedCandidate: TmdbCandidate | null;
  onSelect: (candidate: TmdbCandidate) => void;
  onApply: (candidate: TmdbCandidate) => void;
  onClear: () => void;
}

export interface CandidateRowProps {
  candidate: TmdbCandidate;
  selected: boolean;
  onSelect: (candidate: TmdbCandidate) => void;
  onApply: (candidate: TmdbCandidate) => void;
}

export interface ScrapePreviewCardProps {
  selectedCandidate: TmdbCandidate;
  selectedGroup: FolderGroup | null;
}

export function isMovieCandidate(candidate: TmdbCandidate): boolean {
  const mediaType = candidate.media_type as string;
  return mediaType === 'Movie' || mediaType === 'movie';
}
