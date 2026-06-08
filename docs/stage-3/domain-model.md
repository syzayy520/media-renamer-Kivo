# 领域模型

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 核心领域对象

### 1. MediaItem

**含义**：扫描发现的媒体文件

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 唯一标识 |
| filePath | String | 否 | 文件完整路径 |
| fileName | String | 否 | 文件名（含扩展名） |
| extension | String | 否 | 扩展名 |
| fileSize | u64 | 否 | 文件大小（字节） |
| isVideo | bool | 否 | 是否视频文件 |
| isCompanion | bool | 否 | 是否伴随文件 |

**生命周期**：扫描 → 解析 → 预览 → 执行/跳过
**所属功能族**：scan/

---

### 2. ParsedMediaInfo

**含义**：解析后的媒体信息

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| mediaItem | MediaItem | 否 | 原始文件 |
| mediaType | MediaType | 否 | 电影/剧集/动漫/特别篇 |
| title | String | 否 | 标题 |
| year | Option<u16> | 是 | 年份 |
| season | Option<u16> | 是 | 季号 |
| episode | Option<u16> | 是 | 集号 |
| episodeEnd | Option<u16> | 是 | 多集结束号 |
| episodeTitle | Option<String> | 是 | 集标题 |
| resolution | Option<String> | 是 | 分辨率 |
| source | Option<String> | 是 | 来源 |
| videoCodec | Option<String> | 是 | 视频编码 |
| audioCodec | Option<String> | 是 | 音频编码 |
| group | Option<String> | 是 | 编码组 |
| confidence | u8 | 否 | 置信度 0-100 |
| ruleSources | Vec<String> | 否 | 规则命中来源 |
| specialType | Option<SpecialType> | 是 | 特别篇类型 |
| extraType | Option<String> | 是 | Extras 类型 |
| extraNumber | Option<u16> | 是 | Extras 编号 |

**生命周期**：解析 → 预览 → 人工确认（可选）→ 执行
**所属功能族**：parse/

---

### 3. RenameRule

**含义**：重命名规则/模板

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 规则 ID |
| name | String | 否 | 规则名称 |
| mediaType | MediaType | 否 | 适用媒体类型 |
| template | String | 否 | 命名模板 |
| isDefault | bool | 否 | 是否默认规则 |

**生命周期**：配置 → 使用 → 更新
**所属功能族**：config/

---

### 4. RenamePreviewItem

**含义**：重命名预览项

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 预览 ID |
| parsedInfo | ParsedMediaInfo | 否 | 解析信息 |
| originalPath | String | 否 | 原始路径 |
| newName | String | 否 | 新文件名 |
| targetPath | String | 否 | 目标路径 |
| conflicts | Vec<Conflict> | 否 | 冲突列表 |
| needsReview | bool | 否 | 是否需要人工确认 |
| skipped | bool | 否 | 是否跳过 |
| riskLevel | RiskLevel | 否 | 风险等级 |

**生命周期**：预览 → 人工确认 → 执行
**所属功能族**：rename/

---

### 5. RenameTask

**含义**：重命名任务

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 任务 ID |
| scanPath | String | 否 | 扫描目录 |
| createdAt | DateTime | 否 | 创建时间 |
| completedAt | Option<DateTime> | 是 | 完成时间 |
| status | TaskStatus | 否 | 任务状态 |
| totalFiles | u32 | 否 | 总文件数 |
| renamedCount | u32 | 否 | 改名数 |
| skippedCount | u32 | 否 | 跳过数 |
| failedCount | u32 | 否 | 失败数 |
| results | Vec<RenameResult> | 否 | 改名结果 |

**生命周期**：创建 → 预览 → 执行 → 完成/回滚
**持久化**：是
**所属功能族**：rename/ + audit/

---

### 6. RenameResult

**含义**：单个文件改名结果

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 结果 ID |
| taskId | String | 否 | 所属任务 |
| beforePath | String | 否 | 改名前路径 |
| afterPath | String | 否 | 改名后路径 |
| status | RenameStatus | 否 | 成功/失败/跳过 |
| error | Option<String> | 是 | 错误信息 |
| ruleSource | String | 否 | 规则命中来源 |
| executedAt | DateTime | 否 | 执行时间 |

**生命周期**：执行 → 记录
**持久化**：是
**所属功能族**：rename/ + audit/

---

### 7. Conflict

**含义**：冲突信息

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| conflictType | ConflictType | 否 | 冲突类型 |
| message | String | 否 | 冲突描述 |
| severity | Severity | 否 | 严重程度 |

**冲突类型**：
- TargetExists: 目标路径已存在
- DuplicateTarget: 多源指向同一目标
- PathTooLong: 路径过长
- InvalidChars: 非法字符
- PermissionDenied: 权限不足
- SourceNotFound: 源文件不存在

---

### 8. RollbackRecord

**含义**：回滚记录

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 回滚 ID |
| taskId | String | 否 | 原任务 ID |
| createdAt | DateTime | 否 | 回滚时间 |
| status | RollbackStatus | 否 | 回滚状态 |
| results | Vec<RollbackResult> | 否 | 回滚结果 |

**持久化**：是
**所属功能族**：rollback/

---

### 9. ScanReport

**含义**：扫描报告

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| scanPath | String | 否 | 扫描目录 |
| totalFiles | u32 | 否 | 总文件数 |
| videoCount | u32 | 否 | 视频文件数 |
| companionCount | u32 | 否 | 伴随文件数 |
| permissionErrors | Vec<String> | 否 | 权限错误列表 |
| scanDuration | Duration | 否 | 扫描耗时 |

---

### 10. ManualReviewItem

**含义**：人工确认项

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| previewItem | RenamePreviewItem | 否 | 预览项 |
| editedMediaType | Option<MediaType> | 是 | 修改后的类型 |
| editedTitle | Option<String> | 是 | 修改后的标题 |
| editedYear | Option<u16> | 是 | 修改后的年份 |
| editedSeason | Option<u16> | 是 | 修改后的季号 |
| editedEpisode | Option<u16> | 是 | 修改后的集号 |
| confirmed | bool | 否 | 是否已确认 |
| skipped | bool | 否 | 是否跳过 |

---

### 11. AuditLogEntry

**含义**：审计日志条目

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 日志 ID |
| timestamp | DateTime | 否 | 时间戳 |
| level | LogLevel | 否 | 日志级别 |
| action | String | 否 | 操作类型 |
| details | serde_json::Value | 否 | 详细信息 |

**持久化**：是
**所属功能族**：audit/

---

## 枚举类型

### MediaType
```
Movie | Series | Anime | Special | OVA | NCOP | NCED | Extras | Unknown
```

### SpecialType
```
Special | SP | OVA | NCOP | NCED | Extra
```

### TaskStatus
```
Created | Scanning | Previewing | Reviewing | Executing | Completed | Failed | RolledBack
```

### RenameStatus
```
Success | Failed | Skipped | Conflict
```

### RiskLevel
```
None | Low | Medium | High | Critical
```

### ConflictType
```
TargetExists | DuplicateTarget | PathTooLong | InvalidChars | PermissionDenied | SourceNotFound
```

### RollbackStatus
```
Pending | InProgress | Completed | PartialFailed | Failed
```

---

*下一阶段：interfaces-and-contracts.md*
