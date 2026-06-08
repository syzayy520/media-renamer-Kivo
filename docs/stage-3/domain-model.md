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

### 13. RenameConflict

**含义**：重命名冲突（带路径和阻塞标记）

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| conflict_type | ConflictType | 否 | 冲突类型 |
| source_path | String | 否 | 源路径 |
| target_path | String | 否 | 目标路径 |
| message | String | 否 | 冲突描述 |
| blocking | bool | 否 | 是否阻塞执行 |

**所属功能族**：rename/conflict_detector.rs

---

### 14. MetadataSource

**含义**：元数据来源

| 值 | 说明 |
|------|------|
| LocalRule | 本地规则解析 |
| Tmdb | TMDb API |
| Manual | 手动输入 |

**所属功能族**：rename/template.rs

---

### 15. SafetyReport

**含义**：安全检查报告

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| can_execute | bool | 否 | 是否可以执行 |
| dry_run | bool | 否 | 是否为预演模式（默认 true） |
| checks | Vec\<SafetyCheck\> | 否 | 检查结果列表 |
| blocking_reasons | Vec\<String\> | 否 | 阻塞原因列表 |

**所属功能族**：rename/safety_checker.rs

---

### 16. SafetyCheck

**含义**：单项安全检查结果

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| name | String | 否 | 检查名称 |
| passed | bool | 否 | 是否通过 |
| message | String | 否 | 消息 |

**检查项**：confidence, manual_review, conflicts, invalid_chars, path_length, skipped

**所属功能族**：rename/safety_checker.rs

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

### 17. ExecutionMode

**含义**：执行模式

| 值 | 说明 |
|------|------|
| DryRun | 预演模式（默认），不修改文件，返回模拟结果 |
| Confirmed | 确认模式，必须通过安全检查，执行真实文件改名 |

**所属功能族**：rename/execution/execution_mode.rs

---

### 18. RollbackReport

**含义**：回滚可行性报告

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| can_rollback | bool | 否 | 是否可以回滚 |
| rollbackable_results | Vec\<RenameResult\> | 否 | 可回滚的结果列表 |
| blocked_results | Vec\<BlockedRollbackEntry\> | 否 | 被阻断的结果列表 |
| blocking_reasons | Vec\<String\> | 否 | 阻断原因列表 |

**所属功能族**：rollback/state_checker.rs

---

### 19. RollbackEntry

**含义**：单个文件回滚结果

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| result_id | String | 否 | 原结果 ID |
| before_path | String | 否 | 回滚前路径（原 afterPath） |
| after_path | String | 否 | 回滚后路径（原 beforePath） |
| status | RollbackStatus | 否 | 回滚状态 |
| error | Option\<String\> | 是 | 错误信息 |

**所属功能族**：rollback/executor/rollback_entry.rs

---

### 20. BlockedRollbackEntry

**含义**：被阻断的回滚项

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| result_id | String | 否 | 原结果 ID |
| reason | String | 否 | 阻断原因 |

**所属功能族**：rollback/state_checker.rs

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

### 11. RuleMatchEvidence

**含义**：置信度评分的单条证据

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| rule_name | String | 否 | 规则名称（如 "movie_year", "anime_group"） |
| matched_pattern | String | 否 | 匹配到的模式描述（如 "year=1999"） |
| confidence_delta | i16 | 否 | 置信度增量（正数加分，负数扣分） |

**所属功能族**：parse/confidence.rs

---

### 12. ConfidenceResult

**含义**：置信度评分的完整结果

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| score | u8 | 否 | 最终置信度 (0-100) |
| evidences | Vec\<RuleMatchEvidence\> | 否 | 匹配证据列表 |
| needs_review | bool | 否 | 是否需要人工确认（score < 70） |

**阈值**：DEFAULT_CONFIDENCE_THRESHOLD = 70
**评分规则**：
- 电影：有年份 +10，无年份 -15，有分辨率 +5，有来源 +3
- 剧集：有季集号 +10，有集标题 +5，有分辨率 +3
- 动漫：有 Group +10，无 Group -10，有集号 +5，有分辨率 +5
- 特别篇：S00 格式 +10，有 special_type +5
- 通用：标题为空 -30，标题短于 3 字符 -20
**所属功能族**：parse/confidence.rs

---

### 13. AuditLogEntry

**含义**：审计日志条目

| 字段 | 类型 | 可空 | 说明 |
|------|------|:----:|------|
| id | String | 否 | 日志 ID (UUID) |
| task_id | Option\<String\> | 是 | 关联任务 ID |
| event_type | String | 否 | 事件类型（task_created/preview/execute/failure/config 等） |
| message | String | 否 | 事件消息（已脱敏） |
| created_at | DateTime\<Utc\> | 否 | 创建时间 |

**持久化**：是（SQLite audit_log 表）
**所属功能族**：audit/db/log_repository.rs, audit/logger.rs

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
Previewing | Pending | Executing | Completed | Failed | RolledBack
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
Success | Failed | Blocked
```

**说明**：
- Success: 回滚成功
- Failed: 回滚失败（afterPath 不存在、beforePath 被占用等）
- Blocked: 被阻断（任务状态不允许回滚、已回滚等）

---

*下一阶段：interfaces-and-contracts.md*
