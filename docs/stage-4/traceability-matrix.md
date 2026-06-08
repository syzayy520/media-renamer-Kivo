# 可追溯性矩阵

> 阶段 4：任务分解与执行计划
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 需求 → 任务映射

### F-001 媒体扫描

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-001-01 | 扫描指定目录及子目录 | T-RUST-003, T-UI-004 |
| F-001-02 | 识别 13 种视频文件扩展名 | T-RUST-003 |
| F-001-03 | 显示扫描进度 | T-RUST-010, T-UI-004 |
| F-001-04 | 支持取消扫描 | T-RUST-003, T-UI-004 |
| F-001-05 | 记录权限失败 | T-RUST-003 |
| F-001-06 | 最大文件数量保护 | T-RUST-003 |
| F-001-07 | 符号链接检测 | T-RUST-003 |
| F-001-08 | 循环路径检测 | T-RUST-003 |
| F-001-09 | 网络路径断连处理 | T-RUST-003 |
| F-001-10 | 伴随文件识别 | T-RUST-003 |

### F-002 文件名解析

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-002-01 | 提取标题 | T-RUST-004 |
| F-002-02 | 提取年份 | T-RUST-004 |
| F-002-03 | 提取季号 | T-RUST-004 |
| F-002-04 | 提取集号 | T-RUST-004 |
| F-002-05 | 提取多集标识 | T-RUST-004 |
| F-002-06 | 提取分辨率 | T-RUST-004 |
| F-002-07 | 提取来源 | T-RUST-004 |
| F-002-08 | 提取视频编码 | T-RUST-004 |
| F-002-09 | 提取音频编码 | T-RUST-004 |
| F-002-10 | 提取编码组名 | T-RUST-004 |
| F-002-11 | 提取特殊标记 | T-RUST-004 |
| F-002-12 | 置信度评分 | T-RUST-004 |
| F-002-13 | 电影名不含年份低置信度 | T-RUST-004 |
| F-002-14 | 无法区分类型低置信度 | T-RUST-004 |
| F-002-15 | 规则命中来源 | T-RUST-004 |

### F-003 分类识别

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-003-01 | 识别电影 | T-RUST-004 |
| F-003-02 | 识别电视剧 | T-RUST-004 |
| F-003-03 | 识别动漫 | T-RUST-004 |
| F-003-04 | 识别特别篇/SP | T-RUST-004 |
| F-003-05 | 识别 OVA | T-RUST-004 |
| F-003-06 | 识别 NCOP/NCED | T-RUST-004 |
| F-003-07 | 识别 Extras | T-RUST-004 |
| F-003-08 | 标记未识别 | T-RUST-004 |
| F-003-09 | 动漫与剧集边界判定 | T-RUST-004 |

### F-004 重命名预览

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-004-01 | 按模板生成新文件名 | T-RUST-005 |
| F-004-02 | 展示原路径 vs 新名称对比 | T-UI-005, T-UI-003 |
| F-004-03 | 展示目标路径 | T-UI-005 |
| F-004-04 | 展示推断类型 | T-UI-005 |
| F-004-05 | 展示置信度 | T-UI-005, T-UI-003 |
| F-004-06 | 展示规则命中来源 | T-UI-005 |
| F-004-07 | 展示风险标记 | T-UI-005, T-UI-003 |
| F-004-08 | 标记需人工确认 | T-RUST-005, T-UI-005 |
| F-004-09 | 标记是否跳过 | T-UI-005 |
| F-004-10 | 展示冲突原因 | T-RUST-005, T-UI-005 |
| F-004-11 | 批量选中/取消 | T-UI-005 |
| F-004-12 | 搜索/筛选/排序 | T-UI-005, T-UI-002 |

### F-005 低置信度标记

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-005-01 | 置信度 < 阈值标记 | T-RUST-004, T-RUST-005 |
| F-005-02 | 低置信度默认不执行 | T-RUST-005 |
| F-005-03 | 必须进入人工确认 | T-UI-006 |
| F-005-04 | 可修改类型/标题/年份/季集 | T-UI-006 |
| F-005-05 | 可标记跳过 | T-UI-006 |
| F-005-06 | 确认后置信度更新 | T-RUST-004 |

