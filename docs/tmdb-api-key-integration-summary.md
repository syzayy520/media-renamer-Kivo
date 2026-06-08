# TMDb API Key 配置能力集成总结

> 项目：media-renamer-Kivo
> 日期：2026-06-08
> 状态：已完成

---

## 一、集成概述

已成功将 TMDb 用户自定义 API Key 配置能力同步到项目文档、架构、任务、实现计划和 UI 设计中。

---

## 二、文档更新清单

### 2.1 需求文档 (`docs/stage-1/requirements.md`)

**新增需求**：
- F-METADATA-001：用户可在设置页配置 TMDb API Key
- F-METADATA-002：用户可保存、更新、清除 TMDb API Key
- F-METADATA-003：TMDb Key 必须本地安全保存
- F-METADATA-004：没有 Key 时不影响本地规则重命名
- F-METADATA-005：联网元数据只能作为可选增强来源
- F-METADATA-006：TMDb 匹配结果必须带来源、置信度、人工确认状态

**详细需求**（F-METADATA-001-01 到 F-METADATA-001-20）：
- 设置页提供 TMDb API Key 输入入口
- 用户可以保存、更新、清除 TMDb API Key
- UI 必须对 Key 做遮罩显示
- 必须有"测试连接 / 验证配置"入口
- 没有 Key 时，显示"未配置，可继续使用本地规则解析"
- 配置 Key 后，后续元数据模块可以使用该 Key
- TMDb 功能必须是可选增强，不得阻塞本地规则重命名主流程

**MVP 范围调整**：
- MVP 包含：TMDb API Key 配置基础功能
- V1 增强：TMDb 搜索、自动补全、海报预览、多候选匹配等

### 2.2 验收标准 (`docs/stage-1/acceptance.md`)

**新增安全验收标准**：
- A-SEC-007: TMDb API Key 不得出现在日志、审计日志、控制台、导出文件中
- A-SEC-008: TMDb API Key 不得被提交进仓库
- A-SEC-009: UI 中不得明文长期展示完整 TMDb API Key

**新增功能验收标准**：
- A-FUNC-015: TMDb API Key 配置入口（设置页）
- A-FUNC-016: TMDb API Key 保存/更新/清除功能
- A-FUNC-017: TMDb API Key 遮罩显示
- A-FUNC-018: TMDb API Key 测试连接入口
- A-FUNC-019: 未配置 TMDb Key 时显示提示信息
- A-FUNC-020: TMDb 功能不阻塞本地规则重命名主流程

**新增失败条件**：
- TMDb API Key 出现在日志、审计日志、控制台、导出文件中
- TMDb API Key 被提交进仓库
- 未配置 TMDb Key 时，本地扫描/预览/改名/回滚功能不可用
- 配置 TMDb Key 后，设置页无法显示已配置状态
- 清除 TMDb Key 后，系统继续使用旧 Key

### 2.3 架构决策记录 (`docs/stage-2/architecture-decision-record.md`)

**新增 ADR**：
- ADR-METADATA-001：TMDb 作为可选 MetadataProvider，不作为 MVP 主链路依赖
- ADR-SECRET-001：用户 API Key 的本地安全存储策略

### 2.4 模块树 (`docs/stage-3/module-tree.md`)

**新增 Rust 模块**：
```
src-tauri/src/
  config/
    secret/                    # 安全存储子族
      api_key_store.rs         # API Key 存储
      redaction.rs             # 数据脱敏
    user_settings/             # 用户设置子族
      metadata_settings.rs     # 元数据设置
  metadata/                    # 元数据功能族
    provider/                  # Provider 子族
      metadata_provider.rs     # Provider 接口
      metadata_query.rs        # 查询接口
      metadata_match.rs        # 匹配接口
    tmdb/                      # TMDb 子族
      tmdb_client.rs           # TMDb 客户端
      tmdb_config.rs           # TMDb 配置
      tmdb_error.rs            # TMDb 错误
      tmdb_mapper.rs           # TMDb 数据映射
    tests/                     # 测试子族
      provider_contract_tests.rs
      tmdb_config_tests.rs
```

**新增前端模块**：
```
src/
  components/
    api-key-input/             # API Key 输入组件
    metadata-source-badge/     # 元数据来源标签
  state/
    config-state/              # 配置状态
    metadata-state/            # 元数据状态
```

### 2.5 模块家谱 (`docs/stage-3/module-genealogy.md`)

**新增功能族**：
- 配置功能族子族：安全存储、数据脱敏、用户设置
- 元数据功能族：Provider 接口、TMDb 客户端、测试

