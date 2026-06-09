// types/candidateSearch.ts — Candidate Search Mock UI 类型定义
// 职责：定义 P2-010 Mock UI 的 DTO 类型，不包含 API key、backend command、真实 fetch 结果

/**
 * Mock TMDb 候选卡片 DTO
 * 仅用于 P2-010 Mock UI 展示，不包含真实 TMDb API 响应
 */
export interface MockTmdbCandidate {
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
}

/**
 * Candidate Search Mock UI 状态
 * 仅用于 P2-010 Mock UI 本地状态展示，不保存到全局 store
 */
export interface CandidateSearchMockState {
  mode: CandidateSearchMockMode;
  query: string;
  mediaType: 'Movie' | 'Tv';
  language: string;
  isLoading: boolean;
  candidates: MockTmdbCandidate[];
  error: CandidateSearchMockError | null;
  lastUpdatedAt: number | null;
}

/**
 * Candidate Search Mock UI 模式
 * - idle: 初始状态，未开始搜索
 * - loading: 加载中（mock）
 * - success: 搜索成功（mock 数据）
 * - error: 搜索错误（mock）
 * - empty: 搜索结果为空（mock）
 */
export type CandidateSearchMockMode = 'idle' | 'loading' | 'success' | 'error' | 'empty';

/**
 * Candidate Search Mock UI 错误
 * 仅用于 P2-010 Mock UI 展示，不包含真实错误信息
 */
export interface CandidateSearchMockError {
  code: string;
  message: string;
  retryable: boolean;
}
