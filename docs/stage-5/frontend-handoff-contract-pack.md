# Frontend Handoff Contract Pack

**Date**: 2026-06-09  
**Branch**: backend-continuation  
**HEAD**: ab9c12a docs(backend): add release safety gate audit  

---

## 1. Current Backend Status

| Item | Status |
|------|--------|
| Branch | `backend-continuation` |
| HEAD | `ab9c12a` |
| Dry-run MVP | ✅ READY / SAFE |
| Real Rename | CORE READY / EXPOSURE DEFERRED |
| Rollback | CORE READY / EXPOSURE DEFERRED |
| cargo test | 296 tests PASS |

---

## 2. Exposed Command List

| # | Command | 功能族 | 只读 | 写配置 | 触发媒体文件变更 | 前端推荐使用场景 |
|---|---------|--------|------|--------|-----------------|-----------------|
| 1 | `start_rename_session` | session | ❌ | ❌ | ❌ (dry-run only) | 扫描目录并生成重命名预览 |
| 2 | `get_app_config` | config | ✅ | ❌ | ❌ | 首屏加载配置 |
| 3 | `get_all_templates` | config | ✅ | ❌ | ❌ | 展示模板列表 |
| 4 | `set_template` | config | ❌ | ✅ | ❌ | 修改重命名模板 |
| 5 | `get_confidence_threshold` | config | ✅ | ❌ | ❌ | 获取置信度阈值 |
| 6 | `set_confidence_threshold` | config | ❌ | ✅ | ❌ | 修改置信度阈值 |
| 7 | `get_task` | audit | ✅ | ❌ | ❌ | 查询单个任务详情 |
| 8 | `get_all_tasks` | audit | ✅ | ❌ | ❌ | 展示任务历史列表 |
| 9 | `get_audit_logs` | audit | ✅ | ❌ | ❌ | 查询任务审计日志 |

**确认**：当前暴露的 commands 中没有真实 rename / rollback。

---

## 3. start_rename_session Contract

### 输入参数

```typescript
interface StartRenameSessionRequest {
  directory: string;  // 目录路径
}
```

### 返回结构

```typescript
interface PipelineResult {
  task_id: string;           // 审计任务 ID（已写入数据库）
  scan: ScanSummary;         // 扫描摘要
  previews: RenamePreviewItem[];  // 重命名预览列表
  safety: SafetyReport;      // 安全检查报告
  parsed_count: number;      // 成功解析的文件数
  unknown_count: number;     // 未识别的文件数
}

interface ScanSummary {
  scan_path: string;             // 扫描目录
  video_count: number;           // 视频文件数
  companion_count: number;       // 伴随文件数
  permission_error_count: number; // 权限错误数
  scan_duration_ms: number;      // 扫描耗时（毫秒）
}
```

### 示例 Payload

#### 空目录返回示例

```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "scan": {
    "scan_path": "/path/to/empty",
    "video_count": 0,
    "companion_count": 0,
    "permission_error_count": 0,
    "scan_duration_ms": 5
  },
  "previews": [],
  "safety": {
    "can_execute": true,
    "dry_run": true,
    "checks": [],
    "blocking_reasons": []
  },
  "parsed_count": 0,
  "unknown_count": 0
}
```

#### 正常媒体目录返回示例

```json
{
  "task_id": "550e8400-e29b-41d4-a716-446655440001",
  "scan": {
    "scan_path": "/path/to/movies",
    "video_count": 3,
    "companion_count": 5,
    "permission_error_count": 0,
    "scan_duration_ms": 120
  },
  "previews": [
    {
      "id": "preview-0",
      "source_path": "/path/to/movies/The.Matrix.1999.1080p.mkv",
      "original_name": "The.Matrix.1999.1080p.mkv",
      "proposed_name": "The Matrix (1999) [1080p].mkv",
      "target_path": "/path/to/movies/The Matrix (1999) [1080p].mkv",
      "media_type": "Movie",
      "confidence": 95,
      "needs_manual_review": false,
      "should_skip": false,
      "conflicts": [],
      "evidence": [],
      "metadata_source": "LocalRule"
    }
  ],
  "safety": {
    "can_execute": true,
    "dry_run": true,
    "checks": [
      {"name": "confidence", "passed": true, "message": "All items have sufficient confidence"},
      {"name": "manual_review", "passed": true, "message": "No items require manual review"},
      {"name": "conflicts", "passed": true, "message": "No blocking conflicts detected"}
    ],
    "blocking_reasons": []
  },
  "parsed_count": 3,
  "unknown_count": 0
}
```

#### Low Confidence 返回示例

