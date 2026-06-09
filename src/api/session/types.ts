// api/session/types.ts — Session 相关类型
// 职责：PipelineResult / ScanSummary / RenamePreviewItem / SafetyReport 等

// ===== Media Types =====

export type MediaType =
  | 'Movie'
  | 'Series'
  | 'Anime'
  | 'Special'
  | 'Ova'
  | 'Ncop'
  | 'Nced'
  | 'Extras'
  | 'Unknown';

// ===== Scan =====

export interface ScanSummary {
  scan_path: string;
  video_count: number;
  companion_count: number;
  permission_error_count: number;
  scan_duration_ms: number;
}

// ===== Rename Preview =====

export interface RenameConflict {
  conflict_type: 'TargetExists' | 'DuplicateTarget' | 'PathTooLong' | 'InvalidChars' | 'PermissionDenied' | 'SourceNotFound';
  source_path: string;
  target_path: string;
  message: string;
  blocking: boolean;
}

export interface RenamePreviewItem {
  id: string;
  parsed_info: ParsedMediaInfo;
  source_path: string;
  original_name: string;
  proposed_name: string;
  target_path: string;
  media_type: MediaType;
  confidence: number;
  needs_manual_review: boolean;
  should_skip: boolean;
  conflicts: RenameConflict[];
  evidence: RuleMatchEvidence[];
  metadata_source: 'LocalRule' | 'Tmdb' | 'Manual';
}

export interface ParsedMediaInfo {
  media_item: MediaItem;
  media_type: MediaType;
  title: string;
  year: number | null;
  season: number | null;
  episode: number | null;
  episode_end: number | null;
  episode_title: string | null;
  resolution: string | null;
  source: string | null;
  video_codec: string | null;
  audio_codec: string | null;
  group: string | null;
  confidence: number;
  rule_sources: string[];
  special_type: string | null;
  extra_type: string | null;
  extra_number: number | null;
}

export interface MediaItem {
  id: string;
  file_path: string;
  file_name: string;
  extension: string;
  file_size: number;
  is_video: boolean;
  is_companion: boolean;
}

export interface RuleMatchEvidence {
  rule_name: string;
  matched_pattern: string;
  captured_groups: Record<string, string>;
}

// ===== Safety =====

export interface SafetyReport {
  can_execute: boolean;
  dry_run: boolean;
  checks: SafetyCheck[];
  blocking_reasons: string[];
}

export interface SafetyCheck {
  name: string;
  passed: boolean;
  message: string;
}

// ===== Pipeline =====

export interface PipelineResult {
  task_id: string;
  scan: ScanSummary;
  previews: RenamePreviewItem[];
  safety: SafetyReport;
  parsed_count: number;
  unknown_count: number;
}

// ===== Session Input =====

export interface StartRenameSessionInput {
  directory: string;
}