**新增薄入口文件**：
- config/secret/mod.rs
- config/user_settings/mod.rs
- metadata/mod.rs
- metadata/provider/mod.rs
- metadata/tmdb/mod.rs
- metadata/tests/mod.rs

### 2.6 UI 状态模型 (`docs/stage-3/ui-state-model.md`)

**新增 metadataStore**：
- TMDb API Key 管理
- 连接状态管理
- 配置状态管理
- 遮罩显示逻辑

### 2.7 接口与契约 (`docs/stage-3/interfaces-and-contracts.md`)

**新增 Tauri IPC 命令**：
- get_tmdb_api_key：获取 TMDb API Key
- save_tmdb_api_key：保存 TMDb API Key
- clear_tmdb_api_key：清除 TMDb API Key
- test_tmdb_connection：测试 TMDb 连接

**新增 Rust 内部接口**：
- config/secret/api_key_store.rs 接口
- config/secret/redaction.rs 接口
- config/user_settings/metadata_settings.rs 接口
- metadata/provider/metadata_provider.rs 接口
- metadata/tmdb/tmdb_client.rs 接口

### 2.8 任务分解 (`docs/stage-4/tasks.md`)

**新增任务**：
- T-RUST-002a：config/secret/ 安全存储模块 (4h)
- T-RUST-002b：config/user_settings/ 用户设置模块 (2h)
- T-RUST-013：metadata/ 元数据模块 (8h)
- T-UI-011：TMDb API Key 配置页面 (4h)

**任务依赖更新**：
- T-RUST-002a 依赖 T-RUST-001
- T-RUST-002b 依赖 T-RUST-002a
- T-RUST-013 依赖 T-RUST-002a, T-RUST-002b
- T-UI-011 依赖 T-UI-002

**任务汇总更新**：
- Rust 核心任务：12 → 15 (+12h)
- UI 前端任务：10 → 11 (+4h)
- 总预估：136h → 152h (+16h)

### 2.9 可追溯性矩阵 (`docs/stage-4/traceability-matrix.md`)

**新增需求映射**：
- F-METADATA-001 到 F-METADATA-006 的完整映射

**新增 ADR 映射**：
- ADR-METADATA-001 → T-RUST-013
- ADR-SECRET-001 → T-RUST-002a

**新增测试覆盖**：
- config/secret 测试
- config/user_settings 测试
- metadata 测试
- metadata/provider 测试
- metadata/tmdb 测试
- metadataStore 测试
- TMDb API Key 配置集成测试
- API Key 安全存储测试
- 数据脱敏测试

---

## 三、TMDb API Key 基本原则

### 3.1 安全原则

1. TMDb API Key 必须由用户在设置页自行填写
2. 禁止硬编码任何 TMDb Key
3. 禁止把 TMDb Key 写进源码
4. 禁止把 TMDb Key 提交到 Git
5. 禁止把 TMDb Key 输出到日志、审计日志、错误日志、控制台
6. 禁止在 UI 明文长期展示完整 Key
7. 禁止把 TMDb Key 当作 MVP 核心功能的必需条件

### 3.2 功能原则

8. 没有 TMDb Key 时，软件仍必须可以完成本地扫描、解析、预览、改名、日志和回滚
9. TMDb 只能作为可选元数据增强能力
10. 所有联网元数据结果必须显示来源、置信度和可人工确认状态，不得静默覆盖本地解析结果

---

## 四、产品范围

### 4.1 MVP 必须支持

1. 设置页提供 TMDb API Key 输入入口
2. 用户可以保存、更新、清除 TMDb Key
3. UI 必须对 Key 做遮罩显示
4. 必须有"测试连接 / 验证配置"入口
5. 没有 Key 时，显示"未配置，可继续使用本地规则解析"
6. 配置 Key 后，后续元数据模块可以使用该 Key
7. TMDb 功能必须是可选增强，不得阻塞本地规则重命名主流程

### 4.2 MVP 基础配置闭环

- 保存 Key
- 清除 Key
- 遮罩显示
- 读取配置状态
- 测试连接可以先做成明确的接口和 UI 状态；如果实际联网查询暂未完成，必须标记为后续任务，不得假装已完成

### 4.3 V1 增强功能

- TMDb 搜索电影
- TMDb 搜索电视剧
- 自动补全年份、官方标题、季集信息
- 海报和简介预览
- 多候选匹配选择
- 中文 / 原文标题偏好
- TMDb 与本地解析结果冲突时人工确认

