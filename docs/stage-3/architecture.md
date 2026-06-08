# 系统架构

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、架构概述

media-renamer-Kivo 采用分层架构，前端 React + 后端 Rust 核心通过 Tauri IPC 通信。

```
┌─────────────────────────────────────┐
│            UI Layer (React)         │
│  pages / components / state / flows │
├─────────────────────────────────────┤
│          Tauri IPC Bridge           │
├─────────────────────────────────────┤
│       Application Layer (Rust)      │
│  scan / parse / rename / rollback   │
├─────────────────────────────────────┤
│        Domain Layer (Rust)          │
│  models / rules / validators        │
├─────────────────────────────────────┤
│     Infrastructure Layer (Rust)     │
│  fs / db / config / logger          │
└─────────────────────────────────────┘
```

---

## 二、分层职责

| 层 | 职责 | 技术 |
|----|------|------|
| UI | 页面组合、交互、状态展示 | React + TypeScript |
| Application | 业务流程编排 | Rust |
| Domain | 领域模型、规则、验证 | Rust |
| Infrastructure | 文件系统、数据库、配置 | Rust |

---

## 三、核心模块

| 模块 | 功能族 | 职责 |
|------|--------|------|
| scan | 扫描 | 目录遍历、文件识别、权限检查 |
| parse | 解析 | 文件名解析、分类识别、置信度评分 |
| rename | 重命名 | 模板渲染、冲突检测、安全检查、执行 |
| rollback | 回滚 | 任务回滚、部分回滚、状态检测 |
| audit | 审计 | 日志记录、JSONL 导出 |
| config | 配置 | 模板管理、阈值设置、白名单 |
| shared | 共享 | 路径工具、结果类型、平台适配 |

---

## 四、数据流

```
用户选择目录
    ↓
scan: 遍历目录 → 识别视频文件 → 返回 MediaItem[]
    ↓
parse: 解析文件名 → 返回 ParsedMediaInfo[] (含置信度)
    ↓
rename: 模板渲染 → 冲突检测 → 返回 RenamePreviewItem[]
    ↓
UI: 展示预览 → 人工确认 → 用户点击执行
    ↓
rename: 安全检查 → 执行改名 → 返回 RenameResult[]
    ↓
audit: 记录审计日志
```

---

## 五、技术栈确认

| 组件 | 技术 |
|------|------|
| 桌面框架 | Tauri v2 |
| 前端 | React 18 + TypeScript |
| 核心 | Rust |
| 样式 | Tailwind CSS |
| 状态 | Zustand |
| 数据库 | SQLite (rusqlite) |
| 打包 | MSI + NSIS |

---

*下一阶段：module-tree.md*
