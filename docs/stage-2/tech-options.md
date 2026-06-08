# 技术选型方案比较

> 阶段 2：技术选型审查
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 候选方案

### 方案 A：Tauri v2 + React + TypeScript + Rust Core

| 层 | 技术 | 说明 |
|----|------|------|
| 桌面框架 | Tauri v2 | Rust 后端 + WebView 前端 |
| 前端 | React 18 + TypeScript | UI 渲染 |
| 核心逻辑 | Rust | 文件扫描、解析、重命名、回滚 |
| 数据库 | SQLite (rusqlite) | 任务/日志持久化 |
| 样式 | Tailwind CSS | 原子化 CSS |
| 状态管理 | Zustand | 轻量级状态管理 |
| 打包 | Tauri bundler | MSI/NSIS |

### 方案 B：Electron + React + TypeScript

| 层 | 技术 | 说明 |
|----|------|------|
| 桌面框架 | Electron | Chromium + Node.js |
| 前端 | React 18 + TypeScript | UI 渲染 |
| 核心逻辑 | TypeScript (Node.js) | 文件扫描、解析、重命名、回滚 |
| 数据库 | better-sqlite3 | 任务/日志持久化 |
| 样式 | Tailwind CSS | 原子化 CSS |
| 状态管理 | Zustand | 轻量级状态管理 |
| 打包 | electron-builder | MSI/NSIS |

---

## 16 维度比较

| 维度 | 方案 A (Tauri) | 方案 B (Electron) |
|------|:---:|:---:|
| 1. 跨平台 | 9 | 10 |
| 2. 文件系统访问 | 10 | 9 |
| 3. 大批量稳定性 | 10 | 8 |
| 4. 回滚便利性 | 10 | 8 |
| 5. 安全风险 | 9 | 7 |
| 6. 性能/资源 | 10 | 6 |
| 7. 打包体积 | 10 | 5 |
| 8. 维护成本 | 8 | 9 |
| 9. 测试便利性 | 8 | 9 |
| 10. 扩展性 | 9 | 9 |
| 11. UI 开发效率 | 8 | 9 |
| 12. Windows 兼容性 | 9 | 10 |
| 13. 自动更新 | 8 | 9 |
| 14. 产品化能力 | 9 | 8 |
| 15. UI 美观空间 | 9 | 9 |
| 16. 家谱树维护 | 9 | 8 |
| **总分** | **145** | **135** |

---

## 各维度分析

### 1. 跨平台能力
- Tauri: Windows/macOS/Linux，WebView 随系统
- Electron: Windows/macOS/Linux，自带 Chromium

### 2. 文件系统访问
- Tauri: Rust 原生 fs，性能优异，安全沙箱可控
- Electron: Node.js fs，足够用但性能不如 Rust

### 3. 大批量扫描稳定性
- Tauri: Rust 并发扫描 10,000+ 文件无压力
- Electron: Node.js 单线程需 worker_threads，大数量时可能卡顿

### 4. 回滚便利性
- Tauri: Rust 原子操作，事务安全
- Electron: Node.js 也可靠，但 Rust 更健壮

### 5. 安全风险
- Tauri: 最小权限原则，API 白名单
- Electron: Node.js 全权限，需额外安全措施

### 6. 性能/资源占用
- Tauri: 内存 ~30MB，CPU 低
- Electron: 内存 ~150MB+，CPU 较高

### 7. 打包体积
- Tauri: ~3-8MB
- Electron: ~80-150MB

### 8. 长期维护成本
- Tauri: Rust + TS 双栈，Rust 人才稀缺
- Electron: 全 JS/TS，人才充足

### 9. 测试便利性
- Tauri: Rust 单测 + 前端单测，需分层测试
- Electron: 全 JS/TS，测试工具链成熟

### 10. 扩展性（TMDb/TVDB）
- Tauri: Rust HTTP client + 前端 fetch
- Electron: Node.js HTTP，更灵活

### 11. UI 开发效率
- Tauri: React 开发体验一致
- Electron: React 开发体验一致

### 12. Windows 兼容性
- Tauri: WebView2 需 Win10+，符合 MVP 要求
- Electron: Chromium 全兼容

### 13. 自动更新
- Tauri: 内置 updater 插件
- Electron: electron-updater 成熟

### 14. 产品化能力
- Tauri: 更现代，体积小，启动快
- Electron: 生态成熟，案例多

### 15. UI 美观空间
- Tauri: WebView 渲染，CSS 完全控制
- Electron: Chromium 渲染，CSS 完全控制

### 16. 家谱树维护
- Tauri: Rust 强类型，边界清晰
- Electron: TS 也支持，但弱于 Rust

---

## 三轮审查结论

### 第 1 轮：初步比较
- Tauri 在性能、安全、体积方面优势明显
- Electron 在人才、维护、生态方面优势明显

### 第 2 轮：稳定性/维护性/风险批判
- Tauri 的 Rust 核心对文件操作更可靠
- Electron 的全 JS 栈降低开发门槛
- Tauri 的 WebView2 依赖可能在旧系统上有兼容问题

### 第 3 轮：收敛
- **推荐方案 A：Tauri v2 + React + TypeScript + Rust Core**
- 用户倾向：若无明显反转，优先 Tauri
- 结论：Tauri 在性能、安全、体积方面优势显著，符合专业桌面工具定位

---

*下一阶段：scoring-matrix.md*
