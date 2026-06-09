# P2-001 — Preview Workbench v2 设计文档

> 分支：`preview-workbench-v2-design`
> 起始 HEAD：`97c5b5c`（`ui-redesign-dry-run-mvp` Final PR/Merge Ready）
> 类型：design-audit / docs-only
> 日期：2026-06-09

---

## 1. 设计目标

### 1.1 为什么旧版三栏更适合人工媒体匹配

当前 dry-run MVP 采用「Scan → Preview → Safety → Audit」线性向导模式，用户在每页之间来回切换才能完成「查看文件 → 选候选 → 确认预览」的操作闭环。对于 **10+ 文件的批量场景**，这种模式有三个核心痛点：

1. **上下文断裂**：用户在 Preview 页看完文件列表，切到 Safety 页看安全检查，再回来找那个文件，已经忘了行号。
2. **候选确认低效**：每个文件只有一个 `proposed_name`，没有「候选列表 → 人工选择」的交互路径。自动匹配低置信度时，用户无从介入。
3. **反馈链路长**：修改 → 预览 → 安全检查 → 回到修改，每一步都是整页切换。

三栏工作台将这些信息同时呈现在一屏之内：

| 左栏 | 中栏 | 右栏 |
|------|------|------|
| 文件队列 + 状态筛选 | 匹配候选（当前 proposed + 未来 TMDb 候选） | 重命名预览 + 安全摘要 + 详情 |
| 我要处理哪些文件？ | 这些文件能匹配什么？ | 匹配后变成什么样？安全吗？ |

**一眼闭环**，无需翻页。

### 1.2 当前 dry-run MVP 的不足

| 维度 | 现状 | 问题 |
|------|------|------|
| 预览方式 | 单页表格，所有 `RenamePreviewItem` 平铺 | 无法聚焦单个文件的决策 |
| 候选选择 | 无 — 只有 `proposed_name` 一条路径 | 低置信度文件无人工补救手段 |
| 安全联动 | Safety 独立页面 | 用户需翻页才能看到冲突/风险 |
| 筛选能力 | `previewFilterStore` 只支持 text + 4 种 filter | 无法按文件/候选/状态三维筛选 |
| 批量确认 | 无「逐个确认 → 批量导出计划」流程 | 每次都是全量 dry-run，无法部分确认 |

### 1.3 v2 的目标不是炫 UI

v2 的目标是 **提升批量确认效率**：

- 从「翻页式全量预览」→「一屏式逐文件决策」
- 从「只有一个 proposed_name」→「候选列表 + 人工选择」
- 从「安全检查独立页」→「安全摘要内嵌在决策流程中」
- 保留 dry-run 底座 — 任何操作都不触碰真实文件系统

---

## 2. 页面布局草案

### 2.1 整体布局

