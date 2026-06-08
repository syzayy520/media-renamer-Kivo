# 错误处理策略

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、错误分类

### 1.1 按严重程度

| 级别 | 类型 | 示例 | 处理策略 |
|------|------|------|----------|
| FATAL | 系统级 | DB 不可写、应用目录损坏 | 弹窗提示 + 退出 |
| ERROR | 操作级 | 文件改名失败、权限不足 | 记录 + 跳过 + 继续 |
| WARN | 数据级 | 低置信度、路径过长 | 标记 + 允许用户决策 |
| INFO | 状态级 | 扫描完成、任务创建 | 正常日志 |

### 1.2 按模块

| 模块 | 错误类型 | 来源 |
|------|----------|------|
| scan/ | IoError, PermissionDenied, PathTooLong | std::fs |
| parse/ | ParseError, UnsupportedFormat | 正则匹配 |
| rename/ | ConflictError, SafetyCheckFailed, IoError | 冲突检测 + fs |
| rollback/ | RollbackConflict, TargetNotFound, IoError | 状态检查 + fs |
| audit/ | DatabaseError | rusqlite |
| config/ | ConfigError, IoError | toml 解析 |

---

## 二、错误类型定义

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // 基础错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    // 扫描错误
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("Path too long ({length} > {max}): {path}")]
    PathTooLong { path: String, length: usize, max: usize },

    #[error("Circular symlink detected: {path}")]
    CircularSymlink { path: String },

    #[error("Max file limit reached: {limit}")]
    MaxFileLimitReached { limit: u32 },

    // 解析错误
    #[error("Parse failed: {reason}")]
    ParseFailed { reason: String },

    #[error("Unknown media format: {filename}")]
    UnknownFormat { filename: String },

    // 重命名错误
    #[error("Conflict: {conflict_type}: {message}")]
    Conflict {
        conflict_type: ConflictType,
        message: String,
    },

    #[error("Safety check failed: {check_name}: {reason}")]
    SafetyCheckFailed { check_name: String, reason: String },

    #[error("Rename failed: {before} -> {after}: {reason}")]
    RenameFailed {
        before: String,
        after: String,
        reason: String,
    },

    // 回滚错误
    #[error("Rollback conflict: {message}")]
    RollbackConflict { message: String },

    #[error("Source file not found: {path}")]
    SourceNotFound { path: String },

    #[error("Target already exists: {path}")]
    TargetAlreadyExists { path: String },

    // 配置错误
    #[error("Config error: {0}")]
    Config(String),

    // 任务错误
    #[error("Task not found: {task_id}")]
    TaskNotFound { task_id: String },

    #[error("Task not in rollbackable state: {status}")]
    TaskNotRollbackable { status: String },
}
```

---

## 三、错误处理模式

### 3.1 Rust 核心层

**原则**：使用 `Result<T, AppError>` 传播，不 panic。

```rust
// ✅ 正确：Result 传播
fn rename_file(item: &RenamePreviewItem) -> Result<RenameResult, AppError> {
    let source = Path::new(&item.originalPath);
    if !source.exists() {
        return Err(AppError::SourceNotFound {
            path: item.originalPath.clone(),
        });
    }
    // ...
}

