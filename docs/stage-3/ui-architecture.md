# UI 架构

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## UI 家谱树

```
src/
├── app/                    # 应用壳族
│   ├── App.tsx             # 根组件
│   ├── layout.tsx          # 主布局
│   └── router.tsx          # 路由
│
├── pages/                  # 页面族（只负责组合）
│   ├── scan/               # 扫描页
│   ├── preview/            # 预览页
│   ├── manual-review/      # 人工确认页
│   ├── execution-confirm/  # 执行确认页
│   ├── task-history/       # 历史页
│   └── settings/           # 设置页
│
├── components/             # 组件族（展示+交互）
│   ├── file-table/         # 文件表格
│   ├── risk-badge/         # 风险标签
│   ├── confidence-indicator/ # 置信度
│   ├── path-diff/          # 路径对比
│   ├── confirmation-dialog/ # 确认弹窗
│   └── index.ts            # 薄入口
│
├── flows/                  # 流程族（编排逻辑）
│   ├── scan-flow/          # 扫描流程
│   ├── rename-preview-flow/ # 预览流程
│   ├── manual-review-flow/ # 人工确认流程
│   └── rollback-flow/      # 回滚流程
│
├── state/                  # 状态族（UI 状态）
│   ├── scan-state/         # 扫描状态
│   ├── preview-state/      # 预览状态
│   ├── execution-state/    # 执行状态
│   └── rollback-state/     # 回滚状态
│
└── design-system/          # 设计系统族
    ├── tokens/             # 设计令牌
    ├── buttons/            # 按钮
    └── index.ts            # 薄入口
```

---

## 分层规则

| 层 | 职责 | 禁止 |
|----|------|------|
| pages/ | 页面组合 | 业务逻辑 |
| components/ | 展示和交互 | 文件系统操作 |
| flows/ | UI 流程编排 | 底层文件操作 |
| state/ | UI 状态 | 重命名规则 |
| design-system/ | 视觉基础 | 业务逻辑 |

---

## Tauri IPC 调用

前端通过 Tauri invoke 调用 Rust 后端：

```typescript
// 扫描
const report = await invoke('scan_directory', { path });

// 解析
const items = await invoke('parse_media_files', { mediaItems });

// 预览
const preview = await invoke('generate_preview', { parsedItems, template });

// 执行
const results = await invoke('execute_rename', { previewItems });

// 回滚
const rollback = await invoke('rollback_task', { taskId });
```

---

## 状态管理

使用 Zustand，每个功能族独立 store：

| Store | 职责 |
|-------|------|
| scanStore | 扫描状态、进度、结果 |
| previewStore | 预览列表、筛选、排序 |
| executionStore | 执行状态、结果 |
| rollbackStore | 回滚状态、结果 |

---

*下一阶段：ux-flow.md*
