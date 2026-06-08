# 接口与契约

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、Tauri IPC 命令清单

### 1.1 扫描命令

```rust
#[tauri::command]
async fn scan_directory(path: String, app_handle: AppHandle) -> Result<ScanReport, String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| path | String | 扫描目录路径 |
| app_handle | AppHandle | Tauri 句柄（用于事件推送） |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | ScanReport | 扫描报告 |
| Err | String | 错误信息 |

**事件推送**：
- `scan-progress` → `{ scanned: u32, total: u32, current_file: String }`

---

### 1.2 解析命令

```rust
#[tauri::command]
async fn parse_media_files(media_items: Vec<MediaItem>) -> Result<Vec<ParsedMediaInfo>, String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| media_items | Vec\<MediaItem\> | 待解析文件列表 |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | Vec\<ParsedMediaInfo\> | 解析结果 |
| Err | String | 错误信息 |

---

### 1.3 预览命令

```rust
#[tauri::command]
async fn generate_preview(
    parsed_items: Vec<ParsedMediaInfo>,
    template: Option<String>
) -> Result<Vec<RenamePreviewItem>, String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| parsed_items | Vec\<ParsedMediaInfo\> | 已解析项 |
| template | Option\<String\> | 自定义模板（None 使用默认） |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | Vec\<RenamePreviewItem\> | 预览列表 |
| Err | String | 错误信息 |

---

### 1.4 人工确认命令

```rust
#[tauri::command]
async fn apply_manual_review(
    review_items: Vec<ManualReviewItem>
) -> Result<Vec<ParsedMediaInfo>, String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| review_items | Vec\<ManualReviewItem\> | 用户编辑后的确认项 |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | Vec\<ParsedMediaInfo\> | 更新后的解析结果 |
| Err | String | 错误信息 |

---

### 1.5 执行命令

```rust
#[tauri::command]
async fn execute_rename(
    preview_items: Vec<RenamePreviewItem>,
    dry_run: bool,
    app_handle: AppHandle
) -> Result<RenameTask, String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| preview_items | Vec\<RenamePreviewItem\> | 待执行列表 |
| dry_run | bool | 是否为预演模式 |
| app_handle | AppHandle | Tauri 句柄（用于事件推送） |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | RenameTask | 任务结果 |
| Err | String | 错误信息 |

**事件推送**：
- `rename-progress` → `{ completed: u32, total: u32, current_file: String, status: String }`

---

### 1.6 回滚命令

```rust
#[tauri::command]
async fn rollback_task(
    task_id: String,
    app_handle: AppHandle
) -> Result<RollbackRecord, String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| task_id | String | 待回滚任务 ID |
| app_handle | AppHandle | Tauri 句柄（用于事件推送） |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | RollbackRecord | 回滚记录 |
| Err | String | 错误信息 |

**事件推送**：
- `rollback-progress` → `{ completed: u32, total: u32, current_file: String, status: String }`

---

### 1.7 任务查询命令

```rust
#[tauri::command]
async fn get_task_history() -> Result<Vec<RenameTask>, String>

#[tauri::command]
async fn get_task_detail(task_id: String) -> Result<RenameTask, String>
```

---

### 1.8 配置命令

```rust
#[tauri::command]
async fn get_config() -> Result<AppConfig, String>

#[tauri::command]
async fn update_template(media_type: MediaType, template: String) -> Result<(), String>

#[tauri::command]
async fn update_confidence_threshold(value: u8) -> Result<(), String>

#[tauri::command]
async fn get_templates() -> Result<Vec<RenameRule>, String>
```

---

### 1.9 TMDb API Key 命令