```
┌─────────────────────────────────────────────────────────────────────┐
│  TopBar                                                            │
├──────────┬────────────────────────────────────┬─────────────────────┤
│          │                                    │                     │
│  左栏     │  中栏                                │  右栏                │
│  文件队列  │  匹配候选                             │  预览/安全/详情       │
│          │                                    │                     │
│  280px   │  flex-1                             │  360px              │
│  fixed   │  auto                               │  fixed              │
│          │                                    │                     │
│          │                                    │                     │
├──────────┴────────────────────────────────────┴─────────────────────┤
│  底部 Action Bar (56px)                                            │
│  [Apply Candidate] [Clear Selection] [Export Plan]                 │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 左栏 — 文件队列 (FileQueuePanel)

```
┌──────────────────────┐
│ 🔍 搜索文件名...       │
├──────────────────────┤
│ 筛选: All | ⚠ Review │
│       | ❌ Conflict  │
│       | ✅ Safe       │
├──────────────────────┤
│ ┌──────────────────┐ │
│ │ 🎬 Movie.Name... │ │ ← selected / highlighted
│ │   Movie · ⚠ Review│ │
│ └──────────────────┘ │
│ ┌──────────────────┐ │
│ │ 📺 Show.S01E02.. │ │
│ │   Series · ✅ Safe │ │
│ └──────────────────┘ │
│ ┌──────────────────┐ │
│ │ 🎭 Anime.EP03... │ │
│ │   Anime · ❌ Conf │ │
│ └──────────────────┘ │
│         ...          │
├──────────────────────┤
│ 12 文件 · 3 ⚠ · 2 ❌ │
└──────────────────────┘
```

**核心交互**：

- 点击文件 → 更新中栏候选 + 右栏预览
- 筛选标签只影响左栏可见范围，不影响数据
- 底部统计实时更新
- 虚拟滚动支持 100+ 文件

### 2.3 中栏 — 匹配候选 (CandidatePanel)

```
┌────────────────────────────────────┐
│ 匹配候选 (3)                        │
├────────────────────────────────────┤
│ ┌────────────────────────────────┐ │
│ │ ★ 自动匹配          置信度 0.92  │ │ ← applied candidate
│ │                                │ │
│ │ Movie Name (2024)              │ │
│ │ /media/Movie Name (2024).mkv   │ │
│ │ 来源: LocalRule                 │ │
│ │ [✓ 已选择]                      │ │
│ └────────────────────────────────┘ │
│ ┌────────────────────────────────┐ │
│ │ 候选 2              置信度 0.71  │ │
│ │                                │ │
│ │ Another Movie (2023)           │ │
│ │ 来源: TMDb                      │ │
│ │ [选择此候选]                    │ │
│ └────────────────────────────────┘ │
│ ┌────────────────────────────────┐ │
│ │ 候选 3              置信度 0.55  │ │
│ │                                │ │
│ │ Some Title (2022)              │ │
│ │ 来源: TMDb                      │ │
│ │ [选择此候选]                    │ │
│ └────────────────────────────────┘ │
├────────────────────────────────┤
│ [🔍 搜索 TMDb] ← 未来票，当前禁用  │
└────────────────────────────────┘
```

**核心交互**：

- 自动匹配 = `proposed_name`（来自 PipelineResult.previews[i]），始终显示
- 候选 2..N = 未来 TMDb search 结果，当前阶段只有自动匹配一个候选
- 点击「选择此候选」→ 更新右栏预览 + 标记 applied
- 「搜索 TMDb」按钮当前 disabled，灰显，hover 提示「需要 TMDb 联网权限，敬请期待」

### 2.4 右栏 — 预览 / 安全摘要 / 详情 (PreviewDetailPanel)

```
┌─────────────────────────┐
│ 重命名预览                │
├─────────────────────────┤
│ 原始文件名                │
│ Movie.Name.2024.1080p... │
│                         │
│ ↓ 重命名为                │
│                         │
│ Movie Name (2024).mkv   │
│                         │
│ 置信度: 0.92  ✅ 高       │
│ 来源: LocalRule          │
│ 类型: Movie              │
├─────────────────────────┤
│ 安全摘要                  │
├─────────────────────────┤
│ ✅ 目标路径不存在冲突       │
│ ✅ 文件名无非法字符         │
│ ⚠ 路径长度接近限制 (230)   │
│ ❌ 目标文件已存在           │
├─────────────────────────┤
│ 文件详情                  │
├─────────────────────────┤
│ 解析: Movie Name (2024)  │
│ 季: -  集: -              │
│ 分辨率: 1080p             │
│ 视频编码: x264            │
│ 音频编码: AAC             │
│ 发布组: GROUP             │
├─────────────────────────┤
│ 冲突详情                  │
├─────────────────────────┤
│ ❌ TargetExists           │
│   /media/Movie Name.mkv  │
│   已存在，将跳过           │
└─────────────────────────┘
```

**核心交互**：

- 实时反映左栏选中文件 + 中栏 applied candidate
- 安全摘要只读展示 `RenamePreviewItem.conflicts`
- 详情展示 `ParsedMediaInfo` 的关键字段
- 无编辑操作 — 纯只读确认面板

### 2.5 底部 — Dry-run Action Bar

```
┌─────────────────────────────────────────────────────────────────────┐
│  已确认 8/12 文件  │  [Apply Candidate]  [Clear Selection]  [Export Plan]  │
└─────────────────────────────────────────────────────────────────────┘
```

**按钮说明**：

| 按钮 | 行为 | 安全边界 |
|------|------|----------|
| Apply Candidate | 将中栏选中的候选标记为 applied，更新右栏预览 | 仅内存状态变更，不触发 rename |
| Clear Selection | 清除当前文件的 applied candidate，恢复为自动匹配 | 仅内存状态变更 |
| Export Plan | 导出当前所有 applied 候选为 JSON/CSV 计划文件 | 只写文档文件，不触碰媒体文件 |

**绝对禁止出现的按钮**：Execute Rename / Rollback / Delete / Overwrite / Confirm & Execute

---

## 3. 数据流设计

### 3.1 scanStore 当前能提供什么

```typescript
// scanStore.result: PipelineResult | null
PipelineResult = {
  task_id: string;           // 会话 ID
  scan: ScanSummary;         // 扫描统计
  previews: RenamePreviewItem[];  // ★ 核心数据 — 所有文件的预览项
  safety: SafetyReport;      // 全局安全报告
  parsed_count: number;
  unknown_count: number;
}

