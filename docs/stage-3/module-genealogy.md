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

| 子族 | 文件 | 职责 | 依赖 |
|------|------|------|------|
| 电影解析 | movie_parser.rs | 解析电影文件名 | shared/result_types |
| 剧集解析 | series_parser.rs | 解析剧集文件名 | shared/result_types |
| 动漫解析 | anime_parser.rs | 解析动漫文件名 | shared/result_types |
| 特别篇解析 | special_parser.rs | 解析特别篇文件名 | shared/result_types |
| 置信度 | confidence.rs | 置信度评分 | 无 |
| 分类器 | classifier.rs | 类型分类 | 各解析器 |

### 重命名功能族 (rename/)

| 子族 | 文件 | 职责 | 依赖 |
|------|------|------|------|
| 模板 | template.rs | 模板渲染 | config/template_manager |
| 冲突检测 | conflict_detector.rs | 检测冲突 | shared/path_utils |
| 安全检查 | safety_checker.rs | 10 项安全检查 | conflict_detector |
| 执行器 | executor.rs | 执行改名 | audit/logger |
| 预览 | preview_generator.rs | 生成预览 | template + conflict_detector |

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
| config/mod.rs | ≤30 | pub use template_manager, threshold |
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
