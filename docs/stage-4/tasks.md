# 任务分解

> 阶段 4：任务分解与执行计划
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 分解原则

1. 原子性：每个任务可独立完成和测试
2. 可追溯：每个任务对应需求 ID
3. 依赖清晰：任务间依赖关系明确
4. 粒度适中：单个任务 2-8 小时

---

## 任务依赖图

```
T-INF-001 (项目初始化)
  ├── T-INF-002 (Rust 依赖)
  ├── T-INF-003 (前端依赖)
  ├── T-INF-004 (目录结构)
  │     └── T-INF-005 (设计系统)
  │
  ├── T-RUST-001 (shared/)
  │     ├── T-RUST-002 (config/)
  │     ├── T-RUST-002a (config/secret/)
  │     │     └── T-RUST-002b (config/user_settings/)
  │     ├── T-RUST-003 (scan/)
  │     ├── T-RUST-004 (parse/)
  │     ├── T-RUST-006 (audit/)
  │     │     ├── T-RUST-007 (rollback/)
  │     │     └── T-RUST-009 (数据库)
  │     └── T-RUST-005 (rename/)
  │           └── T-RUST-008 (IPC 命令)
  │                 └── T-RUST-010 (事件通道)
  │                       └── T-RUST-011 (错误处理)
  │                             └── T-RUST-012 (崩溃恢复)
  │
  ├── T-RUST-013 (metadata/)
  │     ├── T-RUST-013a (metadata/provider/)
  │     └── T-RUST-013b (metadata/tmdb/)
  │           └── T-RUST-013c (metadata/tests/)
  │
  └── T-UI-001 (应用壳) + T-UI-002 (状态) + T-UI-003 (组件)
        ├── T-UI-004 (扫描页)
        │     └── T-UI-005 (预览页)
        │           ├── T-UI-006 (人工确认页)
        │           └── T-UI-007 (执行确认页)
        ├── T-UI-008 (历史页)
        ├── T-UI-009 (设置页)
        └── T-UI-010 (Toast)

T-TEST-* 可在对应模块完成后并行执行
T-PKG-* 在所有功能完成后执行
```

---

## A. 基础设施任务

### T-INF-001: 初始化 Tauri v2 + React + TS 项目

| 属性 | 值 |
|------|-----|
| 需求 | ADR-001, ADR-002, ADR-010 |
| 优先级 | P0 |
| 预估 | 2h |
| 依赖 | 无 |

**子任务**：
1. `npm create tauri-app` 初始化
2. 配置 tsconfig.json, vite.config.ts, tailwind.config.js
3. 验证 `npm run dev` 可启动

**验收**：窗口显示成功，Tailwind 生效

---

### T-INF-002: Rust 依赖配置

| 属性 | 值 |
|------|-----|
| 需求 | ADR-006 |
| 优先级 | P0 |
| 预估 | 1h |
| 依赖 | T-INF-001 |

**子任务**：编辑 Cargo.toml 添加 serde, rusqlite, regex, anyhow, thiserror, tracing, uuid, chrono, camino, walkdir, tokio

**验收**：`cargo build` 编译成功

---

### T-INF-003: 前端依赖配置

| 属性 | 值 |
|------|-----|
| 需求 | ADR-002~004 |
| 优先级 | P0 |
| 预估 | 1h |
| 依赖 | T-INF-001 |

**子任务**：安装 react-router-dom, zustand, @tauri-apps/api, lucide-react, @tanstack/react-virtual, vitest, @testing-library/react

**验收**：`npm install` 成功

---

### T-INF-004: 创建整树家谱目录结构

| 属性 | 值 |
|------|-----|
| 需求 | ADR-010, NF-009 |
| 优先级 | P0 |
| 预估 | 2h |
| 依赖 | T-INF-001 |

**子任务**：
1. Rust: src-tauri/src/{scan,parse,rename,rollback,audit,config,shared}/
2. 前端: src/{app,pages,components,flows,state,design-system}/
3. 测试目录: src-tauri/tests/, tests/
4. 示例目录: examples/fixtures/sandbox/
5. 薄入口文件 (mod.rs, index.ts)

