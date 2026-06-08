# 模块家谱

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、顶层功能族

| 功能族 | 目录 | 职责 | 允许业务逻辑 |
|--------|------|------|:------------:|
| 扫描 | scan/ | 目录遍历、文件识别 | 是 |
| 解析 | parse/ | 文件名解析、分类识别 | 是 |
| 重命名 | rename/ | 模板渲染、冲突检测、执行 | 是 |
| 回滚 | rollback/ | 任务回滚、状态检测 | 是 |
| 审计 | audit/ | 日志记录、导出 | 是 |
| 配置 | config/ | 模板管理、阈值设置 | 是 |
| 共享 | shared/ | 路径工具、结果类型 | 否（仅工具） |
| 应用壳 | app/ | 根组件、布局、路由 | 否 |
| 页面 | pages/ | 页面组合 | 否 |
| 组件 | components/ | 展示和交互 | 否 |
| 流程 | flows/ | UI 流程编排 | 否 |
| 状态 | state/ | UI 状态管理 | 否 |
| 设计系统 | design-system/ | 视觉基础 | 否 |

---

## 二、子功能族

### 扫描功能族 (scan/)

| 子族 | 文件 | 职责 | 依赖 |
|------|------|------|------|
| 遍历器 | scanner.rs | 递归遍历目录 | shared/path_utils |
| 检测器 | file_detector.rs | 检测视频文件类型 | 无 |
| 进度 | progress.rs | 扫描进度管理 | 无 |

### 解析功能族 (parse/)

| 子族 | 文件 | 职责 | 依赖 | 测试数 |
|------|------|------|------|:------:|
| 核心类型 | movie_parser.rs | MediaType / ParsedMediaInfo / SpecialType 定义 + 电影解析 | shared/result_types | 6 |
| 剧集解析 | series_parser.rs | 解析剧集文件名（S01E01 格式） | movie_parser (类型) | 7 |
| 动漫解析 | anime_parser.rs | 解析动漫文件名（三种模式） | movie_parser (类型) | 8 |
| 特别篇解析 | special_parser.rs | 解析 SP/OVA/NCOP/NCED/Extras | movie_parser (类型) | 9 |
| 置信度 | confidence.rs | RuleMatchEvidence 证据链评分 | movie_parser (类型) | 8 |
| 分类器 | classifier.rs | classify 快速分类 + classify_and_parse 统一入口 | 各解析器 | 15 |

**架构说明**：
- movie_parser.rs 承载 ParsedMediaInfo、MediaType、SpecialType 核心类型定义（最小职责原则的例外：类型定义跟首个消费者同文件）
- classifier.rs 是统一入口：classify() 快速判断类型，classify_and_parse() 返回完整解析结果
- 分类优先级：extras > ncop/nced > ova > sp > anime > series > movie > unknown
- 各解析器独立，互不调用；classifier 按优先级依次尝试
- 动漫三种模式：[Group] Title - EP、Title - EP、Title.EP（排除 S01E01 格式）
- 特别篇三种模式：S00Exx > Extras 关键字 > SP/OVA/NCOP/NCED 关键字

### 重命名功能族 (rename/)

| 子族 | 文件 | 职责 | 依赖 | 测试数 |
|------|------|------|------|:------:|
| 模板 | template.rs | 模板渲染 + RenamePreviewItem/RenameConflict/MetadataSource 领域对象 | parse/confidence, shared/result_types | 12 |
| 冲突编排 | conflict_detector.rs | 冲突检测编排器，委托 5 个 checker | 各 checker 子模块 | 8 |
| 路径存在检查 | path_exists_checker.rs | 检查目标路径是否已存在 | shared/path_utils | 1 |
| 路径长度检查 | path_length_checker.rs | 检查路径是否过长 | shared/path_utils | 1 |
| 非法字符检查 | invalid_chars_checker.rs | 检查路径非法字符 | shared/path_utils | 1 |
| 重复目标检查 | duplicate_target_checker.rs | 检查多源指向同一目标 | 无 | 1 |
| 大小写冲突检查 | case_conflict_checker.rs | 检查 Windows 大小写冲突 | 无 | 1 |
| 安全检查 | safety_checker.rs | 置信度/人工确认/冲突/非法字符/路径长度检查 | conflict_detector | 9 |
| 预览 | preview_generator.rs | 从 ParsedMediaInfo 生成预览项 + 冲突标记 | template + conflict_detector + confidence | 8 |
| 执行 | execution/ | DryRun/Confirmed 执行模式 + 安全检查 + 审计记录 | safety_checker, conflict_detector, audit/db | 14 |