// 每个 RenamePreviewItem 包含：
RenamePreviewItem = {
  id: string;                // 唯一标识
  parsed_info: ParsedMediaInfo;   // 解析出的媒体信息
  source_path: string;       // 原始路径
  original_name: string;     // 原始文件名
  proposed_name: string;     // ★ 自动匹配的提议名（= 唯一候选）
  target_path: string;       // 目标路径
  media_type: MediaType;     // 媒体类型
  confidence: number;        // 置信度 0..1
  needs_manual_review: bool; // 是否需要人工审核
  should_skip: bool;         // 是否建议跳过
  conflicts: RenameConflict[];  // 冲突列表
  evidence: RuleMatchEvidence[];// 规则匹配证据
  metadata_source: 'LocalRule' | 'TMDb' | 'Manual'; // 元数据来源
}
```

**关键结论**：scanStore 已经提供了工作台所需的全部原始数据。Workbench 不需要新的 API 调用，只需要从 `scanStore.result.previews` 读取 + 维护选择状态。

### 3.2 Preview 数据如何进入 Workbench

```
scanStore.result.previews (RenamePreviewItem[])
    │
    ├──→ FileQueuePanel: 读取 id, original_name, media_type, confidence,
    │                    needs_manual_review, should_skip, conflicts
    │                    生成队列项 + 状态标签
    │
    ├──→ CandidatePanel: 读取 proposed_name, confidence, metadata_source
    │                    生成候选列表（当前只有 1 个自动匹配）
    │
    └──→ PreviewDetailPanel: 读取全部字段
                         生成预览 + 安全摘要 + 详情
```

**数据流方向**：单向，从 scanStore 到各 panel。无反向写入 scanStore。

### 3.3 选择状态归属

```
                     ┌─────────────────────────────┐
                     │   workbenchStore             │
                     │                              │
                     │   selectedFileId: string|null│ ← 左栏点击
                     │   appliedMap:                │ ← 中栏 Apply
                     │     Record<fileId, candidate>│
                     │   searchQuery: string        │ ← TMDb 搜索(未来)
                     │   isSearching: boolean       │ ← TMDb 搜索(未来)
                     └─────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
         FileQueue      CandidatePanel  PreviewDetail
         (highlight)    (applied mark)  (preview content)
```

### 3.4 哪些状态应该进 store

| 状态 | 归属 | 理由 |
|------|------|------|
| `selectedFileId` | **store** | 多面板共享：左栏高亮 + 中栏加载候选 + 右栏加载预览 |
| `appliedMap` | **store** | 跨组件共享：中栏标记 + 右栏内容 + Action Bar 统计 |
| `searchQuery` | **store** | 未来 TMDb 搜索跨组件共享 |
| `isSearching` | **store** | 搜索加载状态，中栏和 Action Bar 都需要 |
| 队列筛选 filter | **local state** | 仅影响左栏，不需要共享 |
| 候选列表展开/折叠 | **local state** | 仅影响中栏单个候选卡片 |
| 右栏 Tab 切换 | **local state** | 仅影响右栏内部展示 |

**原则**：跨面板共享 → store；单面板内部 → local state。

### 3.5 store 与 local state 的边界

```
workbenchStore (Zustand)
├── 核心选择状态 (跨面板)
│   ├── selectedFileId
│   ├── appliedMap: Map<fileId, AppliedCandidate>
│   └── 选中/应用 actions
│
├── TMDb 搜索状态 (未来)
│   ├── searchQuery
│   ├── isSearching
│   └── searchResults: TmdbCandidate[]
│
└── 不包含
    ├── 队列筛选条件 → FileQueuePanel local state
    ├── 候选卡片 UI 状态 → CandidateCard local state
    └── 右栏 Tab 状态 → PreviewDetailPanel local state