**验收**：目录结构符合 module-tree.md，入口文件 ≤30 行

---

### T-INF-005: 设计系统基础

| 属性 | 值 |
|------|-----|
| 需求 | visual-design-direction.md |
| 优先级 | P0 |
| 预估 | 3h |
| 依赖 | T-INF-003, T-INF-004 |

**子任务**：
1. Tailwind 主题配置（色彩、字体、间距）
2. design-system/tokens/ (colors, typography, spacing)
3. design-system/buttons/ (Primary, Secondary, Danger, Ghost)

**验收**：按钮组件可渲染，符合设计规范

---

## B. Rust 核心任务

### T-RUST-001: shared/ 共享模块

| 属性 | 值 |
|------|-----|
| 需求 | F-006-03, F-006-04 |
| 优先级 | P0 |
| 预估 | 3h |
| 依赖 | T-INF-002, T-INF-004 |

**子任务**：
1. shared/path_utils.rs: sanitize_filename, is_path_too_long, has_invalid_chars, normalize_path
2. shared/result_types.rs: AppError, AppResult
3. shared/platform.rs: 平台适配
4. 单元测试

**验收**：路径工具正确处理 Windows 路径，测试覆盖率 >90%

---

### T-RUST-002: config/ 配置模块

| 属性 | 值 |
|------|-----|
| 需求 | F-012, NF-008 |
| 优先级 | P0 |
| 预估 | 3h |
| 依赖 | T-RUST-001 |

**子任务**：
1. template_manager.rs: get/set/get_all 模板
2. threshold.rs: get/set 阈值
3. config_loader.rs: config.toml 读写
4. 单元测试

**验收**：配置正确读写，默认模板符合 requirements.md

---

### T-RUST-002a: config/secret/ 安全存储模块

| 属性 | 值 |
|------|-----|
| 需求 | ADR-SECRET-001, F-METADATA-003 |
| 优先级 | P0 |
| 预估 | 4h |
| 依赖 | T-RUST-001 |

**子任务**：
1. api_key_store.rs: API Key 安全存储（SQLite 加密或系统密钥库）
2. redaction.rs: 数据脱敏处理
3. 单元测试
4. 安全测试（确保 Key 不被泄露）

**验收**：
- API Key 安全存储
- 脱敏显示正确
- 无日志/审计日志泄露
- 无 Git 提交风险

---

### T-RUST-002b: config/user_settings/ 用户设置模块

| 属性 | 值 |
|------|-----|
| 需求 | F-METADATA-001 ~ F-METADATA-006 |
| 优先级 | P0 |
| 预估 | 2h |
| 依赖 | T-RUST-002a |

**子任务**：
1. metadata_settings.rs: TMDb 设置管理
2. 单元测试

**验收**：TMDb 设置正确读写

---

### T-RUST-013: metadata/ 元数据模块

| 属性 | 值 |
|------|-----|
| 需求 | F-METADATA-001 ~ F-METADATA-006, ADR-METADATA-001 |
| 优先级 | P0 |
| 预估 | 8h |
| 依赖 | T-RUST-002a, T-RUST-002b |

**子任务**：
1. provider/metadata_provider.rs: Provider 接口定义
2. provider/metadata_query.rs: 查询接口
3. provider/metadata_match.rs: 匹配接口
4. tmdb/tmdb_client.rs: TMDb 客户端实现
5. tmdb/tmdb_config.rs: TMDb 配置管理
6. tmdb/tmdb_error.rs: 错误处理
7. tmdb/tmdb_mapper.rs: 数据映射
8. tests/provider_contract_tests.rs: Provider 契约测试
9. tests/tmdb_config_tests.rs: TMDb 配置测试
10. 单元测试

**验收**：
- Provider 接口清晰
- TMDb 客户端功能正常
- 配置管理正确
- 错误处理完善
- 测试覆盖完整

