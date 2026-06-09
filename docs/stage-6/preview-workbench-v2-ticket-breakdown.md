# P2 Ticket Breakdown — Preview Workbench v2

> 每张票独立可交付、可回滚。
> 安全底线：不开放真实 rename、不开放 rollback、不联网调用 TMDb、不泄露 API key。

---

## P2-002 Preview Workbench route skeleton

| 项目 | 内容 |
|------|------|
| **目标** | 创建隐藏的 `/workbench` experimental route + 空三栏 page shell，不影响现有页面 |
| **允许修改范围** | `src/app/router.tsx`（新增一条 route）、`src/features/preview-workbench/page/PreviewWorkbenchPage.tsx`（新建） |
| **禁止修改范围** | `src/features/preview/**`、`src/features/scan/**`、`src/features/safety/**`、任何 store、任何 API |
| **安全边界** | 无数据流、无 API 调用、无 rename/rollback、只渲染空布局 |
| **验证命令** | `npm run build && npx tsc --noEmit`；Tauri 窗口访问 `/workbench` 看到空三栏 |
| **需要用户视觉验收** | ✅ 是 — 确认空三栏布局和 `/workbench` route 可达 |

### 交付物

- `PreviewWorkbenchPage.tsx`：渲染三栏占位布局（左 280px / 中 flex / 右 360px）+ 底部 56px action bar 占位
- `router.tsx`：新增 `<Route path="/workbench" element={<PreviewWorkbenchPage />} />`
- Sidebar 不显示 `/workbench` 链接 — 只能通过地址栏手动访问

### 验收标准

1. `/workbench` 在 Tauri 窗口中可达
2. 三栏 + 底部 action bar 占位可见
3. 窗口缩小时布局不塌陷（最小 1024px）
4. `/preview` 等现有页面不受影响

---

## P2-003 Workbench file queue panel

| 项目 | 内容 |
|------|------|
| **目标** | 实现左栏文件队列：从 scanStore 读取 previews，渲染文件列表 + 筛选 + 搜索 + 统计 |
| **允许修改范围** | `src/features/preview-workbench/file-list/`（新建目录及文件）、`src/features/preview-workbench/page/PreviewWorkbenchPage.tsx`（集成左栏） |
| **禁止修改范围** | `src/features/scan/state/scanStore.ts`、`src/features/preview/**`、任何 API |
| **安全边界** | 只读 scanStore.result.previews，不修改任何 store 数据 |
| **验证命令** | `npm run build && npx tsc --noEmit`；Tauri 窗口：scan 后切到 `/workbench` 看到文件列表 |
| **需要用户视觉验收** | ✅ 是 — 确认文件列表渲染、筛选、搜索、统计 |

### 新建文件

```
src/features/preview-workbench/file-list/
├── FileQueuePanel.tsx       # 左栏容器
├── FileQueueItem.tsx        # 单个文件行
├── FileQueueFilter.tsx      # 筛选标签栏
├── FileQueueSearch.tsx      # 搜索输入框
└── FileQueueStats.tsx       # 底部统计
```

### 验收标准

1. 文件列表正确读取 scanStore.result.previews
2. 点击文件行高亮当前选中项
3. 筛选标签（All / Review / Conflict / Safe）正确过滤
4. 搜索框按文件名过滤
5. 底部统计实时更新
6. 100+ 文件不卡（使用虚拟滚动或限制渲染）

---

## P2-004 Candidate panel static scaffold

| 项目 | 内容 |
|------|------|
| **目标** | 实现中栏候选展示：显示当前选中文件的自动匹配候选（proposed_name），不联网 |
| **允许修改范围** | `src/features/preview-workbench/candidate-list/`（新建）、`src/features/preview-workbench/candidate-search/TmdbSearchButton.tsx`（新建，disabled）、`src/features/preview-workbench/page/PreviewWorkbenchPage.tsx`（集成中栏） |
| **禁止修改范围** | 任何 API、scanStore、任何网络调用 |
| **安全边界** | 只读 scanStore 数据，不触发 TMDb 搜索，搜索按钮 disabled |
| **验证命令** | `npm run build && npx tsc --noEmit`；Tauri 窗口：选中文件后中栏显示自动匹配 |
| **需要用户视觉验收** | ✅ 是 — 确认候选展示和 disabled 搜索按钮 |

