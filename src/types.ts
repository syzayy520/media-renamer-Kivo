// 后端数据类型定义
// 职责：定义与Rust后端对应的TypeScript类型

// 媒体类型
export type MediaType = 
  | 'Movie'
  | 'Series'
  | 'Anime'
  | 'Special'
  | 'OVA'
  | 'NCOP'
  | 'NCED'
  | 'Extras'
  | 'Unknown';

// 特别篇类型
export type SpecialType = 
  | 'Special'
  | 'SP'
  | 'OVA'
  | 'NCOP'
  | 'NCED'
  | 'Extra';

// 元数据来源
export type MetadataSource = 
  | 'LocalRule'
  | 'Tmdb'
  | 'Manual';

// 任务状态
export type TaskStatus = 
  | 'Previewing'
  | 'Ready'
  | 'Executing'
  | 'Completed'
  | 'Failed'
  | 'Cancelled';

// 冲突类型
export type ConflictType = 
  | 'TargetExists'
  | 'SourceNotFound'
  | 'TargetIsSource'
  | 'NameCollision';

// 媒体项（扫描结果）
export interface MediaItem {
  id: string;
  file_path: string;
  file_name: string;
  extension: string;
  file_size: number;
  is_video: boolean;
  is_companion: boolean;
}

// 解析后的媒体信息
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
  special_type: SpecialType | null;
  extra_type: string | null;
  extra_number: number | null;
}

// 重命名冲突
export interface RenameConflict {
  conflict_type: ConflictType;
  source_path: string;
  target_path: string;
  message: string;
  blocking: boolean;
}

// 规则匹配证据
export interface RuleMatchEvidence {
  rule_name: string;
  matched_pattern: string;
  captured_groups: Record<string, string>;
}

// 重命名预览项
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
  metadata_source: MetadataSource;
}

// 安全检查
export interface SafetyCheck {
  name: string;
  passed: boolean;
  message: string;
}

// 安全报告
export interface SafetyReport {
  can_execute: boolean;
  dry_run: boolean;
  checks: SafetyCheck[];
  blocking_reasons: string[];
}

// 扫描摘要
export interface ScanSummary {
  scan_path: string;
  video_count: number;
  companion_count: number;
  permission_error_count: number;
  scan_duration_ms: number;
}

// 流水线结果
export interface PipelineResult {
  task_id: string;
  scan: ScanSummary;
  previews: RenamePreviewItem[];
  safety: SafetyReport;
  parsed_count: number;
  unknown_count: number;
}

// 重命名任务
export interface RenameTask {
  id: string;
  status: TaskStatus;
  template: string;
  total_files: number;
  created_at: string;
  updated_at: string;
  error_message: string | null;
}

// 审计日志条目
export interface AuditLogEntry {
  id: string;
  task_id: string | null;
  event_type: string;
  message: string;
  created_at: string;
}

// 应用配置
export interface AppConfig {
  templates: TemplatesConfig;
  thresholds: ThresholdsConfig;
  limits: LimitsConfig;
}

// 模板配置
export interface TemplatesConfig {
  movie: string;
  series: string;
  anime: string;
  special: string;
  extras: string;
}

// 阈值配置
export interface ThresholdsConfig {
  confidence: number;
}

// 限制配置
export interface LimitsConfig {
  max_files: number;
  max_path_length: number;
}

// 重命名规则
export interface RenameRule {
  media_type: MediaType;
  template: string;
  is_default: boolean;
}

// ─── TMDb Search Contract Types ───────────────────────────────────────────────

// TMDb 候选来源
export type TmdbCandidateSource = 'Tmdb';

// TMDb 搜索错误码
export type TmdbSearchErrorCode =
  | 'Unknown'
  | 'InvalidInput'
  | 'ApiKeyMissing'
  | 'ApiKeyInvalid'
  | 'RateLimited'
  | 'NetworkError'
  | 'ServiceUnavailable';

// TMDb 搜索错误
export interface TmdbSearchError {
  code: TmdbSearchErrorCode;
  message: string;
  retryable: boolean;
  retry_after: number | null;
}

// TMDb 候选项
export interface TmdbCandidate {
  tmdb_id: number;
  title: string;
  original_title: string | null;
  year: number | null;
  media_type: MediaType;
  overview: string | null;
  poster_path: string | null;
  backdrop_path: string | null;
  vote_average: number | null;
  vote_count: number | null;
  popularity: number | null;
  release_date: string | null;
  original_language: string | null;
  genre_ids: number[];
  source: TmdbCandidateSource;
  confidence: number;
}