```rust
#[tauri::command]
async fn get_tmdb_api_key() -> Result<TmdbApiKeyResponse, String>

#[tauri::command]
async fn save_tmdb_api_key(api_key: String) -> Result<TmdbApiKeyResponse, String>

#[tauri::command]
async fn clear_tmdb_api_key() -> Result<(), String>

#[tauri::command]
async fn test_tmdb_connection() -> Result<(), String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| api_key | String | TMDb API Key |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | TmdbApiKeyResponse | 包含 apiKey 和 maskedApiKey |
| Err | String | 错误信息 |

**TmdbApiKeyResponse 结构**：
```rust
#[derive(Serialize, Deserialize)]
pub struct TmdbApiKeyResponse {
    pub api_key: Option<String>,
    pub masked_api_key: Option<String>,
}
```

---

### 1.10 导出命令

```rust
#[tauri::command]
async fn export_audit_log(task_id: String, format: String) -> Result<String, String>
```

| 参数 | 类型 | 说明 |
|------|------|------|
| task_id | String | 任务 ID |
| format | String | 导出格式 ("jsonl") |

| 返回 | 类型 | 说明 |
|------|------|------|
| Ok | String | 导出文件路径 |
| Err | String | 错误信息 |

---

## 二、Rust 内部接口

### 2.1 scan 模块

```rust
// scan/scanner.rs
pub fn scan_directory(path: &Path) -> Result<(ScanReport, Vec<MediaItem>)>

// scan/file_detector.rs
pub fn is_video(extension: &str) -> bool
pub fn is_companion(extension: &str) -> bool
pub fn get_video_extensions() -> &'static [&'static str]
pub fn get_companion_extensions() -> &'static [&'static str]

// scan/progress.rs
pub struct ScanProgress {
    pub scanned: u32,
    pub total: u32,
    pub current_file: String,
}
```

### 2.2 parse 模块

```rust
// parse/classifier.rs
pub fn classify(filename: &str) -> MediaType
pub fn classify_special(filename: &str) -> SpecialType
pub fn classify_and_parse(filename: &str) -> Option<ParsedMediaInfo>

// parse/movie_parser.rs
pub fn parse_movie(filename: &str) -> Option<ParsedMediaInfo>

// parse/series_parser.rs
pub fn parse_series(filename: &str) -> Option<ParsedMediaInfo>

// parse/anime_parser.rs
pub fn parse_anime(filename: &str) -> Option<ParsedMediaInfo>

// parse/special_parser.rs
pub fn parse_special(filename: &str) -> Option<ParsedMediaInfo>

// parse/confidence.rs
pub fn evaluate_confidence(info: &mut ParsedMediaInfo) -> ConfidenceResult
pub fn adjust_confidence(info: &mut ParsedMediaInfo)
pub fn needs_review(info: &ParsedMediaInfo, threshold: u8) -> bool

// parse/confidence.rs - 数据结构
pub struct RuleMatchEvidence {
    pub rule_name: String,
    pub matched_pattern: String,
    pub confidence_delta: i16,
}

pub struct ConfidenceResult {
    pub score: u8,
    pub evidences: Vec<RuleMatchEvidence>,
    pub needs_review: bool,
}

pub const DEFAULT_CONFIDENCE_THRESHOLD: u8 = 70
```

### 2.3 rename 模块

```rust
// rename/template.rs - 领域对象 + 模板渲染
pub enum MetadataSource { LocalRule, Tmdb, Manual }

pub struct RenameConflict {
    pub conflict_type: ConflictType,
    pub source_path: String,
    pub target_path: String,
    pub message: String,
    pub blocking: bool,
}

pub struct RenamePreviewItem {
    pub id: String,
    pub parsed_info: ParsedMediaInfo,
    pub source_path: String,
    pub original_name: String,
    pub proposed_name: String,
    pub target_path: String,
    pub media_type: MediaType,
    pub confidence: u8,
    pub needs_manual_review: bool,
    pub should_skip: bool,
    pub conflicts: Vec<RenameConflict>,
    pub evidence: Vec<RuleMatchEvidence>,
    pub metadata_source: MetadataSource,
}

pub fn render(info: &ParsedMediaInfo, template: &str) -> String
pub fn get_default_template(media_type: &MediaType) -> &str

// rename/conflict_detector.rs - 冲突检测
pub fn detect_conflicts(preview_items: &[RenamePreviewItem]) -> Vec<RenameConflict>
pub fn has_blocking_conflicts(conflicts: &[RenameConflict]) -> bool

// rename/safety_checker.rs - 安全检查
pub struct SafetyReport {
    pub can_execute: bool,
    pub dry_run: bool,
    pub checks: Vec<SafetyCheck>,
    pub blocking_reasons: Vec<String>,
}