---

### T-RUST-003: scan/ 扫描模块

| 属性 | 值 |
|------|-----|
| 需求 | F-001-01 ~ F-001-10 |
| 优先级 | P0 |
| 预估 | 6h |
| 依赖 | T-RUST-001, T-RUST-002 |

**子任务**：
1. file_detector.rs: is_video, is_companion
2. scanner.rs: scan_directory, 递归遍历, 符号链接/循环检测, 权限错误记录, 最大文件数保护
3. progress.rs: ScanProgress
4. 单元测试

**验收**：正确识别 13 种视频扩展名，递归遍历，权限错误不中断，最大文件数限制

---

### T-RUST-004: parse/ 解析模块

| 属性 | 值 |
|------|-----|
| 需求 | F-002, F-003 |
| 优先级 | P0 |
| 预估 | 12h |
| 依赖 | T-RUST-001 |
| 状态 | **已完成** ✅ |

**子任务**：
1. ✅ movie_parser.rs: 电影文件名解析（6 个测试通过）
2. ✅ series_parser.rs: 剧集文件名解析（7 个测试通过）
3. ✅ anime_parser.rs: 动漫文件名解析（8 个测试通过）
4. ✅ special_parser.rs: 特别篇/OVA/NCOP/NCED 解析（9 个测试通过）
5. ✅ confidence.rs: 置信度评分（8 个测试通过）
6. ✅ classifier.rs: 类型分类 + classify_and_parse 统一入口（15 个测试通过）
7. 待后续阶段：rename-fixtures.md 45 个测试用例

**验证结果**（2026-06-08）：
- `cargo test`: 74/74 PASS
- `cargo clippy`: 无 warning
- `cargo fmt --check`: 通过
- parse 模块单元测试：53 个全部通过

**关键实现**：
- classifier 优先级：extras > ncop/nced > ova > sp > anime > series > movie > unknown
- confidence 使用 RuleMatchEvidence 证据链评分
- classify_and_parse 为统一分类+解析入口
- 动漫三种解析模式：[Group] Title - EP / Title - EP / Title.EP
- 特别篇三种模式：S00E / Extras / SP/OVA/NCOP/NCED 关键字

---

### T-RUST-005: rename/ 重命名模块

| 属性 | 值 |
|------|-----|
| 需求 | F-004, F-006, F-007 |
| 优先级 | P0 |
| 预估 | 12h |
| 依赖 | T-RUST-001, T-RUST-002, T-RUST-004 |
| 状态 | **Safety Core Round 1 完成** ✅ (executor 待实现) |

**子任务**：
1. ✅ template.rs: 模板渲染 + RenamePreviewItem/RenameConflict/MetadataSource 领域对象（12 个测试通过）
2. ✅ conflict_detector.rs: 6 种冲突检测 + has_blocking_conflicts（8 个测试通过）
3. ✅ safety_checker.rs: 置信度/人工确认/冲突/非法字符/路径长度检查（9 个测试通过）
4. ✅ preview_generator.rs: 预览生成 + 默认模板 + sanitize（8 个测试通过）
5. 待后续阶段：executor.rs: 批量执行 + 进度推送
6. ✅ 单元测试：37 个新增测试全部通过

**验证结果**（2026-06-08）：
- `cargo test`: 111/111 PASS (74 baseline + 37 rename)
- `cargo clippy --all-targets -- -D warnings`: 通过
- `cargo fmt --check`: 通过

**关键实现**：
- render() 支持 20+ 变量替换，自动清理空括号和多余空格
- 冲突检测：TargetExists/DuplicateTarget/CaseConflict/PathTooLong/InvalidChars/SourceNotFound
- SafetyReport 包含 can_execute, dry_run, checks, blocking_reasons
- preview_generator 串联 template → confidence → conflict_detector 完整预览链
- 本轮为 Safety Core Round 1，不执行真实文件改名

---

### T-RUST-006: audit/ 审计模块

