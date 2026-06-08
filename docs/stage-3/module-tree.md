# 模块树

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 完整模块树

```
media-renamer-Kivo/
├── docs/                              # 文档族
│   ├── stage-0/                       # 阶段 0 文档
│   ├── stage-1/                       # 阶段 1 文档
│   ├── stage-2/                       # 阶段 2 文档
│   ├── stage-3/                       # 阶段 3 文档
│   └── stage-4/                       # 阶段 4 文档
│
├── src-tauri/                         # Rust 核心族
│   ├── src/
│   │   ├── scan/                      # 扫描功能族
│   │   │   ├── scanner.rs             # 目录遍历器
│   │   │   ├── file_detector.rs       # 文件类型检测
│   │   │   ├── progress.rs            # 扫描进度
│   │   │   └── mod.rs                 # 薄入口
│   │   │
│   │   ├── parse/                     # 解析功能族
│   │   │   ├── movie_parser.rs        # 电影解析器
│   │   │   ├── series_parser.rs       # 剧集解析器
│   │   │   ├── anime_parser.rs        # 动漫解析器
│   │   │   ├── special_parser.rs      # 特别篇解析器
│   │   │   ├── confidence.rs          # 置信度评分
│   │   │   ├── classifier.rs          # 类型分类器
│   │   │   └── mod.rs                 # 薄入口
│   │   │
│   │   ├── rename/                    # 重命名功能族
│   │   │   ├── template.rs            # 模板渲染
│   │   │   ├── conflict_detector.rs   # 冲突检测
│   │   │   ├── safety_checker.rs      # 安全检查
│   │   │   ├── executor.rs            # 执行器
│   │   │   ├── preview_generator.rs   # 预览生成
│   │   │   └── mod.rs                 # 薄入口
│   │   │
│   │   ├── rollback/                  # 回滚功能族
│   │   │   ├── rollback_executor.rs   # 回滚执行器
│   │   │   ├── state_checker.rs       # 状态检查
│   │   │   └── mod.rs                 # 薄入口
│   │   │
│   │   ├── audit/                     # 审计功能族
│   │   │   ├── logger.rs             # 日志记录器
│   │   │   ├── exporter.rs           # JSONL 导出器
│   │   │   └── mod.rs                 # 薄入口
│   │   │
│   │   ├── config/                    # 配置功能族
│   │   │   ├── template_manager.rs   # 模板管理
│   │   │   ├── threshold.rs          # 阈值配置
│   │   │   ├── secret/               # 安全存储子族
│   │   │   │   ├── api_key_store.rs  # API Key 存储
│   │   │   │   ├── redaction.rs      # 数据脱敏
│   │   │   │   └── mod.rs            # 薄入口
│   │   │   ├── user_settings/        # 用户设置子族
│   │   │   │   ├── metadata_settings.rs # 元数据设置
│   │   │   │   └── mod.rs            # 薄入口
│   │   │   └── mod.rs                 # 薄入口
│   │   │
│   │   ├── metadata/                 # 元数据功能族
│   │   │   ├── provider/             # Provider 子族
│   │   │   │   ├── metadata_provider.rs # Provider 接口
│   │   │   │   ├── metadata_query.rs # 查询接口
│   │   │   │   ├── metadata_match.rs # 匹配接口
│   │   │   │   └── mod.rs            # 薄入口
│   │   │   ├── tmdb/                 # TMDb 子族
│   │   │   │   ├── tmdb_client.rs    # TMDb 客户端
│   │   │   │   ├── tmdb_config.rs    # TMDb 配置
│   │   │   │   ├── tmdb_error.rs     # TMDb 错误
│   │   │   │   ├── tmdb_mapper.rs    # TMDb 数据映射
│   │   │   │   └── mod.rs            # 薄入口
│   │   │   ├── tests/                # 测试子族
│   │   │   │   ├── provider_contract_tests.rs # Provider 契约测试
│   │   │   │   ├── tmdb_config_tests.rs # TMDb 配置测试
│   │   │   │   └── mod.rs            # 薄入口
│   │   │   └── mod.rs                # 薄入口
│   │   │
│   │   ├── shared/                    # 共享模块
│   │   │   ├── path_utils.rs         # 路径工具
│   │   │   ├── result_types.rs       # 结果类型
│   │   │   └── mod.rs                 # 薄入口
│   │   │
│   │   └── lib.rs                     # 库入口
│   │
│   ├── tests/                         # Rust 测试
│   │   ├── scan_tests.rs
│   │   ├── parse_tests.rs
│   │   ├── rename_tests.rs
│   │   └── rollback_tests.rs
│   │
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                               # 前端族
│   ├── app/                           # 应用壳族
│   │   ├── App.tsx                    # 应用根组件
│   │   ├── layout.tsx                 # 布局组件
│   │   └── router.tsx                 # 路由配置
│   │
│   ├── pages/                         # 页面族
│   │   ├── scan/                      # 扫描页面
│   │   │   ├── ScanPage.tsx
│   │   │   └── index.ts
│   │   ├── preview/                   # 预览页面
│   │   │   ├── PreviewPage.tsx
│   │   │   └── index.ts
│   │   ├── manual-review/             # 人工确认页面
│   │   │   ├── ManualReviewPage.tsx
│   │   │   └── index.ts
│   │   ├── execution-confirm/         # 执行确认页面
│   │   │   ├── ExecutionConfirmPage.tsx
│   │   │   └── index.ts
│   │   ├── task-history/              # 任务历史页面
│   │   │   ├── TaskHistoryPage.tsx
│   │   │   └── index.ts
│   │   └── settings/                  # 设置页面
│   │       ├── SettingsPage.tsx
│   │       └── index.ts
│   │
│   ├── components/                    # 组件族
│   │   ├── file-table/                # 文件表格组件
│   │   │   ├── FileTable.tsx
│   │   │   ├── FileTableRow.tsx
│   │   │   └── index.ts
│   │   ├── risk-badge/                # 风险标签组件
│   │   │   ├── RiskBadge.tsx
│   │   │   └── index.ts
│   │   ├── confidence-indicator/      # 置信度指示器
│   │   │   ├── ConfidenceIndicator.tsx
│   │   │   └── index.ts
│   │   ├── path-diff/                 # 路径对比组件
│   │   │   ├── PathDiff.tsx
│   │   │   └── index.ts
│   │   ├── confirmation-dialog/       # 确认弹窗组件
│   │   │   ├── ConfirmationDialog.tsx
│   │   │   └── index.ts
│   │   ├── api-key-input/             # API Key 输入组件
│   │   │   ├── ApiKeyInput.tsx
│   │   │   └── index.ts
│   │   ├── metadata-source-badge/     # 元数据来源标签
│   │   │   ├── MetadataSourceBadge.tsx
│   │   │   └── index.ts
│   │   └── index.ts                   # 组件入口
│   │
│   ├── flows/                         # 流程族
│   │   ├── scan-flow/                 # 扫描流程
│   │   │   ├── ScanFlow.tsx
│   │   │   └── index.ts
│   │   ├── rename-preview-flow/       # 预览流程
│   │   │   ├── RenamePreviewFlow.tsx
│   │   │   └── index.ts
│   │   ├── manual-review-flow/        # 人工确认流程
│   │   │   ├── ManualReviewFlow.tsx
│   │   │   └── index.ts
│   │   └── rollback-flow/             # 回滚流程
│   │       ├── RollbackFlow.tsx
│   │       └── index.ts
│   │
│   ├── state/                         # 状态族
│   │   ├── scan-state/                # 扫描状态
│   │   │   ├── scanStore.ts
│   │   │   └── index.ts
│   │   ├── preview-state/             # 预览状态
│   │   │   ├── previewStore.ts
│   │   │   └── index.ts
│   │   ├── execution-state/           # 执行状态
│   │   │   ├── executionStore.ts
│   │   │   └── index.ts
│   │   ├── rollback-state/            # 回滚状态
│   │   │   ├── rollbackStore.ts
│   │   │   └── index.ts
│   │   ├── config-state/              # 配置状态
│   │   │   ├── configStore.ts
│   │   │   └── index.ts
│   │   └── metadata-state/            # 元数据状态
│   │       ├── metadataStore.ts
│   │       └── index.ts
│   │
│   └── design-system/                 # 设计系统族
│       ├── tokens/                    # 设计令牌
│       │   ├── colors.ts
│       │   ├── typography.ts
│       │   └── spacing.ts
│       ├── buttons/                   # 按钮组件
│       │   └── Button.tsx
│       └── index.ts
│
├── tests/                             # 前端测试
│   ├── components/
│   ├── pages/
│   └── flows/
│
├── examples/                          # 示例/测试数据
│   └── fixtures/
│       └── sandbox/                   # 测试沙盒
│
├── package.json
├── tsconfig.json
├── tailwind.config.js
└── README.md
```

---

## 入口文件规范

| 文件 | 最大行数 | 允许内容 |
|------|:--------:|---------|
| mod.rs | 30 | pub use, pub mod |
| index.ts | 30 | export, re-export |
| lib.rs | 50 | 模块声明 + Tauri 命令注册 |

---

*下一阶段：module-genealogy.md*