pub struct SafetyCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

pub fn check_all(preview_items: &[RenamePreviewItem]) -> SafetyReport
pub fn check_single(item: &RenamePreviewItem) -> Vec<SafetyCheck>

// rename/preview_generator.rs - 预览生成
pub fn generate(parsed_items: &[ParsedMediaInfo], template_str: &str) -> Vec<RenamePreviewItem>
pub fn generate_with_default_template(parsed_items: &[ParsedMediaInfo]) -> Vec<RenamePreviewItem>
pub fn sanitize_proposed_name(name: &str) -> String

// rename/execution/ - Controlled Rename Execution（子功能族）

// rename/execution/execution_mode.rs
pub enum ExecutionMode { DryRun, Confirmed }

// rename/execution/single_rename.rs
pub fn execute_single_rename(
    source_path: &str,
    target_path: &str,
) -> AppResult<()>

// rename/execution/execution_summary.rs
pub struct ExecutionSummary { pub total, pub success, pub failed, pub skipped }
pub fn summarize(results: &[RenameResult]) -> ExecutionSummary

// rename/execution/executor_core.rs
pub fn execute(
    conn: &Connection,
    task_id: &str,
    preview_items: &[RenamePreviewItem],
    mode: ExecutionMode,
) -> AppResult<Vec<RenameResult>>

// rename/execution/mod.rs - re-export hub
pub use execution_mode::ExecutionMode;
pub use execution_summary::{summarize, ExecutionSummary};
pub use executor_core::execute;
pub use single_rename::execute_single_rename;
```

### 2.4 rollback 模块

```rust
// rollback/state_checker.rs - 回滚可行性检查
pub struct RollbackReport {
    pub can_rollback: bool,
    pub rollbackable_results: Vec<RenameResult>,
    pub blocked_results: Vec<BlockedRollbackEntry>,
    pub blocking_reasons: Vec<String>,
}

pub struct BlockedRollbackEntry {
    pub result_id: String,
    pub reason: String,
}

pub fn check_rollback(results: &[RenameResult]) -> RollbackReport
pub fn can_rollback_single(result: &RenameResult) -> Result<(), String>
pub fn is_task_rollbackable(task_status: &TaskStatus) -> bool

// rollback/executor/ - 回滚执行（子功能族）

// rollback/executor/rollback_entry.rs
pub enum RollbackStatus {
    Success,
    Failed,
    Blocked,
}

pub struct RollbackEntry {
    pub result_id: String,
    pub before_path: String,
    pub after_path: String,
    pub status: RollbackStatus,
    pub error: Option<String>,
}

pub struct RollbackSummary {
    pub total: u32,
    pub success: u32,
    pub failed: u32,
    pub blocked: u32,
}

pub fn summarize_rollback(entries: &[RollbackEntry]) -> RollbackSummary

// rollback/executor/rollback_core.rs
pub fn rollback_task(
    conn: &Connection,
    task_id: &str,
    results: &[RenameResult],
) -> AppResult<Vec<RollbackEntry>>

// rollback/executor/mod.rs - re-export hub
pub use rollback_core::rollback_task;
pub use rollback_entry::{summarize_rollback, RollbackEntry, RollbackStatus, RollbackSummary};
```

### 2.5 audit 模块

```rust
// audit/db/ - 数据库功能族（6 个子模块）

// audit/db/connection.rs - 连接
pub fn create_memory_connection() -> Result<Connection, rusqlite::Error>
pub fn open_connection(path: &str) -> Result<Connection, rusqlite::Error>

// audit/db/schema.rs - 表初始化
pub fn init_tables(conn: &Connection) -> Result<(), rusqlite::Error>
pub fn tables_exist(conn: &Connection) -> Result<bool, rusqlite::Error>

// audit/db/task_status.rs - 任务状态
pub enum TaskStatus { Previewing, Pending, Executing, Completed, Failed, RolledBack }
pub fn parse_task_status(s: &str) -> TaskStatus

