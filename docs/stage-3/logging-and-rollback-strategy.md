# 日志与回滚策略

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、日志架构

### 1.1 日志层级

```
┌─────────────────────────────┐
│    tracing (Rust 日志框架)    │
├─────────────────────────────┤
│   tracing-subscriber         │
│   ├── stdout (开发)          │
│   └── file (生产)            │
├─────────────────────────────┤
│   audit::logger              │
│   └── SQLite audit_logs 表   │
└─────────────────────────────┘
```

### 1.2 双轨日志

| 轨道 | 用途 | 存储 | 格式 |
|------|------|------|------|
| 运行日志 | 调试、排错 | 文件 (tracing) | 结构化文本 |
| 审计日志 | 业务追踪、回滚依据 | SQLite | 结构化记录 |

---

## 二、运行日志

### 2.1 配置

```rust
use tracing_subscriber::{fmt, EnvFilter};

tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env()
        .add_directive("media_renamer=info".parse().unwrap()))
    .with_target(true)
    .with_thread_ids(true)
    .with_file(true)
    .with_line_number(true)
    .init();
```

### 2.2 日志级别使用规范

| 级别 | 用途 | 示例 |
|------|------|------|
| error | 操作失败 | `改名失败: {path}: permission denied` |
| warn | 异常但可继续 | `低置信度: {filename}: confidence={score}` |
| info | 关键操作 | `扫描完成: {count} files found` |
| debug | 开发调试 | `解析结果: {parsed_info:?}` |
| trace | 详细追踪 | `检查文件: {path}` |

### 2.3 日志文件

```
logs/
├── app-2026-06-08.log    # 按日期滚动
├── app-2026-06-09.log
└── app-latest.log        # 符号链接到最新
```

**保留策略**：保留最近 7 天日志。

---

## 三、审计日志

### 3.1 设计原则

1. **不可篡改**：只追加，不修改，不删除
2. **完整记录**：所有关键操作必须记录
3. **可追溯**：包含时间戳、操作者、详情
4. **可导出**：支持 JSONL 格式导出

### 3.2 审计事件清单

| 事件 | 触发点 | 必需字段 |
|------|--------|----------|
| task_created | 创建任务 | taskId, scanPath |
| scan_started | 开始扫描 | taskId, path |
| scan_completed | 扫描完成 | taskId, totalFiles, videoCount, duration |
| scan_failed | 扫描失败 | taskId, error |
| parse_started | 开始解析 | taskId |
| parse_completed | 解析完成 | taskId, totalItems |
| preview_generated | 生成预览 | taskId, previewCount |
| manual_review_applied | 人工确认 | taskId, reviewedCount |
| rename_started | 开始改名 | taskId, totalFiles, dryRun |
| file_renamed | 单文件改名 | taskId, beforePath, afterPath, status, ruleSource |
| rename_completed | 改名完成 | taskId, renamedCount, skippedCount, failedCount |
| rename_failed | 改名失败 | taskId, error |
| rollback_started | 开始回滚 | taskId |
| file_rolled_back | 单文件回滚 | taskId, afterPath, beforePath, status |
| rollback_completed | 回滚完成 | rollbackId, status |
| rollback_failed | 回滚失败 | taskId, error |
| config_changed | 配置变更 | field, oldValue, newValue |
| audit_exported | 导出日志 | taskId, format, filePath |

### 3.3 审计日志条目结构

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: String,           // UUID
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,      // Info, Warn, Error
    pub action: String,       // 事件名
    pub task_id: Option<String>,
    pub details: serde_json::Value,  // 事件详情
}
```

### 3.4 记录方式

```rust
// 使用宏简化调用
macro_rules! audit_log {
    ($action:expr, $details:expr) => {
        audit::logger::log(AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            level: LogLevel::Info,
            action: $action.to_string(),
            task_id: None,
            details: serde_json::to_value($details).unwrap(),
        });
    };

    ($action:expr, $task_id:expr, $details:expr) => {
        audit::logger::log(AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            level: LogLevel::Info,
            action: $action.to_string(),
            task_id: Some($task_id.to_string()),
            details: serde_json::to_value($details).unwrap(),
        });
    };
}

// 使用
audit_log!("file_renamed", task.id, {
    "beforePath": before_path,
    "afterPath": after_path,
    "status": "success",
    "ruleSource": rule_source,
});
```

---

## 四、回滚策略

### 4.1 回滚原理

基于 `rename_results` 表中的 `before_path` 和 `after_path` 映射，将已改名的文件恢复为原名。

```
执行时: fs::rename(before_path, after_path)
回滚时: fs::rename(after_path, before_path)
```

### 4.2 回滚前置检查

```rust
fn check_rollback_prerequisites(task_id: &str) -> Result<RollbackCheckResult, AppError> {
    let results = get_rename_results(task_id)?;

    let mut check = RollbackCheckResult::new();

    for result in &results {
        if result.status != RenameStatus::Success {
            continue; // 跳过失败/跳过的项
        }

        let after_path = Path::new(&result.after_path);
        let before_path = Path::new(&result.before_path);

        // 检查 1: after_path 是否存在
        if !after_path.exists() {
            check.add_issue(RollbackIssue::TargetNotFound {
                path: result.after_path.clone(),
            });
            continue;
        }

        // 检查 2: before_path 是否被占用
        if before_path.exists() {
            check.add_issue(RollbackIssue::SourcePathOccupied {
                path: result.before_path.clone(),
            });
            continue;
        }

        // 检查 3: 权限
        if !is_writable(after_path) {
            check.add_issue(RollbackIssue::PermissionDenied {
                path: result.after_path.clone(),
            });
            continue;
        }

        check.add_rollbackable(result.clone());
    }

    Ok(check)
}
```

### 4.3 回滚执行流程

```
1. 加载任务记录
   │  从 DB 读取 RenameTask + RenameResult[]
   ↓