| 属性 | 值 |
|------|-----|
| 需求 | F-008 |
| 优先级 | P0 |
| 预估 | 4h |
| 依赖 | T-RUST-001 |
| 状态 | **Safety Core Round 2 完成** ✅ |

**子任务**：
1. ✅ redaction.rs: 敏感字段脱敏（TMDb Key/API Key/Token/Secret/URL/JSON）（10 个测试通过）
2. ✅ db.rs: SQLite 表初始化 + rename_tasks/rename_results/audit_log CRUD（8 个测试通过）
3. ✅ logger.rs: 统一写入审计事件，调用 redaction（6 个测试通过）
4. ✅ exporter.rs: JSONL 导出，导出前 redaction（5 个测试通过）
5. ✅ mod.rs: 薄入口，只导出 4 个子模块

**验证结果**（2026-06-08）：
- `cargo test`: 140/140 PASS (111 baseline + 29 audit)
- `cargo clippy --all-targets -- -D warnings`: 通过
- `cargo fmt --check`: 通过

**关键实现**：
- db.rs 定义 RenameTask / RenameResult / AuditLogEntry 数据结构 + TaskStatus 枚举
- redaction.rs 使用 6 个正则表达式覆盖 TMDb Key、通用 API Key、Token、Secret、URL Query、JSON 字段
- logger.rs 提供 log_event / log_failure / log_preview / log_task_created / log_execution_plan
- exporter.rs 支持 export_all / export_by_task，每行一个脱敏后的 JSON 对象
- 本轮为 Safety Core Round 2，不执行真实文件改名

---

### T-RUST-007: rollback/ 回滚模块

| 属性 | 值 |
|------|-----|
| 需求 | F-009, F-010 |
| 优先级 | P0 |
| 预估 | 6h |
| 依赖 | T-RUST-001, T-RUST-006 |

**子任务**：
1. state_checker.rs: check_rollback_state
2. rollback_executor.rs: rollback_task, 逐文件回滚, 冲突暂停
3. 单元测试

**验收**：回滚前置检查正确，回滚执行正确，冲突时暂停报告

---

### T-RUST-008: Tauri IPC 命令层

| 属性 | 值 |
|------|-----|
| 需求 | interfaces-and-contracts.md |
| 优先级 | P0 |
| 预估 | 4h |
| 依赖 | T-RUST-003~007 |

**子任务**：实现 15+ 个 #[tauri::command]，注册到 main.rs

**验收**：所有命令可从前端调用，错误正确转换

---

### T-RUST-009: 数据库初始化

| 属性 | 值 |
|------|-----|
| 需求 | F-008-07, F-010-01 |
| 优先级 | P0 |
| 预估 | 2h |
| 依赖 | T-RUST-006 |

**子任务**：创建 4 张表 (rename_tasks, rename_results, rollback_records, audit_logs)

**验收**：应用启动时自动创建表，结构符合 interfaces-and-contracts.md

---

### T-RUST-010: 事件通道

| 属性 | 值 |
|------|-----|
| 需求 | interfaces-and-contracts.md 事件契约 |
| 优先级 | P0 |
| 预估 | 2h |
| 依赖 | T-RUST-008 |

**子任务**：实现 scan-progress, rename-progress, rollback-progress 事件推送

**验收**：前端可实时监听事件

---

### T-RUST-011: 错误处理集成

| 属性 | 值 |
|------|-----|
| 需求 | error-handling-strategy.md |
| 优先级 | P0 |
| 预估 | 2h |
| 依赖 | T-RUST-001~010 |

**子任务**：统一 AppError, 批量操作错误处理, panic 恢复

**验收**：无 unwrap/expect 在生产代码，错误正确传播

---

### T-RUST-012: 崩溃恢复

| 属性 | 值 |
|------|-----|
| 需求 | F-010 |
| 优先级 | P1 |
| 预估 | 3h |
| 依赖 | T-RUST-007, T-RUST-009 |

**子任务**：任务状态持久化时机，启动时恢复检查，未完成任务处理