AppliedCandidate 类型：
{
  fileId: string;
  candidateType: 'auto' | 'tmdb' | 'manual';  // 候选来源
  proposedName: string;   // 候选提议的文件名
  confidence: number;     // 候选置信度
  source: string;         // 'LocalRule' | 'TMDb' | 'Manual'
}
```

---

## 4. 安全边界

### 4.1 v2 阶段仍然禁止真实 rename

| 操作 | v2 允许 | 说明 |
|------|---------|------|
| 查看文件队列 | ✅ | 只读 scanStore 数据 |
| 查看匹配候选 | ✅ | 只读 previews 数据 |
| 选择候选 | ✅ | 仅内存状态变更 |
| 清除选择 | ✅ | 仅内存状态变更 |
| 导出计划文件 | ✅ | 写 JSON/CSV 文档，不触碰媒体文件 |
| 执行重命名 | ❌ | `execute_rename` 仍 deferred |
| 回滚任务 | ❌ | `rollback_task` 仍 deferred |
| 删除文件 | ❌ | 无此功能 |
| 覆盖文件 | ❌ | 无此功能 |

### 4.2 只允许的 action 闭环

```
选择文件 → 查看候选 → 选择/切换候选 → 确认预览 → 导出计划
                                                     ↓
                                              用户在文件系统外
                                              自行确认或推迟
```

**不存在**「确认 → 执行」的按钮或入口。

### 4.3 审计可追溯

所有用户操作（选择候选、清除选择、导出计划）都应有对应的审计日志记录。当前阶段不要求实时写入 audit API，但 Action Bar 的 Export Plan 应包含完整的操作时间线。

---

## 5. TMDb / metadata 边界

### 5.1 当前只允许 API key 保存

- Settings 页已实现 TMDb API Key 的安全保存 (`do_set_tmdb_api_key`)
- 保存 ≠ 使用 — 保存后不会自动调用 TMDb
- `getTmdbApiKeyStatus` 只返回 `{ configured: boolean }`，不返回明文 key

### 5.2 不在本票联网调用 TMDb

- Workbench v2 的中栏「搜索 TMDb」按钮始终 disabled
- hover 提示「TMDb 联网搜索功能正在开发中」
- 候选列表只有 1 个自动匹配（来自 `proposed_name`）

### 5.3 未来 TMDb search 必须单独 ticket

未来 TMDb 集成需要以下独立票：

1. **TMDb search API 封装** — Rust 后端新增 `search_tmdb` command
2. **TMDb 搜索 UI** — 中栏搜索框 + 结果列表
3. **TMDb rate limit** — 请求频率控制、配额管理
4. **TMDb error handling** — 网络错误、空结果、超时
5. **TMDb cache** — 搜索结果本地缓存，避免重复请求

### 5.4 Rate limit / Error / Empty state 设计预留

即使当前不实现 TMDb 联网，也需要在设计层面预留：

| 场景 | UI 预留 |
|------|---------|
| Rate limit (429) | 中栏顶部 toast: 「TMDb 请求过于频繁，请稍后再试」 |
| 网络错误 | 候选列表区显示错误提示 + 重试按钮 |
| 空结果 | 候选列表区显示「未找到匹配结果，请尝试其他关键词」 |
| API key 未配置 | 「搜索 TMDb」按钮 disabled + tooltip 「请先在设置中配置 TMDb API Key」 |
| 搜索中 | 候选列表区显示 loading skeleton |

---

## 6. 组件家谱设计

### 6.1 目录结构

```
src/features/preview-workbench/
│
├── page/
│   └── PreviewWorkbenchPage.tsx      # 页面入口，三栏布局 + Action Bar
│
├── file-list/
│   ├── FileQueuePanel.tsx            # 左栏容器
│   ├── FileQueueItem.tsx             # 单个文件行
│   ├── FileQueueFilter.tsx           # 筛选标签栏
│   ├── FileQueueSearch.tsx           # 搜索输入框
│   └── FileQueueStats.tsx            # 底部统计
│
├── candidate-list/
│   ├── CandidatePanel.tsx            # 中栏容器
│   ├── CandidateCard.tsx             # 单个候选卡片
│   ├── CandidateBadge.tsx            # 来源/置信度标签
│   └── CandidateEmptyState.tsx       # 无候选提示
│
├── candidate-search/
│   ├── TmdbSearchButton.tsx          # TMDb 搜索入口 (当前 disabled)
│   ├── TmdbSearchInput.tsx           # 搜索框 (未来启用)
│   └── TmdbSearchResult.tsx          # 搜索结果项 (未来启用)
│
├── rename-preview/
│   ├── RenamePreviewPanel.tsx        # 右栏容器
│   ├── RenameDiffView.tsx            # 原名 → 新名对比
│   ├── ParsedInfoSummary.tsx         # 解析信息摘要
│   └── ConflictDetailList.tsx        # 冲突详情列表
│
├── safety-summary/
│   ├── SafetySummaryPanel.tsx        # 安全摘要容器
│   ├── SafetyCheckItem.tsx           # 单个安全检查项
│   └── SafetyBlockingAlert.tsx       # 阻塞警告
│
├── action-bar/
│   ├── WorkbenchActionBar.tsx        # 底部操作栏容器
│   ├── ApplyCandidateButton.tsx      # 应用候选按钮
│   ├── ClearSelectionButton.tsx      # 清除选择按钮
│   └── ExportPlanButton.tsx          # 导出计划按钮
│
├── state/
│   └── workbenchStore.ts             # Zustand store
│
└── types/
    ├── workbench.ts                  # Workbench 核心类型
    └── candidate.ts                  # 候选相关类型