### F-006 冲突检测

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-006-01 | 目标路径已存在 | T-RUST-005 |
| F-006-02 | 多源指向同一目标 | T-RUST-005 |
| F-006-03 | 路径长度超 260 | T-RUST-001, T-RUST-005 |
| F-006-04 | 非法字符 | T-RUST-001, T-RUST-005 |
| F-006-05 | 权限检查 | T-RUST-005 |
| F-006-06 | 源文件存在性 | T-RUST-005 |
| F-006-07 | 冲突项阻断执行 | T-RUST-005 |
| F-006-08 | 冲突原因展示 | T-UI-005 |
| F-006-09 | 不自动覆盖/追加 | T-RUST-005 |

### F-007 安全执行

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-007-01 | 默认 dry-run | T-RUST-005, T-UI-007 |
| F-007-02 | 执行前展示预览 | T-UI-005, T-UI-007 |
| F-007-03 | 二次确认 | T-UI-007 |
| F-007-04 | 10 项安全检查 | T-RUST-005 |
| F-007-05 | 低置信度不自动执行 | T-RUST-005 |
| F-007-06 | 冲突项不自动执行 | T-RUST-005 |
| F-007-07 | 执行进度展示 | T-RUST-010, T-UI-007 |
| F-007-08 | 执行结果统计 | T-UI-007 |
| F-007-09 | 记录 before/after | T-RUST-006 |
| F-007-10 | 部分失败继续 | T-RUST-005 |

### F-008 审计日志

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-008-01 | 记录 RenameTask | T-RUST-006 |
| F-008-02 | 记录 RenameResult | T-RUST-006 |
| F-008-03 | 包含 before/after/status/error | T-RUST-006 |
| F-008-04 | 包含规则命中来源 | T-RUST-006 |
| F-008-05 | 包含执行时间、任务 ID | T-RUST-006 |
| F-008-06 | JSONL 导出 | T-RUST-006, T-UI-008 |
| F-008-07 | 持久化到本地数据库 | T-RUST-009 |

### F-009 任务回滚

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-009-01 | 按任务回滚 | T-RUST-007 |
| F-009-02 | 基于 before/after 映射 | T-RUST-007 |
| F-009-03 | 回滚前检测 afterPath | T-RUST-007 |
| F-009-04 | 回滚前检测 beforePath | T-RUST-007 |
| F-009-05 | 冲突时暂停报告 | T-RUST-007 |
| F-009-06 | 支持部分回滚 | T-RUST-007 |
| F-009-07 | 回滚记录审计 | T-RUST-006, T-RUST-007 |
| F-009-08 | 回滚结果详细报告 | T-UI-008 |

### F-010 崩溃恢复

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-010-01 | 改名前持久化状态 | T-RUST-012 |
| F-010-02 | 每改一个立即更新 | T-RUST-005, T-RUST-012 |
| F-010-03 | 启动时检查未完成 | T-RUST-012 |
| F-010-04 | 从审计日志恢复 | T-RUST-006, T-RUST-012 |

### F-011 人工确认

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-011-01 | 低置信度审核 | T-UI-006 |
| F-011-02 | 修改解析结果 | T-UI-006 |
| F-011-03 | 标记跳过 | T-UI-006 |

### F-012 命名模板

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-012-01 | 电影默认模板 | T-RUST-002 |
| F-012-02 | 剧集默认模板 | T-RUST-002 |
| F-012-03 | 动漫默认模板 | T-RUST-002 |
| F-012-04 | 特别篇默认模板 | T-RUST-002 |
| F-012-05 | Extras 默认模板 | T-RUST-002 |
| F-012-06 | 多集格式模板 | T-RUST-002 |
| F-012-07 | 模板可配置 | T-UI-009 |