**架构说明**：
- template.rs 承载 RenamePreviewItem、RenameConflict、MetadataSource 领域对象定义
- render() 支持 20+ 变量替换，自动清理空括号和多余空格
- get_default_template() 按媒体类型返回默认模板
- conflict_detector.rs 为纯编排器，委托 5 个 checker 子文件各负一责
- 5 个 checker：path_exists / path_length / invalid_chars / duplicate_target / case_conflict
- safety_checker.rs 输出 SafetyReport (can_execute, dry_run, checks, blocking_reasons)
- preview_generator.rs 串联 template → confidence → conflict_detector 完整预览链
- 本轮为 Safety Core Round 3，不执行真实文件改名

#### 执行功能族 (rename/execution/)

| 子族 | 文件 | 职责 | 依赖 | 测试数 |
|------|------|------|------|:------:|
| 模式 | execution_mode.rs | ExecutionMode 枚举 (DryRun/Confirmed) | 无 | 3 |
| 单文件执行 | single_rename.rs | 单文件重命名执行 | shared/result_types | 0 |
| 执行摘要 | execution_summary.rs | ExecutionSummary 结构体 + summarize() | 无 | 3 |
| 安全门 | safety_gate.rs | 执行前安全门检查 | safety_checker | 1 |
| 冲突过滤 | conflict_filter.rs | 过滤阻塞冲突项 | 无 | 3 |
| 跳过过滤 | skip_filter.rs | 过滤应跳过项 | 无 | 2 |
| 结果记录 | result_recorder.rs | 记录阻塞冲突失败结果 | audit/db | 3 |
| 执行核心 | executor_core.rs | execute() 主编排器，委托各子模块 | 所有子模块 | 8 |
| 薄入口 | mod.rs | re-export hub（无业务逻辑） | 子模块 | 0 |

**架构说明**：
- execution/ 采用整树家谱模式，8 个文件各负一责
- execution_mode.rs 定义 ExecutionMode 枚举（DryRun 默认 / Confirmed 需安全检查）
- single_rename.rs 只负责单文件改名操作（fs::rename）
- execution_summary.rs 负责汇总执行结果
- safety_gate.rs 只负责执行前安全门检查
- conflict_filter.rs 只负责过滤阻塞/非阻塞冲突项
- skip_filter.rs 只负责过滤 should_skip 项
- result_recorder.rs 只负责记录阻塞冲突失败结果
- executor_core.rs 为纯编排器：check_safety_gate → filter_actionable → filter_conflicts → execute_loop
- mod.rs 为 re-export hub，下游模块无需修改 import

### 回滚功能族 (rollback/)

| 子族 | 文件 | 职责 | 依赖 | 测试数 |
|------|------|------|------|:------:|
| 执行 | executor/ | 执行回滚 + 审计记录 | audit/logger, audit/db | 12 |
| 状态检查 | state_checker.rs | 回滚可行性检测（afterPath 存在 + beforePath 不被占用） | audit/db | 7 |

#### 执行功能族 (rollback/executor/)

| 子族 | 文件 | 职责 | 依赖 | 测试数 |
|------|------|------|------|:------:|
| 回滚入口 | rollback_entry.rs | RollbackEntry/RollbackStatus/RollbackSummary + summarize_rollback() | 无 | 4 |
| 单文件回滚 | rollback_single.rs | 单文件回滚操作（fs::rename 反向） | rollback_entry | 2 |
| 回滚检查 | rollback_checker.rs | 检查任务是否可回滚 | audit/db | 1 |
| 回滚审计 | rollback_audit.rs | 更新任务状态 + 写审计日志 | audit/db, audit/logger | 1 |
| 回滚摘要 | rollback_summary.rs | 汇总回滚结果 | rollback_entry | 0 |
| 回滚核心 | rollback_core.rs | rollback_task() 主编排器，委托各子模块 | 所有子模块 | 8 |
| 薄入口 | mod.rs | re-export hub（无业务逻辑） | 子模块 | 0 |

