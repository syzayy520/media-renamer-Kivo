# 架构决策记录 (ADR)

> 阶段 2：技术选型审查
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## ADR-001: 桌面框架选择

**状态**：已决定

**背景**：需要选择桌面应用框架，支持 Windows 10/11，处理 10,000+ 文件扫描和重命名。

**决策**：采用 Tauri v2

**理由**：
1. 性能：Rust 原生 + 轻量 WebView，内存 ~30MB
2. 安全：最小权限 API 沙箱
3. 体积：打包 3-8MB
4. 稳定性：Rust 并发扫描 10,000+ 文件
5. 回滚可靠性：Rust 原子操作
6. 用户倾向：若无明显反转，优先 Tauri

**后果**：
- 需要 Rust 开发能力
- WebView2 依赖 Win10+（符合 MVP 要求）
- 测试需分层（Rust 单测 + 前端单测）

**替代方案**：Electron（评分 8.30 vs Tauri 9.32）

---

## ADR-002: 前端框架选择

**状态**：已决定

**背景**：需要选择前端 UI 框架。

**决策**：采用 React 19 + TypeScript

**理由**：
1. Tauri 官方推荐
2. 生态成熟
3. TypeScript 类型安全
4. 社区支持

**构建工具**：Vite 8.x（快速开发服务器和构建工具）

**后果**：需要 React 开发经验

---

## ADR-003: 样式方案选择

**状态**：已决定

**背景**：需要选择 CSS 方案。

**决策**：采用 Tailwind CSS 4.x

**理由**：
1. 原子化 CSS，开发效率高
2. 与 React 配合良好
3. 主题系统支持
4. 生产构建优化

**后果**：需要学习 Tailwind 语法

---

## ADR-004: 状态管理选择

**状态**：已决定

**背景**：需要管理 UI 状态。

**决策**：采用 Zustand

**理由**：
1. 轻量级，API 简洁
2. TypeScript 支持好
3. 无需 Provider 包裹
4. 与 React 配合良好

**后果**：需要学习 Zustand API

---

## ADR-005: 数据库选择

**状态**：已决定

**背景**：需要持久化任务、日志、回滚记录。

**决策**：采用 SQLite (rusqlite)

**理由**：
1. 零配置嵌入式
2. 无需服务进程
3. 适合桌面应用
4. 生态成熟

**后果**：需要 Rust SQLite 绑定

---

## ADR-006: Rust 核心库选择

**状态**：已决定

**背景**：Rust 核心需要哪些依赖。

**决策**：

| 用途 | 库 |
|------|-----|
| 文件系统 | std::fs + tokio::fs |
| 正则解析 | regex |
| 路径处理 | std::path + camino |
| 错误处理 | anyhow + thiserror |
| 序列化 | serde + serde_json |
| SQLite | rusqlite |
| 日志 | tracing |
| 并发 | tokio |

**后果**：需要管理 Rust 依赖

---

## ADR-007: Tauri 插件选择

**状态**：已决定

**背景**：Tauri 需要哪些插件。

**决策**：

| 插件 | 用途 |
|------|------|
| tauri-plugin-dialog | 文件选择对话框 |
| tauri-plugin-shell | 外部链接 |
| tauri-plugin-updater | 自动更新（V1） |
| tauri-plugin-log | 日志 |

**后果**：需要配置插件

---

## ADR-008: 打包格式

**状态**：已决定

**背景**：需要选择打包格式。

**决策**：MSI + NSIS

**理由**：
1. MSI 适合企业部署
2. NSIS 适合个人用户
3. Tauri bundler 原生支持

**后果**：需要配置打包

---

## ADR-009: 测试框架

**状态**：已决定

**背景**：需要选择测试框架。

**决策**：

| 层 | 框架 |
|----|------|
| Rust 单测 | cargo test |
| Rust 集成测试 | cargo test |
| 前端单测 | Vitest |
| 前端集成 | Vitest + Testing Library |
| E2E | Playwright (V1) |

**后果**：需要配置测试环境

---

## ADR-010: 项目结构

**状态**：已决定

**背景**：需要定义项目目录结构。

**决策**：

```
media-renamer-Kivo/
  src-tauri/          # Rust 核心
    src/
      scan/           # 扫描功能族
      parse/          # 解析功能族
      rename/         # 重命名功能族
      rollback/       # 回滚功能族
      audit/          # 审计功能族
      config/         # 配置功能族
      metadata/       # 元数据功能族（TMDb 等）
      shared/         # 共享模块
    tests/
  src/                # 前端
    app/              # 应用壳
    pages/            # 页面族
    components/       # 组件族
    state/            # 状态族
    flows/            # 流程族
    design-system/    # 设计系统
  tests/
  docs/
  examples/
    fixtures/
      sandbox/
```

**后果**：需要维护整树家谱结构

---

## ADR-METADATA-001: TMDb 作为可选 MetadataProvider

**状态**：已决定

**背景**：需要决定 TMDb 在系统中的定位和依赖关系。

**决策**：TMDb 作为可选 MetadataProvider，不作为 MVP 主链路依赖。

**理由**：
1. 降低外部依赖风险
2. 保证本地功能完整性
3. 用户可选择是否启用
4. 符合渐进式增强原则

**后果**：
- TMDb 模块独立，不影响核心流程
- 需要设计清晰的 Provider 接口
- 需要处理无 Key 时的降级逻辑
- V1 可扩展其他 MetadataProvider

---

## ADR-SECRET-001: 用户 API Key 的本地安全存储策略

**状态**：已决定

**背景**：需要决定如何安全存储用户的 TMDb API Key。

**决策**：采用本地安全存储策略，具体措施包括：
1. 使用 SQLite 加密存储（或系统密钥库）
2. UI 遮罩显示
3. 禁止日志输出
4. 禁止 Git 提交
5. 禁止源码硬编码

**理由**：
1. 保护用户隐私
2. 符合安全最佳实践
3. 防止意外泄露
4. 满足合规要求

**后果**：
- 需要实现安全存储模块
- 需要在多个层面添加安全检查
- 需要更新 .gitignore 和安全策略
- 需要添加相关测试

---

## 决策汇总

| ADR | 决策 | 状态 |
|-----|------|:----:|
| ADR-001 | Tauri v2 | 已决定 |
| ADR-002 | React 19 + TypeScript | 已决定 |
| ADR-003 | Tailwind CSS 4.x | 已决定 |
| ADR-004 | Zustand | 已决定 |
| ADR-005 | SQLite (rusqlite) | 已决定 |
| ADR-006 | Rust 核心库 | 已决定 |
| ADR-007 | Tauri 插件 | 已决定 |
| ADR-008 | MSI + NSIS | 已决定 |
| ADR-009 | 测试框架 | 已决定 |
| ADR-010 | 项目结构 | 已决定 |
| ADR-METADATA-001 | TMDb 作为可选 MetadataProvider | 已决定 |
| ADR-SECRET-001 | 用户 API Key 的本地安全存储策略 | 已决定 |

---

*下一阶段：阶段 3 — 系统架构与 UI/UX 设计*