### 新建文件

```
src/features/preview-workbench/candidate-list/
├── CandidatePanel.tsx       # 中栏容器
├── CandidateCard.tsx        # 单个候选卡片
├── CandidateBadge.tsx       # 来源/置信度标签
└── CandidateEmptyState.tsx  # 无候选提示

src/features/preview-workbench/candidate-search/
└── TmdbSearchButton.tsx     # TMDb 搜索按钮 (disabled)
```

### 验收标准

1. 选中文件后中栏显示自动匹配候选
2. 候选卡片显示 proposed_name、confidence、metadata_source
3. 无选中文件时显示 CandidateEmptyState
4. TMDb 搜索按钮始终 disabled，hover 显示提示
5. 不发出任何网络请求

---

## P2-005 Rename preview detail panel

| 项目 | 内容 |
|------|------|
| **目标** | 实现右栏预览详情：原名→新名对比 + 解析信息 + 冲突列表，不执行 rename |
| **允许修改范围** | `src/features/preview-workbench/rename-preview/`（新建）、`src/features/preview-workbench/page/PreviewWorkbenchPage.tsx`（集成右栏） |
| **禁止修改范围** | 任何 API、任何 store 写操作、任何文件系统操作 |
| **安全边界** | 纯只读展示，不触发任何副作用 |
| **验证命令** | `npm run build && npx tsc --noEmit`；Tauri 窗口：选中文件 + 候选后右栏显示预览 |
| **需要用户视觉验收** | ✅ 是 — 确认预览详情显示正确 |

### 新建文件

```
src/features/preview-workbench/rename-preview/
├── RenamePreviewPanel.tsx   # 右栏容器
├── RenameDiffView.tsx       # 原名 → 新名对比
├── ParsedInfoSummary.tsx    # 解析信息摘要
└── ConflictDetailList.tsx   # 冲突详情列表
```

### 验收标准

1. 原名和新名清晰对比显示
2. 解析信息（title/year/season/episode/resolution/codec/group）正确展示
3. 冲突列表显示冲突类型 + 详细信息
4. 无冲突时不显示冲突区域
5. 不执行任何 rename 操作

---

## P2-006 Workbench dry-run action bar

| 项目 | 内容 |
|------|------|
| **目标** | 实现底部操作栏：Apply Candidate / Clear Selection / Export Plan，禁止 Execute Rename |
| **允许修改范围** | `src/features/preview-workbench/action-bar/`（新建）、`src/features/preview-workbench/state/workbenchStore.ts`（新建）、`src/features/preview-workbench/types/workbench.ts`（新建）、`src/features/preview-workbench/types/candidate.ts`（新建）、`src/features/preview-workbench/page/PreviewWorkbenchPage.tsx`（集成 action bar） |
| **禁止修改范围** | 任何现有 store、任何 API、任何文件系统操作 |
| **安全边界** | Apply/Clear 只改内存状态；Export 只写 JSON/CSV 到用户指定路径；绝不触发 rename/rollback |
| **验证命令** | `npm run build && npx tsc --noEmit`；Tauri 窗口：操作栏三个按钮可用 |
| **需要用户视觉验收** | ✅ 是 — 确认按钮行为和安全性 |

### 新建文件

```
src/features/preview-workbench/action-bar/
├── WorkbenchActionBar.tsx      # 底部操作栏容器
├── ApplyCandidateButton.tsx    # 应用候选按钮
├── ClearSelectionButton.tsx    # 清除选择按钮
└── ExportPlanButton.tsx        # 导出计划按钮

src/features/preview-workbench/state/
└── workbenchStore.ts           # Zustand store

src/features/preview-workbench/types/
├── workbench.ts                # Workbench 核心类型
└── candidate.ts                # 候选相关类型
```

### 验收标准

