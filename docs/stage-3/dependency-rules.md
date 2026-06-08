# 依赖规则

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、依赖方向总览

```
┌─────────────────────────────────────────────────┐
│                   UI Layer                       │
│  pages/ → flows/ → components/ → state/          │
│  design-system/ (无业务依赖)                      │
└───────────────────────┬─────────────────────────┘
                        │ Tauri IPC (invoke)
                        ↓
┌─────────────────────────────────────────────────┐
│              Application Layer                   │
│  scan/  parse/  rename/  rollback/               │
│  audit/  config/                                 │
└───────────────────────┬─────────────────────────┘
                        │
                        ↓
┌─────────────────────────────────────────────────┐
│               Domain Layer                       │
│  models/  rules/  validators/                    │
└───────────────────────┬─────────────────────────┘
                        │
                        ↓
┌─────────────────────────────────────────────────┐
│           Infrastructure Layer                   │
│  fs/  db/  config/  logger/                      │
└───────────────────────┬─────────────────────────┘
                        │
                        ↓
┌─────────────────────────────────────────────────┐
│              shared/ (纯工具)                     │
│  path_utils/  result_types/  platform/            │
└─────────────────────────────────────────────────┘
```

**核心原则**：依赖只能向下，禁止向上或横向跨层。

---

## 二、Rust 核心依赖矩阵

### 2.1 功能族依赖

| 模块 | 可依赖 | 禁止依赖 |
|------|--------|----------|
| scan/ | shared/ | parse/, rename/, rollback/, audit/, config/, UI |
| parse/ | shared/ | scan/, rename/, rollback/, audit/, config/, UI |
| rename/ | config/, shared/, audit/ | scan/, parse/, rollback/, UI |
| rollback/ | audit/, shared/ | scan/, parse/, rename/, config/, UI |
| audit/ | shared/ | scan/, parse/, rename/, rollback/, config/, UI |
| config/ | shared/ | scan/, parse/, rename/, rollback/, audit/, UI |
| shared/ | (无依赖) | 所有功能族 |

### 2.2 详细依赖关系

```
scan/scanner.rs ──→ shared/path_utils.rs
scan/file_detector.rs ──→ (无依赖)
scan/progress.rs ──→ (无依赖)

parse/movie_parser.rs ──→ shared/result_types.rs
parse/series_parser.rs ──→ shared/result_types.rs
parse/anime_parser.rs ──→ shared/result_types.rs
parse/special_parser.rs ──→ shared/result_types.rs
parse/confidence.rs ──→ (无依赖)
parse/classifier.rs ──→ movie_parser, series_parser, anime_parser, special_parser

rename/template.rs ──→ config/template_manager.rs
rename/conflict_detector.rs ──→ shared/path_utils.rs
rename/safety_checker.rs ──→ conflict_detector.rs
rename/executor.rs ──→ audit/logger.rs, shared/path_utils.rs
rename/preview_generator.rs ──→ template.rs, conflict_detector.rs

rollback/rollback_executor.rs ──→ audit/logger.rs
rollback/state_checker.rs ──→ (无依赖)

audit/logger.rs ──→ (无依赖)
audit/exporter.rs ──→ (无依赖)

config/template_manager.rs ──→ (无依赖)
config/threshold.rs ──→ (无依赖)
```

---

## 三、UI 依赖矩阵

### 3.1 层级依赖

| 层 | 可依赖 | 禁止依赖 |
|----|--------|----------|
| pages/ | flows/, components/, state/ | Rust 后端直接调用 |
| flows/ | components/, state/, Tauri invoke | pages/, design-system/ |
| components/ | design-system/ | flows/, state/, Rust 后端 |
| state/ | Tauri invoke | pages/, flows/, components/ |
| design-system/ | (无依赖) | 所有业务层 |

### 3.2 详细依赖关系

```
pages/scan/ ──→ flows/scan-flow/, components/file-table/, state/scan-state/
pages/preview/ ──→ flows/rename-preview-flow/, components/*, state/preview-state/
pages/manual-review/ ──→ flows/manual-review-flow/, components/*, state/preview-state/
pages/execution-confirm/ ──→ components/confirmation-dialog/, state/execution-state/
pages/task-history/ ──→ flows/rollback-flow/, state/rollback-state/
pages/settings/ ──→ (直接 invoke config 命令)

flows/scan-flow/ ──→ state/scan-state/, invoke('scan_directory')
flows/rename-preview-flow/ ──→ state/preview-state/, invoke('parse_media_files'), invoke('generate_preview')
flows/manual-review-flow/ ──→ state/preview-state/, invoke('apply_manual_review')
flows/rollback-flow/ ──→ state/rollback-state/, invoke('rollback_task')

components/file-table/ ──→ design-system/
components/risk-badge/ ──→ design-system/
components/confidence-indicator/ ──→ design-system/
components/path-diff/ ──→ design-system/
components/confirmation-dialog/ ──→ design-system/
```

---

## 四、禁止模式

### 4.1 循环依赖

```
❌ scan/ → parse/ → scan/
❌ rename/ → rollback/ → rename/
❌ flows/ → pages/ → flows/
```

### 4.2 跨层依赖

```
❌ UI 直接调用 Rust 模块（绕过 Tauri IPC）
❌ scan/ 依赖 audit/（应通过 rename/ 转发）
❌ parse/ 依赖 config/（解析器不应关心模板）
```

### 4.3 上帝模块

```
❌ shared/utils.rs（万能工具）
❌ shared/helpers.rs（万能助手）
❌ rename/mod.rs（含业务逻辑的入口文件）
❌ components/common/（通用组件垃圾场）
```

---

## 五、依赖注入点

### 5.1 Rust 侧

| 注入点 | 用途 | 方式 |
|--------|------|------|
| config::template_manager | 模板获取 | 静态函数调用 |
| config::threshold | 阈值获取 | 静态函数调用 |
| audit::logger | 日志记录 | 静态函数调用 |

**MVP 策略**：使用静态函数 + 全局状态（Mutex），避免过度抽象。

### 5.2 UI 侧

| 注入点 | 用途 | 方式 |
|--------|------|------|
| Zustand store | 状态访问 | useStore() hook |
| Tauri invoke | 后端调用 | 直接 invoke() |

**MVP 策略**：无 DI 框架，直接调用。

---

## 六、依赖版本锁定

### 6.1 Rust (Cargo.toml)

```toml
[dependencies]
tauri = { version = "2", features = [...] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled"] }
regex = "1"
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
camino = "1"
walkdir = "2"
```

### 6.2 Frontend (package.json)

```json
{
  "dependencies": {
    "react": "^18.3",
    "react-dom": "^18.3",
    "react-router-dom": "^6",
    "zustand": "^4.5",
    "@tauri-apps/api": "^2",
    "lucide-react": "^0.400"
  },
  "devDependencies": {
    "typescript": "^5.4",
    "vite": "^5",
    "@vitejs/plugin-react": "^4",
    "tailwindcss": "^3.4",
    "vitest": "^1.6",
    "@testing-library/react": "^15"
  }
}
```

---

## 七、依赖检查清单

每个 PR / 提交前检查：

- [ ] 无循环依赖
- [ ] 无跨层依赖
- [ ] 无上帝模块
- [ ] 入口文件 ≤30 行
- [ ] shared/ 不依赖任何功能族
- [ ] UI 不直接调用 Rust 模块

---

*下一阶段：interfaces-and-contracts.md*
