# Stage 5 中途家谱审计报告

> 项目：media-renamer-Kivo
> 日期：2026-06-08
> 审计类型：中途收口审计 + 基础验证

---

## 一、文档/代码不一致修正

| 问题 | 修正 |
|------|------|
| ADR-002 写 "React 18" | 更新为 "React 19"（package.json 实际 ^19.2.6） |
| ADR-003 未标版本 | 更新为 "Tailwind CSS 4.x"（package.json 实际 ^4.3.0） |
| ADR-002 缺少构建工具信息 | 添加 "Vite 8.x"（package.json 实际 ^8.0.12） |
| ADR 汇总表版本过时 | 同步更新 React 19、Tailwind 4.x |
| UI 主题策略 | 确认一致：深色主题 MVP 默认，V1 通过 CSS 变量支持浅色 |

---

## 二、基础验证结果

### 2.1 前端验证

| 验证项 | 结果 | 详情 |
|--------|------|------|
| npm install | PASS | 48 packages，无错误 |
| tsc -b (TypeScript) | PASS | 0 errors, 0 warnings |
| eslint | PASS | 0 errors, 0 warnings |
| vite build | PASS | 构建成功，输出 dist/ |

### 2.2 Rust/Tauri 验证

| 验证项 | 结果 | 详情 |
|--------|------|------|
| cargo fmt --check | PASS | 格式符合规范 |
| cargo check | PASS | 0 errors, 0 warnings |
| cargo clippy | PASS | 0 errors, 0 warnings |
| cargo test | 34/37 PASS | 3 FAIL（regex 调优，见下方） |

### 2.3 测试失败分析

| 测试 | 原因 | 严重度 | 处理 |
|------|------|--------|------|
| test_parse_movie_basic | 正则未匹配 [BluRay] 方括号格式 | 低 | Stage 5 调优 |
| test_parse_series_basic | episode_title 只捕获 "P" 而非 "Pilot" | 低 | Stage 5 调优 |
| test_scan_with_video_files | is_media_file 匹配了 .txt 伴随之 | 低 | Stage 5 修复 |

---

## 三、当前源码树

### 3.1 Rust Core（src-tauri/src/）

```
src-tauri/src/
├── lib.rs                    # 库入口
├── main.rs                   # 应用入口
├── shared/                   # 共享模块 - 已实现
│   ├── mod.rs
│   ├── path_utils.rs         # 路径工具
│   ├── result_types.rs       # 结果类型
│   └── platform.rs           # 平台适配
├── scan/                     # 扫描功能族 - 已实现
│   ├── mod.rs
│   ├── scanner.rs            # 目录遍历器
│   ├── file_detector.rs      # 文件类型检测
│   └── progress.rs           # 扫描进度
├── parse/                    # 解析功能族 - 已实现
│   ├── mod.rs
│   ├── movie_parser.rs       # 电影解析器
│   ├── series_parser.rs      # 剧集解析器
│   ├── anime_parser.rs       # 动漫解析器
│   ├── special_parser.rs     # 特别篇解析器
│   ├── confidence.rs         # 置信度评分
│   └── classifier.rs         # 类型分类器
├── rename/                   # 重命名功能族 - STUB
│   └── mod.rs
├── rollback/                 # 回滚功能族 - STUB
│   └── mod.rs
├── audit/                    # 审计功能族 - STUB
│   └── mod.rs
└── config/                   # 配置功能族 - STUB
    └── mod.rs
```

### 3.2 Frontend（src/）

```
src/
├── main.tsx                  # 应用入口
├── App.tsx                   # 根组件
├── App.css / index.css       # 样式
├── app/                      # 应用壳族 - 空
├── assets/                   # 静态资源
├── components/               # 组件族 - 空
├── design-system/            # 设计系统族 - 空
├── flows/                    # 流程族 - 空
├── pages/                    # 页面族 - 空
└── state/                    # 状态族 - 空
```

---

## 四、家谱合规审计

| 检查项 | 结果 |
|--------|------|
| 文件夹 = 功能族 | PASS |
| 子文件夹 = 子功能族 | PASS |
| 单文件 = 最小职责 | PASS |
| 入口文件 ≤ 30 行 | PASS |
| 禁止万能 utils | PASS |
| 禁止 manager/glue/facade | PASS |
| 禁止宽 mod.rs | PASS |
| 测试镜像模块树 | PASS |

---

## 五、Stage 5 继续判定

### 判定：CONDITIONAL - 仅允许修复失败测试

**Stage 5 may continue ONLY for fixing failing tests and stabilizing the current baseline.**
**New feature expansion is BLOCKED until cargo test reaches 37/37 PASS.**

依据：
1. 前端基础验证通过（4/4）
2. Rust 编译验证通过（fmt、check、clippy）
3. 但 cargo test 存在 3 个失败：34/37 PASS
4. 家谱结构完全合规，零损坏风险
5. 已实现模块质量达标
6. STUB 模块是预期状态

**阻塞条件：**
- 不得继续扩展 TMDb 实现
- 不得继续写大 UI
- 不得继续写 rollback/rename 大模块
- 只允许修复当前失败测试和相关最小实现

---

*审计完成。Stage 5 进入 Test Fix Gate。*