// TMDb 速率限制状态
export interface TmdbRateLimitState {
  remaining: number | null;
  limit: number | null;
  reset_epoch_secs: number | null;
  retry_after_secs: number | null;
}

// TMDb 搜索输入
export interface SearchTmdbCandidatesInput {
  query: string;
  media_type: 'Movie' | 'Tv';
  language: string;
  year: number | null;
  page: number | null;
}

// TMDb 搜索输出
export interface SearchTmdbCandidatesOutput {
  candidates: TmdbCandidate[];
  source: TmdbCandidateSource;
  rate_limit: TmdbRateLimitState | null;
  error: TmdbSearchError | null;
}

// TMDb 搜索功能状态（前端 UI 用）
export type TmdbSearchStatus = 'disabled' | 'idle' | 'loading' | 'success' | 'error';

// ─── TMDb Candidate Apply Contract Types ─────────────────────────────────────

// TMDb 候选应用输入
export interface ApplyTmdbCandidateInput {
  item: RenamePreviewItem;
  candidate: TmdbCandidate;
}

// TMDb 候选应用输出
export interface ApplyTmdbCandidateOutput {
  result: PreviewApplyResult | null;
  error: string | null;
  safety: SafetyReport | null;
}

// 预览候选应用结果
export interface PreviewApplyResult {
  updated_item: RenamePreviewItem;
  success: boolean;
  message: string;
  warnings: string[];
}

// 安全摘要输入
export interface SafetySummaryInput {
  previews: RenamePreviewItem[];
}

// ─── Safe Execution Contract Types ───────────────────────────────────────────

// 执行模式
export type ExecutionMode = 'DryRun' | 'Confirmed';

// 执行项状态
export type ExecutionItemStatus = 'Success' | 'Failed' | 'Skipped' | 'Blocked';

// 安全执行输入
export interface SafeExecuteInput {
  task_id: string;
  preview_items: RenamePreviewItem[];
  mode: ExecutionMode;
  user_confirmed: boolean;
}

// 执行项结果
export interface ExecutionItemResult {
  source_path: string;
  target_path: string;
  status: ExecutionItemStatus;
  error: string | null;
  rollback_entry: RollbackEntry | null;
}

// 执行输出摘要
export interface ExecutionOutputSummary {
  total: number;
  success: number;
  failed: number;
  skipped: number;
  blocked: number;
  mode: ExecutionMode;
  has_rollback_plan: boolean;
}

// 安全执行输出
export interface SafeExecuteOutput {
  allowed: boolean;
  rejection_reason: string | null;
  item_results: ExecutionItemResult[];
  summary: ExecutionOutputSummary | null;
  rollback_plan: RollbackPlan | null;
}

// ─── Rollback Contract Types ─────────────────────────────────────────────────

// 回滚计划
export interface RollbackPlan {
  task_id: string;
  entries: RollbackPlanEntry[];
  created_at: string;
}

// 回滚计划条目
export interface RollbackPlanEntry {
  source_path: string;
  target_path: string;
  rollback_source: string;
  rollback_target: string;
  executed: boolean;
}

// 回滚条目状态
export type RollbackStatus = 'Pending' | 'Success' | 'Failed' | 'Skipped';

// 回滚条目
export interface RollbackEntry {
  id: string;
  task_id: string;
  source_path: string;
  target_path: string;
  rollback_source: string;
  rollback_target: string;
  status: RollbackStatus;
  error_message: string | null;
  created_at: string;
}

// ─── Execution UI State Types ────────────────────────────────────────────────

// 执行 UI 状态
export type ExecutionUIState = 
  | 'idle'
  | 'confirming'
  | 'executing'
  | 'completed'
  | 'failed';

// 执行确认面板状态
export interface ExecutionConfirmState {
  safety_report: SafetyReport | null;
  preview_count: number;
  has_blockers: boolean;
  blocking_reasons: string[];
}

// 执行进度状态
export interface ExecutionProgressState {
  total: number;
  completed: number;
  success: number;
  failed: number;
  skipped: number;
  blocked: number;
  current_item: string | null;
}

// 执行结果状态
export interface ExecutionResultState {
  output: SafeExecuteOutput | null;
  error: string | null;
  has_rollback_plan: boolean;
  can_rollback: boolean;
}