```

### 6.2 单文件职责清单

| 文件 | 唯一职责 | 依赖 |
|------|----------|------|
| `PreviewWorkbenchPage.tsx` | 三栏布局 + Action Bar 组装 | file-list, candidate-list, rename-preview, safety-summary, action-bar |
| `FileQueuePanel.tsx` | 左栏容器：组装搜索 + 筛选 + 列表 + 统计 | FileQueueItem, FileQueueFilter, FileQueueSearch, FileQueueStats, workbenchStore |
| `FileQueueItem.tsx` | 渲染单个文件行：文件名 + 类型图标 + 状态标签 | shared/ui/Badge |
| `FileQueueFilter.tsx` | 筛选标签栏：All / Review / Conflict / Safe | (local state for activeFilter) |
| `FileQueueSearch.tsx` | 搜索输入框 | (local state for query) |
| `FileQueueStats.tsx` | 底部统计：总数 / Review / Conflict | workbenchStore |
| `CandidatePanel.tsx` | 中栏容器：候选列表 + 搜索入口 | CandidateCard, TmdbSearchButton, workbenchStore |
| `CandidateCard.tsx` | 单个候选卡片：名称 + 置信度 + 来源 + 选择按钮 | CandidateBadge, workbenchStore |
| `CandidateBadge.tsx` | 来源/置信度标签 | shared/ui/Badge |
| `CandidateEmptyState.tsx` | 无候选提示 | (none) |
| `TmdbSearchButton.tsx` | TMDb 搜索按钮 (当前 disabled) | workbenchStore (检查 API key status) |
| `TmdbSearchInput.tsx` | 搜索输入框 (未来启用) | workbenchStore |
| `TmdbSearchResult.tsx` | 搜索结果项 (未来启用) | workbenchStore |
| `RenamePreviewPanel.tsx` | 右栏容器：预览 + 安全 + 详情 | RenameDiffView, ParsedInfoSummary, ConflictDetailList |
| `RenameDiffView.tsx` | 原名 → 新名对比展示 | (none) |
| `ParsedInfoSummary.tsx` | 解析信息字段列表 | (none) |
| `ConflictDetailList.tsx` | 冲突详情列表 | (none) |
| `SafetySummaryPanel.tsx` | 安全摘要容器 | SafetyCheckItem, SafetyBlockingAlert |
| `SafetyCheckItem.tsx` | 单个安全检查行 | (none) |
| `SafetyBlockingAlert.tsx` | 阻塞警告横幅 | (none) |
| `WorkbenchActionBar.tsx` | 底部操作栏容器 | ApplyCandidateButton, ClearSelectionButton, ExportPlanButton, workbenchStore |
| `ApplyCandidateButton.tsx` | 应用候选按钮 | workbenchStore |
| `ClearSelectionButton.tsx` | 清除选择按钮 | workbenchStore |
| `ExportPlanButton.tsx` | 导出计划按钮 | workbenchStore |
| `workbenchStore.ts` | 选择状态管理 | types/workbench, types/candidate |
| `workbench.ts` | Workbench 核心类型定义 | api/session/types |
| `candidate.ts` | 候选相关类型定义 | api/session/types |

### 6.3 禁止项

- ❌ `helpers.ts` / `utils.ts` / `manager.ts` / `service.ts` / `glue.ts` / `facade.ts`
- ❌ `index.ts` 做逻辑编排 — 只做 registry / re-export
- ❌ 一个文件包含两个面板的渲染逻辑
- ❌ store 文件包含组件渲染逻辑
- ❌ 类型文件包含运行时逻辑

---

## 7. UI 迁移策略

### 7.1 不推倒当前 PreviewPage

当前 `PreviewPage` 是 PR-ready 的稳定代码，不能破坏。

### 7.2 渐进式迁移路线

```
Phase 1 (P2-002): 隐藏 route + page shell
  /workbench (experimental, 不在 Sidebar 导航中显示)
  PreviewWorkbenchPage 只渲染空三栏布局