---

## 五、实现计划

### 5.1 Rust 核心任务

| 任务 ID | 任务描述 | 预估时间 | 依赖 |
|---------|----------|----------|------|
| T-RUST-002a | config/secret/ 安全存储模块 | 4h | T-RUST-001 |
| T-RUST-002b | config/user_settings/ 用户设置模块 | 2h | T-RUST-002a |
| T-RUST-013 | metadata/ 元数据模块 | 8h | T-RUST-002a, T-RUST-002b |

### 5.2 UI 前端任务

| 任务 ID | 任务描述 | 预估时间 | 依赖 |
|---------|----------|----------|------|
| T-UI-011 | TMDb API Key 配置页面 | 4h | T-UI-002 |

### 5.3 测试任务

| 测试类型 | 覆盖模块 | 任务 ID |
|----------|----------|---------|
| Rust 单测 | config/secret | T-TEST-001 |
| Rust 单测 | config/user_settings | T-TEST-001 |
| Rust 单测 | metadata | T-TEST-001 |
| Rust 单测 | metadata/provider | T-TEST-001 |
| Rust 单测 | metadata/tmdb | T-TEST-001 |
| 前端单测 | metadataStore | T-TEST-002 |
| 集成测试 | TMDb API Key 配置 | T-TEST-003 |
| 安全测试 | API Key 安全存储 | T-TEST-001 |
| 安全测试 | 数据脱敏 | T-TEST-001 |

---

## 六、UI 设计要点

### 6.1 设置页面 TMDb 配置区域

1. **API Key 输入**：
   - 密码输入框，支持显示/隐藏切换
   - 保存按钮
   - 清除按钮

2. **状态显示**：
   - 未配置状态：显示"未配置，可继续使用本地规则解析"
   - 已配置状态：显示遮罩后的 Key（如 `abcd****efgh`）
   - 测试中状态：显示加载动画
   - 连接成功状态：显示绿色成功图标
   - 连接失败状态：显示红色错误图标和错误信息

3. **测试连接**：
   - 测试连接按钮
   - 连接状态显示
   - 错误信息显示

### 6.2 组件设计

1. **ApiKeyInput 组件**：
   - 密码输入框
   - 显示/隐藏切换按钮
   - 保存/清除按钮
   - 状态指示器

2. **MetadataSourceBadge 组件**：
   - 来源标签（本地/TMDb）
   - 置信度指示器
   - 人工确认状态

---

## 七、安全措施

### 7.1 存储安全

- 使用 SQLite 加密存储或系统密钥库
- API Key 加密后存储
- 访问时解密，使用后立即清除内存中的明文

### 7.2 传输安全

- UI 遮罩显示
- IPC 传输时使用安全通道
- 日志中自动脱敏

### 7.3 代码安全

- .gitignore 添加 API Key 相关文件
- 代码审查检查硬编码 Key
- 安全测试验证无泄露

---

## 八、后续工作

### 8.1 立即实施

1. 实现 config/secret/ 安全存储模块
2. 实现 config/user_settings/ 用户设置模块
3. 实现 metadata/ 元数据模块基础架构
4. 实现 TMDb API Key 配置页面

### 8.2 后续迭代

1. 实现 TMDb 客户端完整功能
2. 实现 TMDb 搜索电影/电视剧
3. 实现 TMDb 数据映射和匹配
4. 实现 TMDb 与本地解析结果冲突处理

---

## 九、验证清单

- [ ] 设置页提供 TMDb API Key 输入入口
- [ ] 用户可以保存 TMDb API Key
- [ ] 用户可以更新 TMDb API Key
- [ ] 用户可以清除 TMDb API Key
- [ ] UI 对 Key 做遮罩显示
- [ ] 有"测试连接 / 验证配置"入口
- [ ] 没有 Key 时显示正确提示
- [ ] 配置 Key 后可使用元数据模块
- [ ] TMDb 功能不阻塞本地规则重命名
- [ ] TMDb API Key 本地安全保存
- [ ] 禁止硬编码任何 TMDb Key
- [ ] 禁止把 TMDb Key 写进源码
- [ ] 禁止把 TMDb Key 提交到 Git
- [ ] 禁止把 TMDb Key 输出到日志
- [ ] 禁止在 UI 明文长期展示完整 Key
- [ ] 未配置 Key 时本地功能可用
- [ ] 联网元数据结果显示来源、置信度
- [ ] 不得静默覆盖本地解析结果

---

*文档版本：1.0*
*最后更新：2026-06-08*