### F-METADATA TMDb API Key 配置

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| F-METADATA-001-01 | TMDb API Key 输入入口 | T-UI-011 |
| F-METADATA-001-02 | 保存 TMDb API Key | T-RUST-002a, T-UI-011 |
| F-METADATA-001-03 | 更新 TMDb API Key | T-RUST-002a, T-UI-011 |
| F-METADATA-001-04 | 清除 TMDb API Key | T-RUST-002a, T-UI-011 |
| F-METADATA-001-05 | UI 遮罩显示 | T-UI-011 |
| F-METADATA-001-06 | 测试连接入口 | T-RUST-013, T-UI-011 |
| F-METADATA-001-07 | 未配置时提示 | T-UI-011 |
| F-METADATA-001-08 | 配置后可使用 | T-RUST-013 |
| F-METADATA-001-09 | 可选增强，不阻塞主流程 | T-RUST-013, T-UI-011 |
| F-METADATA-001-10 | 本地安全保存 | T-RUST-002a |
| F-METADATA-001-11 | 禁止硬编码 | T-RUST-002a |
| F-METADATA-001-12 | 禁止写进源码 | T-RUST-002a |
| F-METADATA-001-13 | 禁止提交 Git | T-RUST-002a |
| F-METADATA-001-14 | 禁止输出到日志 | T-RUST-002a |
| F-METADATA-001-15 | 禁止明文长期展示 | T-UI-011 |
| F-METADATA-001-16 | 禁止作为必需条件 | T-RUST-013 |
| F-METADATA-001-17 | 无 Key 时功能可用 | T-RUST-013 |
| F-METADATA-001-18 | 可选元数据增强 | T-RUST-013 |
| F-METADATA-001-19 | 结果显示来源、置信度 | T-RUST-013, T-UI-011 |
| F-METADATA-001-20 | 不得静默覆盖 | T-RUST-013 |

---

## 非功能需求 → 任务映射

| 需求 ID | 需求描述 | 任务 ID |
|---------|----------|---------|
| NF-001 | 10,000 文件稳定 | T-RUST-003, T-TEST-004 |
| NF-002 | 架构目标 100,000 | T-RUST-003 |
| NF-003 | UI 不卡顿 | T-UI-003, T-TEST-004 |
| NF-004 | 启动 <3s | T-PKG-002 |
| NF-005 | 内存 <500MB | T-TEST-004 |
| NF-006 | 100% 审计 | T-RUST-006 |
| NF-007 | 100% 可回滚 | T-RUST-007 |
| NF-008 | 默认 dry-run | T-RUST-005, T-UI-007 |
| NF-009 | 整树家谱 | T-INF-004 |
| NF-010 | 测试覆盖 | T-TEST-001~005 |

---

## ADR → 任务映射

| ADR | 决策 | 任务 ID |
|-----|------|---------|
| ADR-001 | Tauri v2 | T-INF-001 |
| ADR-002 | React + TS | T-INF-001, T-INF-003 |
| ADR-003 | Tailwind CSS | T-INF-005 |
| ADR-004 | Zustand | T-UI-002 |
| ADR-005 | SQLite | T-RUST-009 |
| ADR-006 | Rust 核心库 | T-INF-002 |
| ADR-007 | Tauri 插件 | T-INF-001 |
| ADR-008 | MSI + NSIS | T-PKG-002 |
| ADR-009 | 测试框架 | T-TEST-001~003 |
| ADR-010 | 项目结构 | T-INF-004 |
| ADR-METADATA-001 | TMDb 可选 Provider | T-RUST-013 |
| ADR-SECRET-001 | API Key 安全存储 | T-RUST-002a |

---

## 风险 → 缓解任务映射

| 风险 ID | 风险描述 | 缓解任务 |
|---------|----------|----------|
| R-SCAN-001 | 大目录扫描性能 | T-RUST-003, T-TEST-004 |
| R-SCAN-002 | 网络路径断连 | T-RUST-003 |
| R-PARSE-001 | 文件名解析准确率 | T-RUST-004, T-TEST-001, T-TEST-005 |
| R-RENAME-001 | 误改名 | T-RUST-005, T-UI-007 |
| R-RENAME-002 | 回滚失败 | T-RUST-007 |
| R-RENAME-003 | 路径过长 | T-RUST-001, T-RUST-005 |
| R-UI-001 | 10,000 行卡顿 | T-UI-003 |
| R-UI-002 | 路径对比不清晰 | T-UI-003 |
| R-UI-003 | 100K 架构目标 | T-RUST-003 |
| R-STRUCT-001 | 家谱腐化 | T-INF-004 |

---

## 测试覆盖矩阵