// audit/db/task_repository.rs - 任务仓库
pub struct RenameTask { pub id, pub status, pub template, pub total_files, pub created_at, pub updated_at, pub error_message }
pub fn create_task(template: &str, total_files: u32) -> RenameTask
pub fn insert_task(conn: &Connection, task: &RenameTask) -> Result<(), rusqlite::Error>
pub fn get_task(conn: &Connection, task_id: &str) -> Result<Option<RenameTask>, rusqlite::Error>
pub fn update_task_status(conn: &Connection, task_id: &str, status: TaskStatus, error_message: Option<&str>) -> Result<(), rusqlite::Error>

// audit/db/result_repository.rs - 结果仓库
pub struct RenameResult { pub id, pub task_id, pub source_path, pub target_path, pub status, pub created_at, pub error_message }
pub fn create_result(task_id: &str, source_path: &str, target_path: &str, status: TaskStatus) -> RenameResult
pub fn insert_result(conn: &Connection, result: &RenameResult) -> Result<(), rusqlite::Error>
pub fn get_results_by_task(conn: &Connection, task_id: &str) -> Result<Vec<RenameResult>, rusqlite::Error>

// audit/db/log_repository.rs - 日志仓库
pub struct AuditLogEntry { pub id, pub task_id, pub event_type, pub message, pub created_at }
pub fn create_log_entry(task_id: Option<&str>, event_type: &str, message: &str) -> AuditLogEntry
pub fn insert_log(conn: &Connection, entry: &AuditLogEntry) -> Result<(), rusqlite::Error>
pub fn get_logs_by_task(conn: &Connection, task_id: &str) -> Result<Vec<AuditLogEntry>, rusqlite::Error>
pub fn get_all_logs(conn: &Connection) -> Result<Vec<AuditLogEntry>, rusqlite::Error>

// audit/db/mod.rs - re-export hub（向后兼容）
pub use task_status::TaskStatus;
pub use task_repository::{create_task, get_task, insert_task, update_task_status, RenameTask};
pub use result_repository::{create_result, get_results_by_task, insert_result, RenameResult};
pub use log_repository::{create_log_entry, get_all_logs, get_logs_by_task, insert_log, AuditLogEntry};
pub use schema::init_tables;

// audit/redaction.rs - 敏感字段脱敏
pub fn redact_text(input: &str) -> String
pub fn contains_sensitive(text: &str) -> bool

// audit/logger.rs - 审计日志器
pub struct AuditLogger<'a> { conn: &'a Connection }
impl AuditLogger<'a> {
    pub fn new(conn: &'a Connection) -> Self
    pub fn log_event(&self, task_id: Option<&str>, event_type: &str, message: &str) -> Result<AuditLogEntry, rusqlite::Error>
    pub fn log_failure(&self, task_id: Option<&str>, error: &str) -> Result<AuditLogEntry, rusqlite::Error>
    pub fn log_preview(&self, task_id: &str, file_count: u32) -> Result<AuditLogEntry, rusqlite::Error>
    pub fn log_task_created(&self, task_id: &str, template: &str) -> Result<AuditLogEntry, rusqlite::Error>
    pub fn log_execution_plan(&self, task_id: &str, file_count: u32) -> Result<AuditLogEntry, rusqlite::Error>
}

// audit/exporter.rs - JSONL 导出
pub struct JsonlExporter<'a> { conn: &'a Connection }
impl JsonlExporter<'a> {
    pub fn new(conn: &'a Connection) -> Self
    pub fn export_all(&self) -> Result<String, Box<dyn std::error::Error>>
    pub fn export_by_task(&self, task_id: &str) -> Result<String, Box<dyn std::error::Error>>
}
pub fn entry_to_json(entry: &AuditLogEntry) -> Result<String, serde_json::Error>

// 数据结构定义在各自仓库文件中
// audit/db/task_status.rs: TaskStatus enum
// audit/db/task_repository.rs: RenameTask struct
// audit/db/result_repository.rs: RenameResult struct
// audit/db/log_repository.rs: AuditLogEntry struct
```

### 2.6 config 模块

```rust
// config/template_manager.rs
pub fn get_template(media_type: &MediaType) -> String
pub fn set_template(media_type: &MediaType, template: &str) -> Result<()>
pub fn get_all_templates() -> Vec<RenameRule>