**架构说明**：
- executor/ 采用整树家谱模式，6 个文件各负一责
- rollback_entry.rs 定义 RollbackEntry、RollbackStatus (Success/Failed/Blocked)、RollbackSummary
- rollback_single.rs 只负责单文件回滚操作
- rollback_checker.rs 只负责检查任务是否可回滚（状态 + 路径）
- rollback_audit.rs 只负责更新任务状态 + 写审计日志
- rollback_summary.rs 只负责汇总回滚结果
- rollback_core.rs 为纯编排器：check_task_rollbackable → rollback_single → update_status → log_audit
- mod.rs 为 re-export hub，下游模块无需修改 import

### 审计功能族 (audit/)

| 子族 | 文件 | 职责 | 依赖 | 测试数 |
|------|------|------|------|:------:|
| 脱敏器 | redaction.rs | 敏感字段脱敏（TMDb Key/API Key/Token/Secret/URL/JSON） | once_cell, regex | 10 |
| 记录器 | logger.rs | 统一写入审计事件，调用 redaction | db, redaction | 6 |
| 导出器 | exporter.rs | JSONL 导出，导出前 redaction | db, redaction | 5 |

#### 数据库功能族 (audit/db/)

| 子族 | 文件 | 职责 | 依赖 | 测试数 |
|------|------|------|------|:------:|
| 连接 | connection.rs | SQLite 连接创建（内存/文件） | rusqlite | 2 |
| Schema | schema.rs | 表初始化（CREATE TABLE IF NOT EXISTS） | rusqlite | 3 |
| 任务状态 | task_status.rs | TaskStatus 枚举 + Display + FromStr | serde | 4 |
| 任务仓库 | task_repository.rs | rename_tasks 表 CRUD | rusqlite, chrono, uuid, task_status | 5 |
| 结果仓库 | result_repository.rs | rename_results 表 CRUD | rusqlite, chrono, uuid, task_status | 4 |
| 日志仓库 | log_repository.rs | audit_log 表 CRUD | rusqlite, chrono, uuid | 5 |
| 薄入口 | mod.rs | re-export hub（无业务逻辑） | 子模块 | 0 |

**架构说明**：
- audit/db/ 采用整树家谱模式，6 个文件各负一责
- task_status.rs 定义 TaskStatus 枚举（Previewing/Pending/Executing/Completed/Failed/RolledBack）
- connection.rs 只负责连接创建，schema.rs 只负责建表
- 三个仓库各自管理一张表的 CRUD，互不依赖
- mod.rs 为 re-export hub，保持向后兼容（`use audit::db::*` 无需改动）
- redaction.rs 使用 6 个正则表达式覆盖 TMDb Key、通用 API Key、Token、Secret、URL Query、JSON 字段
- logger.rs 提供 log_event / log_failure / log_preview / log_task_created / log_execution_plan
- exporter.rs 支持 export_all / export_by_task，每行一个脱敏后的 JSON 对象
- audit/mod.rs 为薄入口，只导出 4 个子模块（db, redaction, logger, exporter）

### 配置功能族 (config/)

| 子族 | 文件 | 职责 | 依赖 |
|------|------|------|------|
| 模板管理 | template_manager.rs | 管理命名模板 | 无 |
| 阈值 | threshold.rs | 置信度阈值 | 无 |
| 安全存储 | secret/api_key_store.rs | API Key 安全存储 | 无 |
| 数据脱敏 | secret/redaction.rs | 数据脱敏处理 | 无 |
| 用户设置 | user_settings/metadata_settings.rs | 元数据设置管理 | 无 |

### 元数据功能族 (metadata/)