| 测试类型 | 覆盖模块 | 任务 ID | 状态 |
|----------|----------|---------|------|
| Rust 单测 | shared | T-TEST-001 | |
| Rust 单测 | config | T-TEST-001 | |
| Rust 单测 | config/secret | T-TEST-001 | |
| Rust 单测 | config/user_settings | T-TEST-001 | |
| Rust 单测 | scan | T-TEST-001 | |
| Rust 单测 | parse (53 tests) | T-TEST-001 | ✅ 已完成 |
| Rust 单测 | rename (55 tests) | T-TEST-001 | ✅ 已完成 |
| Rust 单测 | rollback (16 tests) | T-TEST-001 | ✅ 已完成 |
| Rust 单测 | audit (44 tests) | T-TEST-001 | ✅ 已完成 |
| Rust 单测 | metadata | T-TEST-001 |
| Rust 单测 | metadata/provider | T-TEST-001 |
| Rust 单测 | metadata/tmdb | T-TEST-001 |
| 前端单测 | 组件 | T-TEST-002 |
| 前端单测 | Store | T-TEST-002 |
| 前端单测 | 流程 | T-TEST-002 |
| 前端单测 | metadataStore | T-TEST-002 |
| 集成测试 | IPC 命令 | T-TEST-003 |
| 集成测试 | 端到端流程 | T-TEST-003 |
| 集成测试 | TMDb API Key 配置 | T-TEST-003 |
| 性能测试 | 大批量 | T-TEST-004 |
| Fixture | 测试数据 | T-TEST-005 |
| 安全测试 | API Key 安全存储 | T-TEST-001 |
| 安全测试 | 数据脱敏 | T-TEST-001 |

---

## Parse 模块测试映射（Stage 5 补充）

> 日期：2026-06-08
> 验证：cargo test 74/74 PASS, clippy PASS

### F-002 文件名解析 → T-RUST-004 → 单元测试映射

| 需求 ID | 解析器 | 测试用例 | 覆盖场景 |
|---------|--------|----------|----------|
| F-002-01 | movie_parser | test_parse_movie_basic | 标题提取 |
| F-002-02 | movie_parser | test_parse_movie_basic | 年份提取 |
| F-002-06 | movie_parser | test_parse_movie_basic | 分辨率提取 |
| F-002-07 | movie_parser | test_parse_movie_basic | 来源提取 |
| F-002-08 | movie_parser | test_parse_movie_h264 | 视频编码提取 |
| F-002-09 | movie_parser | test_parse_movie_basic | 音频编码提取 |
| F-002-01 | series_parser | test_parse_series_basic | 标题提取 |
| F-002-03 | series_parser | test_parse_series_basic | 季号提取 |
| F-002-04 | series_parser | test_parse_series_basic | 集号提取 |
| F-002-05 | series_parser | test_parse_series_multi_episode | 多集标识 |
| F-002-10 | anime_parser | test_parse_anime_group_with_codec | 编码组提取 |
| F-002-11 | special_parser | test_parse_special_ncop | NCOP 特殊标记 |
| F-002-12 | confidence | test_evaluate_movie_with_year | 置信度评分 |
| F-002-13 | confidence | test_evaluate_movie_no_year | 电影无年份低置信度 |
| F-002-15 | confidence | test_evidence_contains_rule_name_and_delta | 规则命中来源 |

### F-003 分类识别 → T-RUST-004 → 单元测试映射

| 需求 ID | 分类器/解析器 | 测试用例 | 覆盖场景 |
|---------|--------------|----------|----------|
| F-003-01 | classifier | test_classify_movie / test_classify_and_parse_movie | 电影识别 |
| F-003-02 | classifier | test_classify_series / test_classify_and_parse_series | 剧集识别 |
| F-003-03 | classifier | test_classify_anime_with_group / test_classify_and_parse_anime | 动漫识别 |
| F-003-04 | classifier | test_classify_sp / test_classify_s00_special | 特别篇/SP 识别 |
| F-003-05 | classifier | test_classify_ova | OVA 识别 |
| F-003-06 | classifier | test_classify_ncop_nced | NCOP/NCED 识别 |
| F-003-07 | classifier | test_classify_extras | Extras 识别 |
| F-003-08 | classifier | test_classify_unknown / test_classify_and_parse_unknown | 未识别标记 |
| F-003-09 | anime_parser | test_parse_anime_no_match (S01E01 excluded) | 动漫与剧集边界 |

### 新增领域对象测试映射

| 领域对象 | 测试用例 | 说明 |
|----------|----------|------|
| RuleMatchEvidence | test_evidence_contains_rule_name_and_delta | 证据链完整性 |
| ConfidenceResult | test_evaluate_movie_with_year | score + evidences + needs_review |

---

## Rename 模块测试映射（Safety Core Round 3）

> 日期：2026-06-08
> 验证：cargo test 202/202 PASS, clippy PASS