1. Apply Candidate 按钮将中栏选中候选标记为 applied
2. Clear Selection 清除当前文件的 applied candidate
3. Export Plan 导出 JSON/CSV 计划文件
4. Action Bar 显示「已确认 X/Y 文件」统计
5. 不存在 Execute Rename / Rollback / Delete 按钮
6. Export 后 toast 提示「计划已保存，未修改任何媒体文件」

---

## P2-007 Candidate state store

| 项目 | 内容 |
|------|------|
| **目标** | 完善 workbenchStore：selectedFileId / appliedMap 联动，防止候选与文件错配 |
| **允许修改范围** | `src/features/preview-workbench/state/workbenchStore.ts`、`src/features/preview-workbench/types/**`、所有 preview-workbench 组件（更新引用） |
| **禁止修改范围** | scanStore、auditPageStore、settingsPageStore、任何 API |
| **安全边界** | store 只管理内存状态，不触发副作用 |
| **验证命令** | `npm run build && npx tsc --noEmit`；测试：选择文件 → 选候选 → 切文件 → 回来确认候选仍在 |
| **需要用户视觉验收** | ✅ 是 — 确认状态联动正确 |

### 核心逻辑

```
selectFile(fileId):
  - 设置 selectedFileId = fileId
  - 不清除 appliedMap（applied 是 per-file 持久的）

applyCandidate(fileId, candidate):
  - 校验 candidate 属于该 fileId 的候选列表
  - appliedMap.set(fileId, candidate)
  - 同步更新右栏预览

clearApplied(fileId):
  - appliedMap.delete(fileId)
  - 恢复为自动匹配显示

防御性检查:
  - selectedFileId 无效时 → 清空选择 + 中栏/右栏显示 empty state
  - scanStore.result 变为 null 时 → 清空 appliedMap + selectedFileId
```

### 验收标准

1. 选择文件 → 中栏和右栏同步更新
2. Apply 候选 → 右栏显示 applied 候选的预览
3. 切换文件 → applied 状态不丢失
4. 回到已 applied 的文件 → 显示 applied 候选
5. scanStore.result 清空 → workbench 状态正确重置

---

## P2-008 Safety summary integration

| 项目 | 内容 |
|------|------|
| **目标** | 在右栏集成安全摘要：只读展示当前文件的安全检查和全局安全报告 |
| **允许修改范围** | `src/features/preview-workbench/safety-summary/`（新建）、`src/features/preview-workbench/rename-preview/RenamePreviewPanel.tsx`（集成安全摘要） |
| **禁止修改范围** | 任何现有 safety 组件、scanStore、任何 API |
| **安全边界** | 纯只读，不修改 safety 数据 |
| **验证命令** | `npm run build && npx tsc --noEmit`；Tauri 窗口：右栏显示安全检查和阻塞警告 |
| **需要用户视觉验收** | ✅ 是 — 确认安全摘要显示正确 |

### 新建文件

```
src/features/preview-workbench/safety-summary/
├── SafetySummaryPanel.tsx   # 安全摘要容器
├── SafetyCheckItem.tsx      # 单个安全检查项
└── SafetyBlockingAlert.tsx  # 阻塞警告
```

### 验收标准

1. 选中文件后，右栏显示该文件的 conflicts 作为安全检查项
2. 阻塞冲突显示红色警告
3. 全局安全报告（can_execute / dry_run）显示在安全摘要顶部
4. 无冲突时安全检查区显示「无安全风险」

---

## P2-009 TMDb search design spike

| 项目 | 内容 |
|------|------|
| **目标** | 设计 TMDb 搜索的完整交互流程和数据流，docs-only 或 mocked-only，不真实联网 |
| **允许修改范围** | `docs/stage-6/tmdb-search-design.md`（新建）、`src/features/preview-workbench/candidate-search/`（可新增 mock UI，不调用真实 API） |
| **禁止修改范围** | 任何 API 文件、Rust 后端、scanStore |
| **安全边界** | 不发出任何网络请求；如果做 mock UI，数据来自本地硬编码 |
| **验证命令** | `npm run build && npx tsc --noEmit`（如果涉及前端代码）；否则只检查文档 |
| **需要用户视觉验收** | ⚠️ 可选 — 如果只做文档则不需要；如果做 mock UI 则需要 |