**验收**：崩溃后重启可检测未完成任务，已执行改名可回滚

---

## C. UI 前端任务

### T-UI-001: 应用壳和路由

| 需求 | ui-architecture.md | 预估 | 3h | 依赖 | T-INF-004, T-INF-005 |

**子任务**：App.tsx, layout.tsx (侧边栏+内容), router.tsx, 导航高亮

**验收**：路由切换正常，侧边栏导航工作

---

### T-UI-002: Zustand Store

| 需求 | ui-state-model.md | 预估 | 4h | 依赖 | T-INF-003 |

**子任务**：scanStore, previewStore, executionStore, rollbackStore, configStore, 状态串联

**验收**：状态管理正确，筛选/排序功能正常

---

### T-UI-003: 设计系统组件

| 需求 | visual-design-direction.md | 预估 | 4h | 依赖 | T-INF-005 |

**子任务**：FileTable (虚拟滚动), RiskBadge, ConfidenceIndicator, PathDiff, ConfirmationDialog

**验收**：虚拟滚动支持 10,000 行，路径对比清晰

---

### T-UI-004: 扫描页面

| 需求 | F-001 | 预估 | 4h | 依赖 | T-UI-001~003 |

**子任务**：ScanPage (目录选择、进度、统计), ScanFlow, 空状态提示

**验收**：扫描进度实时更新，统计信息正确

---

### T-UI-005: 预览页面

| 需求 | F-004 | 预估 | 6h | 依赖 | T-UI-004 |

**子任务**：PreviewPage (文件表格、筛选器、排序、批量操作), PreviewFlow

**验收**：表格展示正确，筛选/排序正常，before/after 对比清晰

---

### T-UI-006: 人工确认页面

| 需求 | F-005, F-011 | 预估 | 4h | 依赖 | T-UI-005 |

**子任务**：ManualReviewPage (编辑表单、跳过、确认), ManualReviewFlow

**验收**：编辑后更新预览，跳过功能正常

---

### T-UI-007: 执行确认页面

| 需求 | F-007 | 预估 | 4h | 依赖 | T-UI-005, T-UI-006 |

**子任务**：ExecutionConfirmPage (统计、风险提示、确认弹窗、进度、结果)

**验收**：确认弹窗工作，执行进度实时更新

---

### T-UI-008: 任务历史页面

| 需求 | F-008, F-009 | 预估 | 4h | 依赖 | T-UI-002 |

**子任务**：TaskHistoryPage (列表、详情、回滚、导出), RollbackFlow

**验收**：任务列表正确，回滚功能正常

---

### T-UI-009: 设置页面

| 需求 | F-012 | 预估 | 3h | 依赖 | T-UI-002 |

**子任务**：SettingsPage (模板编辑、阈值设置), 配置保存

**验收**：模板编辑正常，阈值设置生效

---

### T-UI-011: TMDb API Key 配置页面

| 需求 | F-METADATA-001 ~ F-METADATA-006 | 预估 | 4h | 依赖 | T-UI-002 |

**子任务**：
1. SettingsPage 中添加 TMDb 配置区域
2. ApiKeyInput 组件（输入、遮罩显示、测试连接）
3. MetadataSourceBadge 组件（来源标签）
4. 状态管理（configStore 或 metadataStore）
5. 未配置时的提示信息

**验收**：
- API Key 输入、保存、清除功能正常
- 遮罩显示正确
- 测试连接功能可用
- 未配置时显示正确提示
- 已配置时显示已配置状态

### T-UI-010: Toast 通知

| 需求 | ui-review-notes.md | 预估 | 2h | 依赖 | T-UI-001 |

**子任务**：Toast 组件、容器、showToast 函数

**验收**：Toast 正确显示，自动消失

---

## D. 测试任务

### T-TEST-001: Rust 单元测试

| 预估 | 8h | 依赖 | T-RUST-001~012 |

**子任务**：shared/config/scan/parse/rename/rollback/audit 全模块测试

