# 数据流

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、全局数据流

```
┌────────────┐    IPC     ┌────────────┐    IPC     ┌────────────┐
│  UI (React)│───────────→│  Rust App  │───────────→│  UI (React)│
│  invoke    │            │  Layer     │            │  listen    │
└────────────┘            └────────────┘            └────────────┘
                                │
                    ┌───────────┼───────────┐
                    ↓           ↓           ↓
              ┌──────────┐┌──────────┐┌──────────┐
              │ scan     ││ parse    ││ rename   │
              │ module   ││ module   ││ module   │
              └──────────┘└──────────┘└──────────┘
                    │           │           │
                    ↓           ↓           ↓
              ┌──────────────────────────────────┐
              │       shared (path_utils,        │
              │       result_types)              │
              └──────────────────────────────────┘
                                │
                    ┌───────────┼───────────┐
                    ↓           ↓           ↓
              ┌──────────┐┌──────────┐┌──────────┐
              │ fs (std) ││ db       ││ config   │
              │          ││(rusqlite)││ (toml)   │
              └──────────┘└──────────┘└──────────┘
```

---

## 二、扫描数据流

### 2.1 输入

| 来源 | 数据 | 类型 |
|------|------|------|
| UI invoke | 目录路径 | `String` |

### 2.2 处理链

```
1. scan::scanner::scan_directory(path: String) → Result<ScanReport>
   │  输入: path
   │  遍历: walkdir / std::fs::read_dir
   │  过滤: file_detector::is_video(ext) / is_companion(ext)
   │  输出: ScanReport { totalFiles, videoCount, companionCount, permissionErrors }
   │  事件: ScanProgress { scanned, total, current_file }
   ↓
2. scan::scanner → MediaItem[]
   │  每个文件生成 MediaItem
   │  字段填充: id(uuid), filePath, fileName, extension, fileSize, isVideo, isCompanion
   ↓
3. IPC → UI scanStore
   │  ScanReport + MediaItem[]
   │  事件: ScanProgress 通过 Tauri event 通道推送
```

### 2.3 输出

| 目标 | 数据 | 类型 |
|------|------|------|
| UI scanStore | ScanReport | 结构体 |
| UI scanStore | MediaItem[] | 数组 |
| UI event | ScanProgress | 事件 |

---

## 三、解析数据流

### 3.1 输入

| 来源 | 数据 | 类型 |
|------|------|------|
| UI invoke | MediaItem[] | 数组 |

### 3.2 处理链

```
1. parse::classifier::classify(mediaItems: MediaItem[]) → ClassifiedItems
   │  对每个 MediaItem 调用分类器
   │  分类优先级: 特别篇 > OVA > NCOP/NCED > Extras > 动漫 > 剧集 > 电影 > 未识别
   ↓
2. parse::movie_parser / series_parser / anime_parser / special_parser
   │  按分类结果调用对应解析器
   │  提取: title, year, season, episode, resolution, source, codec, group
   ↓
3. parse::confidence::score(parsed_info: ParsedMediaInfo) → u8
   │  评分规则:
   │  - 有标题+年份+季集: 90-100
   │  - 有标题+季集: 70-89
   │  - 仅有标题: 50-69
   │  - 无法识别: 0-49
   ↓
4. 输出 ParsedMediaInfo[]
   │  每个 ParsedMediaInfo 包含:
   │  - mediaItem (原始)
   │  - mediaType, title, year, season, episode
   │  - resolution, source, videoCodec, audioCodec, group
   │  - confidence (0-100)
   │  - ruleSources (命中的规则列表)
   │  - specialType, extraType, extraNumber (可选)
```

### 3.3 输出

| 目标 | 数据 | 类型 |
|------|------|------|
| UI previewStore | ParsedMediaInfo[] | 数组 |

---

## 四、预览数据流

### 4.1 输入

| 来源 | 数据 | 类型 |
|------|------|------|
| UI invoke | ParsedMediaInfo[] + template | 数组 + 字符串 |

### 4.2 处理链

```
1. rename::template::render(parsed: ParsedMediaInfo, template: String) → String
   │  解析模板变量: {Title}, {Year}, {Season}, {Episode}, etc.
   │  替换为 ParsedMediaInfo 字段值
   ↓
2. rename::conflict_detector::detect(previewItems) → Conflict[]
   │  检测:
   │  - TargetExists: 目标路径是否已存在
   │  - DuplicateTarget: 多源指向同一目标
   │  - PathTooLong: 路径超过 260 字符
   │  - InvalidChars: 非法字符
   │  - PermissionDenied: 权限不足
   │  - SourceNotFound: 源文件不存在
   ↓
3. rename::preview_generator::generate(parsedItems, template) → RenamePreviewItem[]
   │  组合模板渲染 + 冲突检测
   │  设置 needsReview (confidence < threshold)
   │  设置 riskLevel
```

### 4.3 输出

| 目标 | 数据 | 类型 |
|------|------|------|
| UI previewStore | RenamePreviewItem[] | 数组 |

---

## 五、人工确认数据流

### 5.1 输入

| 来源 | 数据 | 类型 |
|------|------|------|
| UI | ManualReviewItem[] | 数组（用户编辑后） |