```json
{
  "previews": [
    {
      "id": "preview-0",
      "confidence": 35,
      "needs_manual_review": true,
      "should_skip": true
    }
  ],
  "safety": {
    "can_execute": false,
    "dry_run": true,
    "checks": [
      {"name": "confidence", "passed": false, "message": "1 items have low confidence (< 70)"},
      {"name": "manual_review", "passed": false, "message": "1 items require manual review"}
    ],
    "blocking_reasons": ["Low confidence items: 1", "Manual review required: 1"]
  }
}
```

#### Conflict 返回示例

```json
{
  "previews": [
    {
      "id": "preview-0",
      "conflicts": [
        {
          "conflict_type": "TargetExists",
          "source_path": "/path/to/movies/Movie.mkv",
          "target_path": "/path/to/movies/Movie (2020).mkv",
          "message": "Target file already exists",
          "blocking": true
        }
      ]
    }
  ],
  "safety": {
    "can_execute": false,
    "dry_run": true,
    "checks": [
      {"name": "conflicts", "passed": false, "message": "1 blocking conflicts detected"}
    ],
    "blocking_reasons": ["Blocking conflicts: 1"]
  }
}
```

#### Error 返回示例

```typescript
// 错误以 String 形式返回
"Path does not exist: /nonexistent/path"
"Path is not a directory: /path/to/file.txt"
```

---

## 4. Config Commands Contract

### get_app_config

```typescript
// 无输入参数
// 返回 AppConfig
interface AppConfig {
  templates: TemplatesConfig;
  thresholds: ThresholdsConfig;
  limits: LimitsConfig;
}

interface TemplatesConfig {
  movie: string;
  series: string;
  anime: string;
  special: string;
  extras: string;
}

interface ThresholdsConfig {
  confidence: number;  // 0-100
}

interface LimitsConfig {
  max_files: number;
  max_path_length: number;
}
```

### get_all_templates

```typescript
// 无输入参数
// 返回 RenameRule[]
interface RenameRule {
  media_type: MediaType;
  template: string;
  is_default: boolean;
}

type MediaType = 'Movie' | 'Series' | 'Anime' | 'Special' | 'Ova' | 'Extras' | 'Unknown';
```

### set_template

```typescript
interface SetTemplateRequest {
  media_type: string;  // "movie" | "series" | "anime" | "special" | "ova" | "extras" | "unknown"
  template: string;    // 模板字符串
}
// 返回 void
```

### get_confidence_threshold

```typescript
// 无输入参数
// 返回 number (0-100)
```

### set_confidence_threshold

```typescript
interface SetConfidenceThresholdRequest {
  value: number;  // 0-100
}
// 返回 void
```

### API Key / Secret 字段规则

**规则**：前端不得展示明文 secret。

API Key 存储在单独的 `.api_keys.json` 文件中，不在 `config.toml` 中。

如果未来暴露 API key 相关 command，必须使用 `mask_api_key` 脱敏：
- 保留前 4 位和后 4 位
- 中间用 `*` 替代
- 示例：`abcd****efgh`

---

## 5. Audit Commands Contract

### get_task

```typescript
interface GetTaskRequest {
  task_id: string;
}
// 返回 RenameTask
interface RenameTask {
  id: string;
  status: TaskStatus;
  template: string;
  total_files: number;
  created_at: string;  // ISO 8601
  updated_at: string;  // ISO 8601
  error_message: string | null;
}

type TaskStatus = 'Previewing' | 'Pending' | 'Executing' | 'Completed' | 'Failed' | 'RolledBack';
```

### get_all_tasks

```typescript
// 无输入参数
// 返回 RenameTask[] (按 created_at 倒序)
```

### get_audit_logs

```typescript
interface GetAuditLogsRequest {
  task_id: string;
}
// 返回 AuditLogEntry[]
interface AuditLogEntry {
  id: string;
  task_id: string | null;
  event_type: string;
  message: string;
  created_at: string;  // ISO 8601
}
```

### 空结果示例

```json
// get_task (不存在)
null

// get_all_tasks (空)
[]

// get_audit_logs (空)
[]
```

### 有数据示例

```json
// get_task
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "Previewing",
  "template": "{Title} ({Year})",
  "total_files": 5,
  "created_at": "2026-06-09T06:00:00Z",
  "updated_at": "2026-06-09T06:00:00Z",
  "error_message": null
}

// get_audit_logs
[
  {
    "id": "log-001",
    "task_id": "550e8400-e29b-41d4-a716-446655440000",
    "event_type": "task_created",
    "message": "Task created with template: {Title} ({Year})",
    "created_at": "2026-06-09T06:00:00Z"
  }
]
```