// config/threshold.rs
pub fn get_threshold() -> u8
pub fn set_threshold(value: u8) -> Result<()>

// config/secret/api_key_store.rs
pub fn get_api_key(service: &str) -> Result<Option<String>>
pub fn save_api_key(service: &str, api_key: &str) -> Result<()>
pub fn delete_api_key(service: &str) -> Result<()>
pub fn mask_api_key(api_key: &str) -> String

// config/secret/redaction.rs
pub fn redact_sensitive_data(data: &str) -> String
pub fn is_sensitive_field(field_name: &str) -> bool

// config/user_settings/metadata_settings.rs
pub fn get_tmdb_settings() -> Result<TmdbSettings>
pub fn save_tmdb_settings(settings: &TmdbSettings) -> Result<()>
```

### 2.7 metadata 模块

```rust
// metadata/provider/metadata_provider.rs
pub trait MetadataProvider {
    fn search_movie(&self, title: &str, year: Option<u32>) -> Result<Vec<MovieResult>>;
    fn search_series(&self, title: &str, year: Option<u32>) -> Result<Vec<SeriesResult>>;
    fn get_movie_details(&self, id: &str) -> Result<MovieDetails>;
    fn get_series_details(&self, id: &str) -> Result<SeriesDetails>;
}

// metadata/provider/metadata_query.rs
pub fn query_movie_metadata(title: &str, year: Option<u32>) -> Result<Vec<MovieResult>>
pub fn query_series_metadata(title: &str, year: Option<u32>) -> Result<Vec<SeriesResult>>

// metadata/provider/metadata_match.rs
pub fn match_movie_metadata(local_info: &ParsedMediaInfo, results: &[MovieResult]) -> Option<MovieResult>
pub fn match_series_metadata(local_info: &ParsedMediaInfo, results: &[SeriesResult]) -> Option<SeriesResult>

// metadata/tmdb/tmdb_client.rs
pub struct TmdbClient {
    api_key: String,
    base_url: String,
}
impl TmdbClient {
    pub fn new(api_key: &str) -> Self;
    pub fn search_movie(&self, title: &str, year: Option<u32>) -> Result<Vec<TmdbMovieResult>>;
    pub fn search_series(&self, title: &str, year: Option<u32>) -> Result<Vec<TmdbSeriesResult>>;
    pub fn get_movie_details(&self, movie_id: u32) -> Result<TmdbMovieDetails>;
    pub fn get_series_details(&self, series_id: u32) -> Result<TmdbSeriesDetails>;
    pub fn test_connection(&self) -> Result<()>;
}

// metadata/tmdb/tmdb_config.rs
pub fn get_tmdb_api_key() -> Result<Option<String>>
pub fn save_tmdb_api_key(api_key: &str) -> Result<()>
pub fn clear_tmdb_api_key() -> Result<()>
pub fn is_tmdb_configured() -> bool

// metadata/tmdb/tmdb_error.rs
#[derive(Debug, thiserror::Error)]
pub enum TmdbError {
    #[error("API key not configured")]
    ApiKeyNotConfigured,
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Movie not found")]
    MovieNotFound,
    #[error("Series not found")]
    SeriesNotFound,
}

// metadata/tmdb/tmdb_mapper.rs
pub fn map_movie_to_parsed_info(tmdb_movie: &TmdbMovieDetails) -> ParsedMediaInfo
pub fn map_series_to_parsed_info(tmdb_series: &TmdbSeriesDetails) -> ParsedMediaInfo
```

### 2.8 shared 模块

```rust
// shared/path_utils.rs
pub fn sanitize_filename(name: &str) -> String
pub fn is_path_too_long(path: &str) -> bool
pub fn has_invalid_chars(path: &str) -> bool
pub fn normalize_path(path: &str) -> String

// shared/result_types.rs
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Conflict error: {0}")]
    Conflict(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("TMDb error: {0}")]
    Tmdb(#[from] TmdbError),
}
```

---

## 三、UI ↔ Rust 数据契约

### 3.1 序列化约定

所有 IPC 数据使用 `serde` + `serde_json` 序列化。

```rust
// Rust 侧
#[derive(Serialize, Deserialize)]
pub struct MediaItem { ... }