2. 前置检查
   │  检查 afterPath 存在性
   │  检查 beforePath 可用性
   │  检查权限
   ↓
3. 创建 RollbackRecord
   │  status = InProgress
   │  持久化到 DB
   ↓
4. 逐文件回滚
   │  for each success_result:
   │    a. fs::rename(after_path, before_path)
   │    b. 记录 RollbackResult
   │    c. 推送进度事件
   │    d. 冲突时暂停
   ↓
5. 更新 RollbackRecord
   │  status = Completed / PartialFailed / Failed
   ↓
6. 更新原 RenameTask
   │  status = RolledBack
   ↓
7. 记录审计日志
```

### 4.4 回滚状态机

```
          ┌──────────┐
          │  Pending  │
          └─────┬────┘
                │ 开始回滚
                ↓
          ┌──────────┐
          │InProgress │
          └─────┬────┘
        ┌───────┼───────┐
        ↓       ↓       ↓
  ┌────────┐┌────────┐┌────────┐
  │Completed││Partial ││ Failed │
  │         ││ Failed ││        │
  └────────┘└────────┘└────────┘
```

### 4.5 部分回滚

**场景**：回滚过程中遇到冲突，部分文件无法回滚。

**策略**：
1. 记录已成功回滚的文件
2. 记录失败的文件及原因
3. RollbackRecord.status = PartialFailed
4. 用户可查看失败项，手动处理后重试

```rust
pub struct RollbackResult {
    pub before_path: String,
    pub after_path: String,
    pub status: RollbackStatus,
    pub error: Option<String>,
}

pub enum RollbackStatus {
    Success,
    Failed,
    Skipped,  // 原任务中失败/跳过的项
}
```

---

## 五、崩溃恢复

### 5.1 持久化时机

| 时机 | 持久化内容 |
|------|------------|
| 任务创建 | RenameTask (status=Created) |
| 扫描完成 | RenameTask (status=Scanning) + ScanReport |
| 执行开始 | RenameTask (status=Executing) |
| 每个文件改名后 | RenameResult (立即写入) |
| 执行完成 | RenameTask (status=Completed) |

### 5.2 恢复流程

```rust
fn check_unfinished_tasks() -> Vec<RenameTask> {
    // 查询 status = Created/Scanning/Executing 的任务
    // 这些任务可能因崩溃而未完成
    db::query_unfinished_tasks()
}

fn recover_task(task: RenameTask) -> RecoveryAction {
    match task.status {
        TaskStatus::Created | TaskStatus::Scanning => {
            // 未开始执行，可安全删除任务
            RecoveryAction::DeleteTask
        }
        TaskStatus::Executing => {
            // 部分执行，检查已完成的文件
            let results = db::get_rename_results(&task.id);
            let success_count = results.iter()
                .filter(|r| r.status == RenameStatus::Success)
                .count();

            if success_count == 0 {
                RecoveryAction::DeleteTask
            } else {
                // 有成功改名的文件，标记为可回滚
                RecoveryAction::MarkAsInterrupted {
                    renamed_count: success_count as u32,
                }
            }
        }
        _ => RecoveryAction::None,
    }
}
```

### 5.3 启动时恢复检查

```rust
// 应用启动时
fn on_app_start() {
    let unfinished = check_unfinished_tasks();
    if !unfinished.is_empty() {
        // 通知用户
        for task in unfinished {
            match recover_task(task) {
                RecoveryAction::DeleteTask => {
                    db::delete_task(&task.id);
                }
                RecoveryAction::MarkAsInterrupted { renamed_count } => {
                    db::update_task_status(&task.id, TaskStatus::Interrupted);
                    // 提示用户：有中断的任务，可查看或回滚
                }
                RecoveryAction::None => {}
            }
        }
    }
}
```

---

## 六、日志导出

### 6.1 JSONL 格式

每行一个 JSON 对象：

```jsonl
{"id":"uuid-1","timestamp":"2026-06-08T15:30:00Z","level":"info","action":"task_created","taskId":"task-1","details":{"scanPath":"D:\\Media"}}
{"id":"uuid-2","timestamp":"2026-06-08T15:30:01Z","level":"info","action":"scan_started","taskId":"task-1","details":{"path":"D:\\Media"}}
{"id":"uuid-3","timestamp":"2026-06-08T15:30:05Z","level":"info","action":"scan_completed","taskId":"task-1","details":{"totalFiles":150,"videoCount":120,"duration":"4.2s"}}
```

### 6.2 导出路径

```
exports/
├── task-1-20260608-153000.jsonl
├── task-2-20260609-100000.jsonl
└── ...
```

---

*下一阶段：ui-state-model.md*