### F-004 重命名预览 → T-RUST-005 → 单元测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-004-01 | template | test_render_movie_template | 电影模板渲染 |
| F-004-01 | template | test_render_series_template | 剧集模板渲染 |
| F-004-01 | template | test_render_anime_template | 动漫模板渲染 |
| F-004-01 | template | test_render_special_template | 特别篇模板渲染 |
| F-004-01 | template | test_render_empty_fields | 空字段处理 |
| F-004-01 | template | test_render_unknown_variables | 未知变量保持原样 |
| F-004-01 | template | test_get_default_template_movie | 默认电影模板 |
| F-004-01 | template | test_get_default_template_series | 默认剧集模板 |
| F-004-01 | template | test_get_default_template_anime | 默认动漫模板 |
| F-004-01 | template | test_get_default_template_extras | 默认 Extras 模板 |
| F-004-01 | template | test_render_idempotency | 幂等性 |
| F-004-01 | template | test_render_illegal_chars_sanitization | 非法字符处理 |
| F-004-08 | preview_generator | test_generate_low_confidence_marks_manual_review | 低置信度标记人工确认 |
| F-004-10 | preview_generator | test_generate_conflict_items_marked | 冲突项标记 |

### F-006 冲突检测 → T-RUST-005 → 集成测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-006-01 | conflict_detector | test_detect_target_exists | 目标路径已存在 |
| F-006-02 | conflict_detector | test_detect_duplicate_target | 多源指向同一目标 |
| F-006-02 | conflict_detector | test_detect_case_conflict | 大小写冲突 |
| F-006-03 | conflict_detector | test_detect_path_too_long | 路径过长 |
| F-006-06 | conflict_detector | test_detect_source_not_found | 源文件不存在 |
| F-006-07 | conflict_detector | test_has_blocking_conflicts | 阻塞冲突判断 |
| F-006-07 | conflict_detector | test_detect_multi_conflict_aggregation | 多冲突聚合 |
| F-006-09 | conflict_detector | test_detect_no_conflict | 无冲突 |
| F-006-01 | path_exists_checker | test_path_exists_detected | 目标路径存在检测 |
| F-006-01 | path_exists_checker | test_path_not_exists_ok | 路径不存在通过 |
| F-006-03 | path_length_checker | test_path_too_long_detected | 路径过长检测 |
| F-006-03 | path_length_checker | test_path_length_ok | 路径长度正常 |
| F-006-04 | invalid_chars_checker | test_invalid_chars_detected | 非法字符检测 |
| F-006-04 | invalid_chars_checker | test_valid_path_ok | 合法路径通过 |
| F-006-02 | duplicate_target_checker | test_duplicate_target_detected | 重复目标检测 |
| F-006-02 | duplicate_target_checker | test_no_duplicate | 无重复通过 |
| F-006-02 | case_conflict_checker | test_case_conflict_detected | 大小写冲突检测 |
| F-006-02 | case_conflict_checker | test_no_case_conflict | 无冲突通过 |

### F-007 安全执行 → T-RUST-005 → 单元测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-007-01 | safety_checker | test_check_all_safe_items | 默认 dry-run |
| F-007-04 | safety_checker | test_check_all_safe_items_pass | 安全项通过 |
| F-007-05 | safety_checker | test_check_all_low_confidence_blocks | 低置信度阻塞 |
| F-007-06 | safety_checker | test_check_all_conflict_blocks | 冲突阻塞 |
| F-007-04 | safety_checker | test_check_all_manual_review_blocks | 人工确认阻塞 |
| F-007-04 | safety_checker | test_check_all_invalid_chars_blocks | 非法字符阻塞 |
| F-007-04 | safety_checker | test_check_all_path_too_long_blocks | 路径过长阻塞 |
| F-007-04 | safety_checker | test_check_single_safe | 单项检查通过 |
| F-007-04 | safety_checker | test_check_single_low_confidence | 单项低置信度 |

### 预览生成测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-004-01 | preview_generator | test_generate_movie_preview | 电影预览 |
| F-004-01 | preview_generator | test_generate_series_preview | 剧集预览 |
| F-004-01 | preview_generator | test_generate_anime_preview | 动漫预览 |
| F-004-01 | preview_generator | test_generate_special_preview | 特别篇预览 |
| F-004-01 | preview_generator | test_generate_with_default_template | 默认模板预览 |
| F-004-01 | preview_generator | test_sanitize_proposed_name | 文件名清理 |