### 设计内容

1. TMDb search API 的 Rust command 接口设计
2. Rate limit 策略（每秒 ≤ 3 次，每天 ≤ 1000 次）
3. Error state 设计（网络错误 / 429 / 空结果 / API key 未配置）
4. Cache 策略（内存缓存，LRU，最多 100 条）
5. 搜索结果 → CandidateCard 的数据映射
6. 用户选择 TMDb 候选后的 applied 流程

### 验收标准

1. 设计文档完整覆盖上述 6 项
2. 不包含任何真实网络调用
3. 不包含 execute_rename / rollback_task

---

## P2-010 Workbench real-window visual pass

| 项目 | 内容 |
|------|------|
| **目标** | 用户在真实 Tauri 窗口中验收 Workbench 全部功能 |
| **允许修改范围** | 仅修复视觉/交互 bug，不新增功能 |
| **禁止修改范围** | 任何新功能、任何 API、任何 store 逻辑 |
| **安全边界** | 只修视觉 bug，不触碰安全边界 |
| **验证命令** | 用户手动验收 |
| **需要用户视觉验收** | ✅ 是 — 这就是验收票 |

### 验收清单

| # | 验收项 | 通过标准 |
|---|--------|----------|
| 1 | 空三栏布局 | 280 / flex / 360 + 底部 56px |
| 2 | 文件列表渲染 | 点击文件 → 高亮 → 中栏/右栏更新 |
| 3 | 筛选功能 | All / Review / Conflict / Safe 正确过滤 |
| 4 | 搜索功能 | 按文件名搜索正确过滤 |
| 5 | 候选展示 | 自动匹配候选显示正确 |
| 6 | TMDb 按钮 disabled | 按钮灰显，hover 显示提示 |
| 7 | 预览详情 | 原名→新名对比 + 解析信息 + 冲突 |
| 8 | 安全摘要 | 安全检查 + 阻塞警告 |
| 9 | Apply Candidate | 选择候选后右栏更新 |
| 10 | Clear Selection | 清除后恢复自动匹配 |
| 11 | Export Plan | 导出 JSON/CSV + toast 提示 |
| 12 | 无 destructive 按钮 | 不存在 Execute/Rollback/Delete |
| 13 | 无 API key 泄露 | 前端无明文 key |
| 14 | 旧 PreviewPage 不受影响 | `/preview` 正常访问 |
| 15 | 100+ 文件不卡 | 帧时间 < 16ms |
| 16 | 无中文竖排 / 布局塌陷 | 最小 1024px 宽度下正常 |
| 17 | 键盘可操作 | Tab/Enter 可导航和选择 |

---

## 票依赖关系

```
P2-002 (route skeleton)
  └──→ P2-003 (file queue) ──→ P2-007 (store) ──→ P2-010 (visual pass)
  └──→ P2-004 (candidate)  ──→ P2-007 (store)
  └──→ P2-005 (preview)    ──→ P2-007 (store)
  └──→ P2-006 (action bar) ──→ P2-007 (store)
  └──→ P2-008 (safety)     ──→ P2-010 (visual pass)
  └──→ P2-009 (TMDb design, independent)
```

**关键路径**：P2-002 → P2-003/004/005/006 (可并行) → P2-007 → P2-008 → P2-010

**独立路径**：P2-009 (可随时进行，不阻塞其他票)

---

## 全局安全约束（所有票通用）

1. ❌ 不开放 `execute_rename` / `rollback_task`
2. ❌ 不添加 Execute / Rename / Rollback / Delete / Overwrite 按钮
3. ❌ 不联网调用 TMDb（P2-009 除外，且只做 mock）
4. ❌ 不泄露 API key 明文
5. ❌ 不修改 scanStore / auditPageStore / settingsPageStore
6. ❌ 不破坏 Scan / Preview / Safety / Audit / Settings 基线
7. ✅ 所有操作可审计
8. ✅ 每张票独立可回滚