Phase 2 (P2-003): 左栏文件队列
  从 scanStore 读取 previews，渲染文件列表
  当前 PreviewPage 不受影响

Phase 3 (P2-004): 中栏候选骨架
  只显示自动匹配(proposed_name)作为唯一候选
  不联网

Phase 4 (P2-005): 右栏预览详情
  显示原名→新名对比 + 解析信息 + 冲突
  不执行 rename

Phase 5 (P2-006): 底部 Action Bar
  Apply / Clear / Export 三个操作
  Export 只写 JSON/CSV

Phase 6 (P2-007): Store 完善
  appliedMap + 选择联动
  确保状态一致性

Phase 7 (P2-008): 安全摘要集成
  从现有 safety 数据只读展示

Phase 8 (P2-010): 用户视觉验收
  真实 Tauri 窗口验收

Phase 9 (未来): 替换 PreviewPage
  Workbench 功能稳定后，将 /preview route 重定向到 /workbench
  旧 PreviewPage 代码保留但不再路由
```

### 7.3 每一步都能回滚

- 每张票都是独立的 Git commit
- 如果某票有问题，`git revert` 单个 commit 即可
- 旧 PreviewPage 始终可通过 `/preview` route 访问
- 新 Workbench 始终可通过 `/workbench` experimental route 访问
- 两套页面并存，互不影响

---

## 8. 风险清单

### 8.1 UI 复杂度风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 三栏布局在小窗口下挤压 | 内容不可读 | 最小窗口宽度 1024px；低于时显示提示而非挤压 |
| 右栏内容过多需要滚动 | 信息丢失 | 安全摘要 + 预览用折叠区段；关键信息优先展示 |
| 移动端不可用 | 不可用 | 不支持移动端 — 这是桌面应用 |

### 8.2 状态错配风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| selectedFileId 对应的 preview 不存在 | 中栏/右栏空白 | 防御性检查：如果 id 无效则清空选择 |
| appliedMap 中的候选与文件不对应 | 预览显示错误 | appliedMap 的 key 是 fileId，value 必须属于该文件的候选列表 |
| scanStore.result 被清空时 workbench 状态残留 | 引用已不存在的数据 | workbenchStore 监听 scanStore.result 变化，null 时清空 |

### 8.3 Candidate selection 与 Rename preview 不一致风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 用户选择候选 A 后，右栏仍显示候选 B 的预览 | 用户确认了错误的重命名 | 选择候选 → 立即更新右栏（同步更新，非异步） |
| 用户快速切换文件，右栏更新延迟 | 短暂的不一致 | 取消前一次渲染，只响应最新的 selectedFileId |
| appliedMap 记录了候选 A，但用户又切换了文件且选了候选 B | 统计不准确 | appliedMap 是 per-file 的，每个文件独立记录 |

### 8.4 Destructive action 误暴露风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| Action Bar 意外出现 Execute 按钮 | 用户可能误操作 | 按钮白名单：只允许 Apply / Clear / Export |
| Export Plan 被误认为执行 | 用户以为文件已被修改 | Export 按钮文案明确「导出计划文件」；导出后 toast 提示「计划已保存，未修改任何媒体文件」 |
| 代码合并时意外引入 rename 调用 | 安全底座被突破 | CI 检查：grep `execute_rename` / `rollback_task`，如果出现在前端代码则 fail |

### 8.5 TMDb 联网失败风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 未来 TMDb 调用超时 | 候选列表加载中卡住 | 超时 5s + 错误提示 + 重试按钮 |
| TMDb 返回错误数据 | 候选信息错误 | 显示置信度，低置信度标黄 |
| API key 泄露 | 安全风险 | 前端永远不持有明文 key；所有 TMDb 调用走 Rust 后端 |

### 8.6 大库性能风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| 1000+ 文件的队列渲染慢 | 左栏卡顿 | 虚拟滚动（react-window 或 @tanstack/virtual） |
| 频繁切换文件触发大量重渲染 | 整体卡顿 | workbenchStore 用 selector 精确订阅；React.memo 隔离面板 |
| appliedMap 过大 | 内存压力 | Map 结构，O(1) 读写；1000 文件 < 100KB |

---

## 9. 验收标准

| # | 标准 | 验证方式 |
|---|------|----------|
| 1 | 100+ 文件列表不卡（< 16ms 帧时间） | Chrome DevTools Performance 面板 |
| 2 | 状态筛选清晰：All / Review / Conflict / Safe | 真实 Tauri 窗口视觉验收 |
| 3 | 当前文件、候选、预览一眼能对应 | 左栏高亮 ↔ 中栏候选 ↔ 右栏预览 三联动 |
| 4 | 无真实文件修改 | 无 `execute_rename` / `rollback_task` 调用 |
| 5 | 无 destructive button | Action Bar 只有 Apply / Clear / Export |
| 6 | 无 API key 泄露 | 前端代码无明文 key；只读 `configured: boolean` |
| 7 | 无中文竖排 / 布局塌陷 | 最小 1024px 宽度下无布局异常 |
| 8 | 旧 PreviewPage 不受影响 | `/preview` route 正常访问 |
| 9 | Workbench 可通过 experimental route 访问 | `/workbench` route 正常访问 |
| 10 | Export Plan 输出完整可读 | JSON/CSV 包含所有 applied 候选信息 |

---

## 附录 A：与现有数据类型的映射

```
PipelineResult.previews[i]  →  FileQueueItem props
  .id                         →  fileId
  .original_name              →  fileName
  .media_type                 →  mediaType (显示图标)
  .confidence                 →  confidence (显示标签)
  .needs_manual_review        →  status: 'review'
  .should_skip                →  status: 'skipped'
  .conflicts.length > 0       →  status: 'conflict'