// ❌ 错误：panic
fn rename_file(item: &RenamePreviewItem) -> RenameResult {
    assert!(Path::new(&item.originalPath).exists()); // 不要这样做
    // ...
}
```

### 3.2 Tauri IPC 层

**原则**：将 `AppError` 转换为 `String` 返回给前端。

```rust
#[tauri::command]
async fn execute_rename(
    preview_items: Vec<RenamePreviewItem>,
    dry_run: bool,
    app_handle: AppHandle,
) -> Result<RenameTask, String> {
    rename::executor::execute_batch(&preview_items, dry_run, &app_handle)
        .map_err(|e| e.to_string())
}
```

### 3.3 UI 层

**原则**：捕获错误，显示用户友好的提示。

```typescript
try {
  const task = await invoke('execute_rename', { previewItems, dryRun: false });
  executionStore.setTask(task);
} catch (error) {
  // 解析错误类型
  const errorMsg = typeof error === 'string' ? error : 'Unknown error';
  executionStore.setError(errorMsg);
  // 显示 toast 通知
  showToast({ type: 'error', message: `执行失败: ${errorMsg}` });
}
```

---

## 四、批量操作错误处理

### 4.1 扫描阶段

**策略**：记录权限错误，继续扫描。

```rust
fn scan_directory(path: &Path) -> Result<(ScanReport, Vec<MediaItem>), AppError> {
    let mut report = ScanReport::new();
    let mut items = Vec::new();

    for entry in walkdir::WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                // 正常处理
            }
            Err(e) => {
                // 记录权限错误，继续
                report.permission_errors.push(e.to_string());
                continue;
            }
        }
    }

    Ok((report, items))
}
```

### 4.2 执行阶段

**策略**：单文件失败不影响其他文件。

```rust
fn execute_batch(items: &[RenamePreviewItem]) -> Vec<RenameResult> {
    let mut results = Vec::new();

    for item in items {
        match execute_single(item) {
            Ok(result) => results.push(result),
            Err(e) => {
                results.push(RenameResult {
                    status: RenameStatus::Failed,
                    error: Some(e.to_string()),
                    // ...
                });
                // 继续处理下一个文件
                continue;
            }
        }
    }

    results
}
```

### 4.3 回滚阶段

**策略**：冲突时暂停，报告失败项。

```rust
fn rollback_task(task_id: &str) -> Result<RollbackRecord, AppError> {
    let results = get_rename_results(task_id)?;
    let mut rollback_results = Vec::new();

    for result in &results {
        match rollback_single(result) {
            Ok(rollback_result) => rollback_results.push(rollback_result),
            Err(AppError::RollbackConflict { message }) => {
                // 冲突：暂停回滚
                return Ok(RollbackRecord {
                    status: RollbackStatus::PartialFailed,
                    results: rollback_results,
                    // ...
                });
            }
            Err(e) => {
                rollback_results.push(RollbackResult {
                    status: RollbackStatus::Failed,
                    error: Some(e.to_string()),
                    // ...
                });
            }
        }
    }

    Ok(RollbackRecord {
        status: RollbackStatus::Completed,
        results: rollback_results,
        // ...
    })
}
```

---

## 五、用户提示规范

### 5.1 错误提示类型

| 类型 | 组件 | 用途 |
|------|------|------|
| Toast | 轻量通知 | 操作成功/失败的即时反馈 |
| Alert | 弹窗 | 需要用户确认的错误 |
| Inline | 行内提示 | 表单验证、字段级错误 |
| Banner | 横幅 | 全局性问题（如 DB 不可用） |

### 5.2 错误消息模板

```typescript
const ERROR_MESSAGES = {
  // 扫描
  SCAN_PERMISSION_DENIED: '无法访问部分文件/目录，已跳过',
  SCAN_PATH_TOO_LONG: '路径过长，已跳过',
  SCAN_MAX_LIMIT: '已达最大文件数量限制 ({limit})',

  // 解析
  PARSE_FAILED: '文件名解析失败: {filename}',

  // 执行
  RENAME_FAILED: '改名失败: {before} → {after}',
  RENAME_CONFLICT: '目标路径冲突: {path}',
  RENAME_SAFETY_CHECK: '安全检查未通过: {check}',

  // 回滚
  ROLLBACK_FAILED: '回滚失败: {reason}',
  ROLLBACK_CONFLICT: '回滚冲突: {path}',

  // 系统
  DB_ERROR: '数据库错误，请重启应用',
  UNKNOWN: '未知错误: {message}',
};
```

### 5.3 提示位置

| 错误类型 | 提示位置 |
|----------|----------|
| 单文件改名失败 | 表格行内 + Toast |
| 批量执行部分失败 | 执行结果页 + Toast |
| 安全检查失败 | 确认弹窗 + 行内标记 |
| 回滚冲突 | 弹窗确认 |
| 系统级错误 | 全局 Banner |

---

## 六、重试策略

| 场景 | 重试 | 策略 |
|------|------|------|
| 文件被占用 | 否 | 记录失败，用户手动重试 |
| 权限不足 | 否 | 记录失败，跳过 |
| 路径不存在 | 否 | 记录失败，跳过 |
| DB 写入失败 | 是 (3次) | 指数退避 |
| 网络路径断连 | 否 | 记录错误，继续 |

---

## 七、防崩溃策略

### 7.1 unwrap/expect 使用规则

```rust
// ❌ 禁止：可能 panic
let value = option.unwrap();

// ✅ 正确：使用 ok_or 转换
let value = option.ok_or(AppError::ParseFailed { reason: "..." })?;

// ⚠️ 仅限：测试代码和不可失败场景
let value = option.expect("invariant: must have value");
```

### 7.2 线程安全

```rust
// 跨线程共享状态使用 Arc<Mutex<T>>
use std::sync::{Arc, Mutex};

let state = Arc::new(Mutex::new(AppState::default()));
```

### 7.3 panic 恢复

```rust
// Tauri 主线程 panic 捕获
std::panic::set_hook(Box::new(|panic_info| {
    tracing::error!("Panic: {:?}", panic_info);
    // 记录到日志文件
}));
```

---

*下一阶段：logging-and-rollback-strategy.md*
