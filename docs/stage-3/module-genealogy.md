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
| 冲突检测 | conflict_detector.rs | 检测目标存在/重复目标/大小写冲突/路径过长/非法字符 | shared/path_utils | 8 |
| 安全检查 | safety_checker.rs | 置信度/人工确认/冲突/非法字符/路径长度检查 | conflict_detector | 9 |
| 预览 | preview_generator.rs | 从 ParsedMediaInfo 生成预览项 + 冲突标记 | template + conflict_detector + confidence | 8 |
| 执行器 | executor.rs | 执行改名 | audit/logger | 待实现 |

**架构说明**：
- template.rs 承载 RenamePreviewItem、RenameConflict、MetadataSource 领域对象定义
- render() 支持 20+ 变量替换，自动清理空括号和多余空格
- get_default_template() 按媒体类型返回默认模板
- conflict_detector.rs 检测 6 种冲突：TargetExists/DuplicateTarget/CaseConflict/PathTooLong/InvalidChars/SourceNotFound
- safety_checker.rs 输出 SafetyReport (can_execute, dry_run, checks, blocking_reasons)
- preview_generator.rs 串联 template → confidence → conflict_detector 完整预览链
- 本轮为 Safety Core Round 1，不执行真实文件改名

### 回滚功能族 (rollback/)

| 子族 | 文件 | 职责 | 依赖 |
|------|------|------|------|
| 执行器 | rollback_executor.rs | 执行回滚 | audit/logger |
| 状态检查 | state_checker.rs | 检测回滚状态 | 无 |

### 审计功能族 (audit/)

| 子族 | 文件 | 职责 | 依赖 |
|------|------|------|------|
| 记录器 | logger.rs | 记录审计日志 | 无 |
| 导出器 | exporter.rs | JSONL 导出 | 无 |

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
| rename/mod.rs | ≤30 | pub use template, conflict_detector, etc. |
| rollback/mod.rs | ≤30 | pub use rollback_executor, state_checker |
| audit/mod.rs | ≤30 | pub use logger, exporter |
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
| src-tauri/src/rename/ | src-tauri/tests/rename_tests.rs |
| src-tauri/src/rollback/ | src-tauri/tests/rollback_tests.rs |
| src/components/ | tests/components/ |
| src/pages/ | tests/pages/ |
| src/flows/ | tests/flows/ |

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