### Redaction 规则

审计日志中的敏感字段会被自动脱敏：
- TMDb API Key → `***REDACTED***`
- 通用 API Key → `***REDACTED***`
- Token / Bearer → `***REDACTED***`
- Secret / Password → `***REDACTED***`
- URL Query 中的 key 参数 → `***REDACTED***`
- JSON 字段中的敏感值 → `***REDACTED***`

---

## 6. Safety Status Mapping

### SafetyReport 字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `can_execute` | boolean | 是否可以执行（所有检查通过） |
| `dry_run` | boolean | 是否为预演模式（当前始终 true） |
| `checks` | SafetyCheck[] | 检查结果列表 |
| `blocking_reasons` | string[] | 阻塞原因列表 |

### SafetyCheck 字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 检查名称 |
| `passed` | boolean | 是否通过 |
| `message` | string | 描述信息 |

### 前端 Badge/Status 推荐映射

| Safety Status | Badge 颜色 | 文案 |
|---------------|-----------|------|
| `can_execute: true` | ✅ 绿色 | 就绪 |
| `can_execute: false` + low confidence | ⚠️ 黄色 | 低置信度 |
| `can_execute: false` + manual_review | ⚠️ 黄色 | 需审核 |
| `can_execute: false` + conflict | ❌ 红色 | 有冲突 |

---

## 7. Preview Item Mapping

### RenamePreviewItem 字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | string | 预览 ID |
| `source_path` | string | 原始路径 (before) |
| `original_name` | string | 原始文件名 |
| `proposed_name` | string | 提议的新文件名 (after) |
| `target_path` | string | 目标路径 |
| `media_type` | MediaType | 媒体类型 |
| `confidence` | number | 置信度 (0-100) |
| `needs_manual_review` | boolean | 是否需要人工确认 |
| `should_skip` | boolean | 是否跳过 |
| `conflicts` | RenameConflict[] | 冲突列表 |
| `evidence` | RuleMatchEvidence[] | 置信度证据 |
| `metadata_source` | MetadataSource | 元数据来源 |

### 前端表格列建议

| 列名 | 字段 | 说明 |
|------|------|------|
| 原文件名 | `original_name` | 原始文件名 |
| 新文件名 | `proposed_name` | 提议的新文件名 |
| 类型 | `media_type` | 媒体类型 |
| 置信度 | `confidence` | 置信度百分比 |
| 状态 | `needs_manual_review` + `conflicts` | 就绪/需审核/有冲突 |
| 操作 | - | 编辑/跳过 |

---

## 8. UI Do / Don't

### Do

- ✅ 默认只展示 dry-run preview
- ✅ 展示 safety report
- ✅ 展示 audit task/log
- ✅ 允许用户查看配置

### Don't

- ❌ 不显示真实 rename 按钮
- ❌ 不显示 rollback 按钮
- ❌ real rename / rollback 入口必须隐藏或 disabled
- ❌ 用户确认 UI 未完成前，不得暴露 destructive action
- ❌ 不得展示明文 API key / secret

---

## 9. Deferred Commands

| Command | Status | 原因 | 未来开启条件 |
|---------|--------|------|-------------|
| `execute_rename` | CORE READY / COMMAND DEFERRED | 需要前端确认 UI | 完成用户确认 UI 后暴露 |
| `rollback_task` | CORE READY / COMMAND DEFERRED | 需要前端确认 UI | 完成用户确认 UI 后暴露 |

---

## 10. Frontend Integration Checklist

1. **首屏加载配置**
   - 调用 `get_app_config` 获取配置
   - 调用 `get_all_templates` 获取模板列表
   - 调用 `get_confidence_threshold` 获取阈值

2. **选择目录**
   - 使用 Tauri dialog 插件选择目录
   - 将目录路径传给 `start_rename_session`

3. **调用 start_rename_session**
   - 传入目录路径
   - 获取 PipelineResult

4. **展示 preview**
   - 遍历 `previews` 数组
   - 展示原文件名 → 新文件名
   - 展示置信度和状态

5. **展示 safety report**
   - 展示 `can_execute` 状态
   - 展示 `checks` 列表
   - 展示 `blocking_reasons`

6. **查询 audit task/log**
   - 使用 `task_id` 调用 `get_task` 获取任务详情
   - 使用 `task_id` 调用 `get_audit_logs` 获取审计日志

7. **禁止真实执行**
   - 不暴露 `execute_rename` 按钮
   - 不暴露 `rollback_task` 按钮
   - 所有操作只展示 dry-run 结果