PipelineResult.previews[i]  →  CandidateCard props
  .proposed_name              →  candidateName (auto match)
  .confidence                 →  confidence
  .metadata_source            →  source

PipelineResult.previews[i]  →  RenamePreviewPanel props
  .original_name              →  oldName
  .proposed_name / applied    →  newName (取决于 appliedMap)
  .parsed_info                →  ParsedInfoSummary
  .conflicts                  →  ConflictDetailList

PipelineResult.safety        →  SafetySummaryPanel props
  .checks[]                   →  SafetyCheckItem[]
  .blocking_reasons[]         →  SafetyBlockingAlert
  .can_execute                →  全局状态指示
```

## 附录 B：workbenchStore 核心接口草案

```typescript
interface WorkbenchState {
  // 选择状态
  selectedFileId: string | null;
  appliedMap: Map<string, AppliedCandidate>;

  // TMDb 搜索 (未来)
  searchQuery: string;
  isSearching: boolean;
  searchResults: TmdbCandidate[];
  searchError: string | null;

  // 计算属性
  appliedCount: number;
  totalFiles: number;

  // Actions
  selectFile: (fileId: string | null) => void;
  applyCandidate: (fileId: string, candidate: AppliedCandidate) => void;
  clearApplied: (fileId: string) => void;
  clearAllApplied: () => void;
  reset: () => void;

  // TMDb actions (未来)
  setSearchQuery: (query: string) => void;
  searchTmdb: (query: string) => Promise<void>;
}

interface AppliedCandidate {
  fileId: string;
  candidateType: 'auto' | 'tmdb' | 'manual';
  proposedName: string;
  confidence: number;
  source: string;
}
```