### 新增领域对象测试映射（Safety Core Round 1）

| 领域对象 | 测试用例 | 说明 |
|----------|----------|------|
| RenameConflict | test_detect_target_exists | conflict_type + source_path + target_path + blocking |
| MetadataSource | test_render_movie_template | LocalRule 来源 |
| SafetyReport | test_check_all_safe_items | can_execute + dry_run + checks + blocking_reasons |
| SafetyCheck | test_check_single_safe | name + passed + message |

---

## Audit 模块测试映射（整树家谱重构）

> 日期：2026-06-08
> 验证：cargo test 178/178 PASS, clippy PASS

### F-008 审计日志 → T-RUST-006 → 单元测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-008-07 | db/connection | test_memory_connection | 内存连接创建 |
| F-008-07 | db/connection | test_connection_open | 文件连接创建 |
| F-008-07 | db/schema | test_init_tables | 三表创建 |
| F-008-07 | db/schema | test_tables_exist | 表存在检查 |
| F-008-07 | db/schema | test_repeated_init_is_idempotent | 幂等初始化 |
| F-008-05 | db/task_status | test_status_display | TaskStatus 显示格式 |
| F-008-05 | db/task_status | test_status_parse | TaskStatus 解析 |
| F-008-05 | db/task_status | test_invalid_status_handling | 无效状态处理 |
| F-008-05 | db/task_status | test_parse_task_status_valid | parse_task_status 兼容 |
| F-008-01 | db/task_repository | test_insert_task | 任务插入 |
| F-008-01 | db/task_repository | test_get_task | 任务查询 |
| F-008-05 | db/task_repository | test_update_task_status | 任务状态更新 |
| F-008-05 | db/task_repository | test_failed_status_record | 失败状态记录 |
| F-008-01 | db/task_repository | test_get_nonexistent_task | 不存在任务查询 |
| F-008-02 | db/result_repository | test_insert_result | 结果插入 |
| F-008-02 | db/result_repository | test_get_results_by_task | 按任务查询结果 |
| F-008-02 | db/result_repository | test_get_results_empty | 空结果查询 |
| F-008-02 | db/result_repository | test_multiple_results | 多结果查询 |
| F-008-03 | db/log_repository | test_insert_log | 日志插入 |
| F-008-05 | db/log_repository | test_get_logs_by_task | 按任务查询日志 |
| F-008-05 | db/log_repository | test_get_all_logs | 全量日志查询 |
| F-008-05 | db/log_repository | test_empty_logs | 空日志查询 |
| F-008-05 | db/log_repository | test_get_logs_by_task_empty | 无结果日志查询 |
| F-008-06 | exporter | test_export_jsonl_format | JSONL 格式验证 |
| F-008-06 | exporter | test_export_multiple_logs | 多条日志导出 |
| F-008-06 | exporter | test_export_redacts_sensitive_data | 导出数据脱敏 |
| F-008-06 | exporter | test_export_empty_logs | 空日志处理 |
| F-008-06 | exporter | test_export_by_task | 按任务导出 |

### 敏感数据脱敏 → 单元测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| NF-006 | redaction | test_redact_tmdb_key | TMDb API Key 脱敏 |
| NF-006 | redaction | test_redact_api_key | 通用 API Key 脱敏 |
| NF-006 | redaction | test_redact_token | Token 脱敏 |
| NF-006 | redaction | test_redact_secret | Secret 脱敏 |
| NF-006 | redaction | test_redact_url_query | URL 查询参数脱敏 |
| NF-006 | redaction | test_redact_json_field | JSON 字段脱敏 |
| NF-006 | redaction | test_normal_text_not_redacted | 正常文本不误脱 |
| NF-006 | redaction | test_contains_sensitive_true/false | 敏感内容检测 |
| NF-006 | logger | test_log_does_not_leak_key | 日志自动脱敏 |
| NF-006 | logger | test_log_event | 审计事件记录 |
| NF-006 | logger | test_log_failure | 失败事件记录 |
| NF-006 | logger | test_log_preview | 预览事件记录 |
| NF-006 | logger | test_log_task_created | 任务创建记录 |
| NF-006 | logger | test_log_error_handling | 错误处理 |