### 5.2 处理链

```
1. UI 展示低置信度项
   │  用户编辑: editedMediaType, editedTitle, editedYear, editedSeason, editedEpisode
   │  用户标记: confirmed / skipped
   ↓
2. UI 调用 invoke('apply_manual_review', { reviewItems })
   │  Rust 端更新 ParsedMediaInfo 对应字段
   │  置信度更新为 100 (已确认)
   ↓
3. 重新生成预览
   │  调用 rename::preview_generator::generate 更新后的数据
```

### 5.3 输出

| 目标 | 数据 | 类型 |
|------|------|------|
| UI previewStore | 更新后的 RenamePreviewItem[] | 数组 |

---

## 六、执行数据流

### 6.1 输入

| 来源 | 数据 | 类型 |
|------|------|------|
| UI invoke | RenamePreviewItem[] (已确认) | 数组 |

### 6.2 处理链

```
1. rename::safety_checker::check_all(previewItems) → SafetyReport
   │  10 项安全检查:
   │  1. 源文件存在性
   │  2. 目标路径冲突
   │  3. 路径长度
   │  4. 非法字符
   │  5. 权限检查
   │  6. 低置信度阻断
   │  7. 冲突阻断
   │  8. 重复目标
   │  9. 文件占用检测
   │  10. 路径可达性
   ↓
2. 创建 RenameTask (持久化)
   │  status = Executing
   │  totalFiles = previewItems.length
   ↓
3. rename::executor::execute(previewItem) → RenameResult
   │  对每个文件:
   │  a. 检查 source 存在
   │  b. 检查 target 不存在
   │  c. fs::rename(source, target)
   │  d. 记录 RenameResult { beforePath, afterPath, status, error }
   │  e. 立即持久化到 DB
   │  f. 通过 event 通道推送进度
   ↓
4. 更新 RenameTask
   │  status = Completed / Failed
   │  completedAt = now
   │  renamedCount, skippedCount, failedCount
```

### 6.3 输出

| 目标 | 数据 | 类型 |
|------|------|------|
| DB | RenameTask + RenameResult[] | 持久化 |
| audit/ | AuditLogEntry[] | 日志 |
| UI event | RenameProgress | 事件 |
| UI executionStore | RenameResult[] | 数组 |

---

## 七、回滚数据流

### 7.1 输入

| 来源 | 数据 | 类型 |
|------|------|------|
| UI invoke | taskId | String |

### 7.2 处理链

```
1. rollback::state_checker::check(taskId) → TaskRollbackState
   │  从 DB 读取 RenameTask + RenameResult[]
   │  检查:
   │  - afterPath 是否存在
   │  - beforePath 是否可用
   │  - 是否已被其他文件占用
   ↓
2. 创建 RollbackRecord (持久化)
   │  status = InProgress
   ↓
3. rollback::rollback_executor::execute(rollbackRecord) → RollbackResult[]
   │  对每个成功的 RenameResult:
   │  a. 检查 afterPath 存在
   │  b. 检查 beforePath 可用
   │  c. fs::rename(afterPath, beforePath)
   │  d. 记录结果
   │  e. 冲突时暂停并报告
   ↓
4. 更新 RollbackRecord
   │  status = Completed / PartialFailed / Failed
   │  更新 RenameTask.status = RolledBack
```

### 7.3 输出

| 目标 | 数据 | 类型 |
|------|------|------|
| DB | RollbackRecord | 持久化 |
| audit/ | AuditLogEntry[] | 日志 |
| UI rollbackStore | RollbackResult[] | 数组 |

---

## 八、审计数据流

### 8.1 触发点

| 事件 | 记录内容 |
|------|----------|
| 任务创建 | action: "task_created", taskId |
| 扫描完成 | action: "scan_completed", scanReport |
| 执行开始 | action: "rename_started", taskId, totalFiles |
| 单文件改名 | action: "file_renamed", beforePath, afterPath, status |
| 执行完成 | action: "rename_completed", taskId, stats |
| 回滚开始 | action: "rollback_started", taskId |
| 单文件回滚 | action: "file_rolled_back", afterPath, beforePath, status |
| 回滚完成 | action: "rollback_completed", rollbackId, stats |

### 8.2 持久化

```
AuditLogEntry → SQLite (audit_logs 表)
             → JSONL 文件 (可选导出)
```

---

## 九、配置数据流

### 9.1 模板管理

```
UI settings 页 → invoke('update_template', { mediaType, template })
    → config::template_manager::set_template(mediaType, template)
    → 持久化到 config.toml
```

### 9.2 阈值管理

```
UI settings 页 → invoke('update_threshold', { value })
    → config::threshold::set_threshold(value)
    → 持久化到 config.toml
```

---

## 十、事件通道

| 事件名 | 方向 | 数据 | 用途 |
|--------|------|------|------|
| scan-progress | Rust→UI | { scanned, total, currentFile } | 扫描进度条 |
| rename-progress | Rust→UI | { completed, total, currentFile, status } | 改名进度条 |
| rollback-progress | Rust→UI | { completed, total, currentFile, status } | 回滚进度条 |

---

*下一阶段：dependency-rules.md*