// TypeScript 侧
interface MediaItem { ... }
```

### 3.2 类型映射

| Rust 类型 | TypeScript 类型 | 备注 |
|-----------|-----------------|------|
| String | string | |
| u8/u16/u32/u64 | number | |
| bool | boolean | |
| Option\<T\> | T \| null | |
| Vec\<T\> | T[] | |
| DateTime | string (ISO 8601) | serde 自动转换 |
| serde_json::Value | any | |

### 3.3 枚举映射

```rust
// Rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Movie,
    Series,
    Anime,
    Special,
    Ova,
    Ncop,
    Nced,
    Extras,
    Unknown,
}
```

```typescript
// TypeScript
type MediaType = 'movie' | 'series' | 'anime' | 'special' | 'ova' | 'ncop' | 'nced' | 'extras' | 'unknown';
```

---

## 四、事件契约

### 4.1 事件命名规范

- 格式：`{module}-{action}`
- 示例：`scan-progress`, `rename-progress`, `rollback-progress`

### 4.2 事件数据结构

```typescript
// 扫描进度
interface ScanProgressEvent {
  scanned: number;
  total: number;
  currentFile: string;
}

// 改名进度
interface RenameProgressEvent {
  completed: number;
  total: number;
  currentFile: string;
  status: 'renaming' | 'success' | 'failed' | 'skipped';
}

// 回滚进度
interface RollbackProgressEvent {
  completed: number;
  total: number;
  currentFile: string;
  status: 'rolling-back' | 'success' | 'failed';
}
```

### 4.3 事件监听（UI 侧）

```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<ScanProgressEvent>('scan-progress', (event) => {
  // 更新 UI 进度条
});
```

---

## 五、数据库 Schema

### 5.1 rename_tasks 表

```sql
CREATE TABLE rename_tasks (
    id TEXT PRIMARY KEY,
    scan_path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    completed_at TEXT,
    status TEXT NOT NULL,
    total_files INTEGER NOT NULL,
    renamed_count INTEGER NOT NULL DEFAULT 0,
    skipped_count INTEGER NOT NULL DEFAULT 0,
    failed_count INTEGER NOT NULL DEFAULT 0
);
```

### 5.2 rename_results 表

```sql
CREATE TABLE rename_results (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL REFERENCES rename_tasks(id),
    before_path TEXT NOT NULL,
    after_path TEXT NOT NULL,
    status TEXT NOT NULL,
    error TEXT,
    rule_source TEXT NOT NULL,
    executed_at TEXT NOT NULL
);
```

### 5.3 rollback_records 表

```sql
CREATE TABLE rollback_records (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL REFERENCES rename_tasks(id),
    created_at TEXT NOT NULL,
    status TEXT NOT NULL
);
```

### 5.4 audit_log 表

```sql
CREATE TABLE audit_log (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    event_type TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TEXT NOT NULL
);
```

---

## 六、配置文件 Schema

### 6.1 config.toml

```toml
[templates]
movie = "{Title} ({Year}) [{Resolution} {Source} {VideoCodec} {AudioCodec}].{ext}"
series = "{SeriesTitle} - S{Season:02}E{Episode:02} - {EpisodeTitle}.{ext}"
anime = "{SeriesTitle} - S{Season:02}E{Episode:02} [{Group}][{Resolution}].{ext}"
special = "{SeriesTitle} - S00E{SpecialNumber:02} - {SpecialTitle}.{ext}"
extras = "{SeriesTitle} - Extra - {ExtraType}{ExtraNumber}.{ext}"

[thresholds]
confidence = 70

[whitelist]
video_extensions = [".mkv", ".mp4", ".avi", ".mov", ".wmv", ".flv", ".webm", ".ts", ".m2ts", ".mts", ".mpg", ".mpeg", ".m4v"]
companion_extensions = [".srt", ".ass", ".ssa", ".sub", ".idx", ".nfo", ".jpg", ".jpeg", ".png", ".webp", ".txt"]

[limits]
max_files = 100000
max_path_length = 260
```

---

*下一阶段：error-handling-strategy.md*