| 子族 | 文件 | 职责 | 依赖 |
|------|------|------|------|
| Provider 接口 | provider/metadata_provider.rs | 元数据 Provider 接口 | 无 |
| 查询接口 | provider/metadata_query.rs | 元数据查询接口 | metadata_provider |
| 匹配接口 | provider/metadata_match.rs | 元数据匹配接口 | metadata_provider |
| TMDb 客户端 | tmdb/tmdb_client.rs | TMDb API 客户端 | config/secret |
| TMDb 配置 | tmdb/tmdb_config.rs | TMDb 配置管理 | config/secret |
| TMDb 错误 | tmdb/tmdb_error.rs | TMDb 错误处理 | 无 |
| TMDb 数据映射 | tmdb/tmdb_mapper.rs | TMDb 数据映射 | metadata_provider |
| Provider 测试 | tests/provider_contract_tests.rs | Provider 契约测试 | metadata_provider |
| TMDb 配置测试 | tests/tmdb_config_tests.rs | TMDb 配置测试 | tmdb_config |

---

## 三、依赖方向

```
UI (pages/flows/components/state)
    ↓ Tauri IPC
Application (scan/parse/rename/rollback)
    ↓
Domain (models/rules/validators)
    ↓
Infrastructure (fs/db/config/logger)
    ↓
shared/ (path_utils, result_types)
```

**禁止依赖**：
- Infrastructure 不得依赖 Application
- Domain 不得依赖 UI
- shared 不得依赖任何功能族

---

## 四、薄入口文件清单

| 文件 | 行数限制 | 内容 |
|------|:--------:|------|
| scan/mod.rs | ≤30 | pub use scanner, file_detector, progress |
| parse/mod.rs | ≤30 | pub use 各解析器, confidence, classifier |
| rename/mod.rs | ≤30 | pub use template, conflict_detector, execution, etc. |
| rollback/mod.rs | ≤30 | pub use executor, state_checker |
| audit/mod.rs | ≤30 | pub use db, redaction, logger, exporter |
| config/mod.rs | ≤30 | pub use template_manager, threshold, secret, user_settings |
| config/secret/mod.rs | ≤30 | pub use api_key_store, redaction |
| config/user_settings/mod.rs | ≤30 | pub use metadata_settings |
| metadata/mod.rs | ≤30 | pub use provider, tmdb, tests |
| metadata/provider/mod.rs | ≤30 | pub use metadata_provider, metadata_query, metadata_match |
| metadata/tmdb/mod.rs | ≤30 | pub use tmdb_client, tmdb_config, tmdb_error, tmdb_mapper |
| metadata/tests/mod.rs | ≤30 | pub use provider_contract_tests, tmdb_config_tests |
| shared/mod.rs | ≤30 | pub use path_utils, result_types |
| components/index.ts | ≤30 | export 各组件 |
| state/*/index.ts | ≤30 | export store |

---

## 五、禁止承载业务逻辑的目录

| 目录 | 原因 |
|------|------|
| app/ | 仅组合，不承载业务 |
| pages/ | 仅页面组合 |
| components/ | 仅展示和交互 |
| design-system/ | 仅视觉基础 |
| shared/ | 仅工具函数 |

---

## 六、测试镜像策略

| 源码 | 测试 |
|------|------|
| src-tauri/src/scan/ | src-tauri/tests/scan_tests.rs |
| src-tauri/src/parse/ | src-tauri/tests/parse_tests.rs |
| src-tauri/src/rename/conflict_detector.rs + checkers | src-tauri/tests/rename/conflict_detection/ (6 个测试文件) |
| src-tauri/src/rename/execution/ | src-tauri/tests/rename/execution/ (6 个测试文件) |
| src-tauri/src/rollback/executor/ | src-tauri/tests/rollback/executor/ (4 个测试文件) |
| src/components/ | tests/components/ |
| src/pages/ | tests/pages/ |
| src/flows/ | tests/flows/ |

**测试入口文件**：
- `tests/rename_execution.rs` — include! 6 个 execution 测试文件（每个 mod {} 隔离）
- `tests/rename_conflict_detection.rs` — include! 6 个 conflict_detection 测试文件
- `tests/rollback_executor.rs` — include! 4 个 rollback executor 测试文件

---

## 七、UI 组件镜像产品流程

| 流程 | 组件 | 页面 |
|------|------|------|
| 扫描 | ScanFlow | ScanPage |
| 预览 | RenamePreviewFlow | PreviewPage |
| 人工确认 | ManualReviewFlow | ManualReviewPage |
| 执行 | ConfirmationDialog | ExecutionConfirmPage |
| 回滚 | RollbackFlow | TaskHistoryPage |

---

*下一阶段：data-flow.md*