### 数据库初始化 → 单元测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-008-07 | db/connection | test_memory_connection | 内存连接 |
| F-008-07 | db/connection | test_connection_open | 文件连接 |
| F-008-07 | db/schema | test_init_tables | 三表创建 |
| F-008-07 | db/schema | test_tables_exist | 表存在检查 |
| F-008-07 | db/schema | test_repeated_init_is_idempotent | 幂等初始化 |

---

## Executor 模块测试映射（Safety Core Round 3）

> 日期：2026-06-08
> 验证：cargo test 202/202 PASS, clippy PASS

### F-007 安全执行 → T-RUST-005 → execution/ 集成测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-007-01 | executor_core | test_dry_run_does_not_modify_files | DryRun 模式不修改文件 |
| F-007-01 | executor_core | test_confirmed_rename_success | Confirmed 模式成功改名 |
| F-007-10 | executor_core | test_source_not_found_fails | 源文件不存在失败 |
| F-007-10 | executor_core | test_target_already_exists_fails | 目标已存在失败 |
| F-007-10 | executor_core | test_partial_failure_aggregation | 部分失败聚合 |
| F-007-06 | conflict_filter | test_conflict_item_blocked | 冲突项阻断执行 |
| F-007-05 | safety_gate | test_low_confidence_blocks_confirmed_mode | 低置信度阻断 Confirmed 模式 |
| F-008-02 | result_recorder | test_audit_result_recorded | 审计结果记录 |
| F-007-01 | skip_filter | test_skip_item_not_executed | 跳过项不执行 |

### 新增领域对象测试映射（Safety Core Round 3 - Executor）

| 领域对象 | 测试用例 | 说明 |
|----------|----------|------|
| ExecutionMode | test_default_is_dry_run | 默认值验证 |
| ExecutionMode | test_mode_equality | 模式相等性 |
| ExecutionMode | test_mode_clone | 模式克隆 |

---

## Rollback 模块测试映射（Safety Core Round 3）

> 日期：2026-06-08
> 验证：cargo test 202/202 PASS, clippy PASS

### F-009 任务回滚 → T-RUST-007 → 单元/集成测试映射

| 需求 ID | 模块 | 测试用例 | 覆盖场景 |
|---------|------|----------|----------|
| F-009-01 | state_checker | test_rollbackable_state | 可回滚状态检测（lib.rs） |
| F-009-03 | state_checker | test_afterpath_not_exist_blocks | afterPath 不存在阻断（lib.rs） |
| F-009-04 | state_checker | test_beforepath_occupied_blocks | beforePath 被占用阻断（lib.rs） |
| F-009-01 | state_checker | test_failed_rename_not_rollbackable | 失败任务不可回滚（lib.rs） |
| F-009-06 | state_checker | test_partial_rollbackable | 部分可回滚（lib.rs） |
| F-009-01 | rollback_single | test_single_file_rollback_success | 单文件回滚成功 |
| F-009-04 | rollback_single | test_beforepath_occupied_blocks | beforePath 被占用阻断 |
| F-009-03 | rollback_single | test_afterpath_missing_blocks | afterPath 不存在阻断 |
| F-009-07 | rollback_audit | test_rollback_writes_audit_log | 回滚写审计日志 |
| F-009-01 | rollback_core | test_multi_file_rollback_success | 多文件回滚成功 |
| F-009-05 | rollback_core | test_partial_failure_recorded | 部分失败记录 |
| F-009-09 | rollback_core | test_does_not_overwrite_existing_file | 不覆盖已存在文件 |
| F-009-01 | rollback_core | test_idempotent_already_rolled_back | 幂等性已回滚阻断 |

### 新增领域对象测试映射（Safety Core Round 3 - Rollback）

| 领域对象 | 测试用例 | 说明 |
|----------|----------|------|
| RollbackReport | test_rollbackable_state | can_rollback + rollbackable_results（lib.rs） |
| RollbackStatus | test_single_file_rollback_success | Success 状态 |
| RollbackStatus | test_partial_failure_recorded | Failed 状态 |
| RollbackStatus | test_beforepath_occupied_blocks | Blocked 状态 |
| RollbackStatus | test_rollback_status_equality | 枚举相等性 |
| RollbackEntry | test_rollback_entry_creation | 结构体创建 |
| RollbackSummary | test_summarize_rollback | 汇总计算 |

---

*阶段 4 完成，进入阶段 5 实施*