**验收**：测试覆盖率 >80%，45 个 fixture 通过

**parse 模块测试进度**（2026-06-08）：
| 文件 | 测试数 | 状态 |
|------|--------|------|
| movie_parser.rs | 6 | ✅ 全通过 |
| series_parser.rs | 7 | ✅ 全通过 |
| anime_parser.rs | 8 | ✅ 全通过 |
| special_parser.rs | 9 | ✅ 全通过 |
| confidence.rs | 8 | ✅ 全通过 |
| classifier.rs | 15 | ✅ 全通过 |
| **parse 合计** | **53** | **✅** |

**rename 模块测试进度**（2026-06-08 Safety Core Round 1）：
| 文件 | 测试数 | 状态 |
|------|--------|------|
| template.rs | 12 | ✅ 全通过 |
| conflict_detector.rs | 8 | ✅ 全通过 |
| safety_checker.rs | 9 | ✅ 全通过 |
| preview_generator.rs | 8 | ✅ 全通过 |
| **rename 合计** | **37** | **✅** |

**audit 模块测试进度**（2026-06-08 Safety Core Round 2）：
| 文件 | 测试数 | 状态 |
|------|--------|------|
| redaction.rs | 10 | ✅ 全通过 |
| db.rs | 8 | ✅ 全通过 |
| logger.rs | 6 | ✅ 全通过 |
| exporter.rs | 5 | ✅ 全通过 |
| **audit 合计** | **29** | **✅** |

**总测试数**：140/140 PASS (111 baseline + 29 audit)

---

### T-TEST-002: 前端单元测试

| 预估 | 6h | 依赖 | T-UI-001~010 |

**子任务**：组件测试、Store 测试、流程测试

**验收**：核心组件/Store/流程测试覆盖

---

### T-TEST-003: 集成测试

| 预估 | 4h | 依赖 | T-RUST-008, T-UI-004~008 |

**子任务**：Tauri IPC 命令测试、端到端流程测试

**验收**：扫描→解析→预览→执行→回滚全流程通过

---

### T-TEST-004: 性能测试

| 预估 | 3h | 依赖 | T-TEST-003 |

**子任务**：10,000 文件扫描、10,000 文件预览、内存占用

**验收**：扫描 <30s，UI 不卡顿，内存 <500MB

---

### T-TEST-005: Sandbox Fixtures

| 预估 | 3h | 依赖 | T-INF-004 |

**子任务**：创建 45 组 fixture 文件，覆盖所有媒体类型

**验收**：fixture 文件可被正确扫描和解析

---

## E. 打包部署任务

### T-PKG-001: 开发环境文档

| 预估 | 2h | 依赖 | T-INF-001 |

**子任务**：README.md，环境要求，开发/构建/测试命令

**验收**：新开发者可按文档搭建环境

---

### T-PKG-002: 生产构建

| 预估 | 2h | 依赖 | T-TEST-001, T-TEST-002 |

**子任务**：Tauri 生产构建配置，MSI + NSIS 打包

**验收**：构建产物可安装运行

---

### T-PKG-003: 交付文档

| 预估 | 2h | 依赖 | T-PKG-002 |

**子任务**：用户手册、已知限制、版本说明

**验收**：文档完整清晰

---

## 任务汇总

| 类别 | 数量 | 总预估 |
|------|------|--------|
| 基础设施 | 5 | 9h |
| Rust 核心 | 15 | 71h |
| UI 前端 | 11 | 42h |
| 测试 | 5 | 24h |
| 打包部署 | 3 | 6h |
| **合计** | **39** | **152h** |

**新增任务说明**：
- T-RUST-002a: config/secret/ 安全存储模块 (+4h)
- T-RUST-002b: config/user_settings/ 用户设置模块 (+2h)
- T-RUST-013: metadata/ 元数据模块 (+8h)
- T-UI-011: TMDb API Key 配置页面 (+4h)

---

*下一阶段：traceability-matrix.md*
