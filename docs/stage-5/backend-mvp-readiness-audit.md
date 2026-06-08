# Backend MVP Readiness Audit

**Date**: 2026-06-09  
**Branch**: backend-continuation  
**HEAD**: 9f24eed test(commands): add command API contract tests  

---

## 1. Audit Scope

对当前后端 MVP 链路做最终就绪审计，检查：

- scan → parse → preview → safety → audit → session command (Dry-run MVP)
- controlled execution core → safety gate → audit → rollback core (Real Rename MVP)

---

## 2. MVP Readiness Matrix

| Module | Status | Notes |
|--------|--------|-------|
| scan | ✅ READY | 稳定扫描 tempdir 媒体目录，有错误处理 |
| parse | ✅ READY | 覆盖 movie/series/anime/special/ova/ncop/nced/extras/unknown |
| preview | ✅ READY | 稳定生成 before/after，有冲突检测和置信度评估 |
| conflict detection | ✅ READY | 阻断冲突 (target exists/duplicate/invalid chars/path too long/case conflict) |
| safety checker | ✅ READY | 阻断低置信度/review required/conflict |
| audit | ✅ READY | 记录 task 和 logs，有任务状态管理 |
| session command | ✅ READY | 返回稳定 PipelineResult 结构，有 task_id 可追踪 |
| config | ✅ READY | 有默认值和持久化，支持模板管理、阈值配置 |
| rollback core | ✅ READY | 与 audit 记录一致，有任务状态检查 |
| real rename safety gate | ✅ READY | 有安全门控，默认 dry-run，Confirmed 模式需通过安全检查 |

---

## 3. Dry-run Backend MVP

**Status: ✅ READY**

Dry-run MVP 链路完整：

1. **scan** - 扫描目录获取 MediaItem 列表
2. **parse** - 逐文件分类 + 解析
3. **preview** - 模板渲染 + 冲突检测
4. **safety** - 安全检查
5. **audit** - 创建任务 + 写审计日志
6. **session command** - 返回 PipelineResult

所有模块已有测试覆盖，使用 tempdir/memory DB，无真实路径。

---

## 4. Real Rename MVP

**Status: CORE READY / EXPOSURE DEFERRED**

后端 core 已有：

- ✅ controlled execution core (`rename/execution/executor_core.rs`)
- ✅ safety gate (`rename/execution/safety_gate.rs`)
- ✅ audit consistency (结果记录到 audit DB)
- ✅ rollback core (`rollback/executor/rollback_core.rs`)

但 **未暴露 command**：

- ❌ 无 `execute_rename` command
- ❌ 无 `rollback_task` command

**Deferred 原因**：需要前端 UI 配合确认操作，当前 UI 冻结。

---

## 5. Rollback MVP

**Status: CORE READY / EXPOSURE DEFERRED**

后端 core 已有：

- ✅ rollback core (`rollback/executor/rollback_core.rs`)
- ✅ rollback checker (`rollback/executor/rollback_checker.rs`)
- ✅ rollback single (`rollback/executor/rollback_single.rs`)
- ✅ rollback audit (`rollback/executor/rollback_audit.rs`)
- ✅ rollback entry (`rollback/executor/rollback_entry.rs`)
- ✅ state checker (`rollback/state_checker.rs`)
- ✅ 测试覆盖 (11 tests)

但 **未暴露 command**：

- ❌ 无 `rollback_task` command

**Deferred 原因**：需要前端 UI 配合确认操作，当前 UI 冻结。

---

## 6. Command Exposure Status

### 已暴露 Commands (9)

| Command | 功能族 | 来源模块 |
|---------|--------|----------|
| `start_rename_session` | session | `session::plan_session` |
| `get_app_config` | config | `commands::config_commands` |
| `get_all_templates` | config | `commands::config_commands` |
| `set_template` | config | `commands::config_commands` |
| `get_confidence_threshold` | config | `commands::config_commands` |
| `set_confidence_threshold` | config | `commands::config_commands` |
| `get_task` | audit | `commands::audit_commands` |
| `get_all_tasks` | audit | `commands::audit_commands` |
| `get_audit_logs` | audit | `commands::audit_commands` |

### 未暴露但已有 Core 的能力

| 能力 | Core 状态 | Command 状态 | 原因 |
|------|-----------|--------------|------|
| execute_rename | ✅ READY | ❌ DEFERRED | 需要前端确认 UI |
| rollback_task | ✅ READY | ❌ DEFERRED | 需要前端确认 UI |

---

## 7. Deferred Items

| # | Item | Status | Reason |
|---|------|--------|--------|
| 1 | execute_rename command | DEFERRED | 需要前端确认 UI，当前 UI 冻结 |
| 2 | rollback_task command | DEFERRED | 需要前端确认 UI，当前 UI 冻结 |
| 3 | Real Rename UI | DEFERRED | 当前 UI 冻结 |
| 4 | Rollback UI | DEFERRED | 当前 UI 冻结 |

---

## 8. Risk Register

| # | Risk | Likelihood | Impact | Mitigation |
|---|------|------------|--------|------------|
| 1 | 真实 rename 操作可能覆盖文件 | Low | High | safety_gate + conflict_detection + dry-run default |
| 2 | rollback 可能失败 | Low | Medium | rollback_checker + rollback_single 状态检查 |
| 3 | 长路径可能导致错误 | Medium | Low | path_length_checker |
| 4 | 特殊字符可能导致错误 | Medium | Low | invalid_chars_checker |
| 5 | 低置信度可能误操作 | Low | High | confidence threshold + manual_review |

---

## 9. Verification Results

| Check | Result |
|-------|--------|
| `cargo fmt --check` | ✅ PASS |
| `cargo check` | ✅ PASS |
| `cargo test` | ✅ 296 tests PASS |
| `cargo clippy --all-targets -- -D warnings` | ✅ PASS |
| `git diff --check` | ✅ PASS (no changes) |

### Test Breakdown

| Test Binary | Count |
|-------------|-------|
| lib.rs (unit tests) | 215 |
| main.rs | 0 |
| command_contract_tests.rs | 35 |
| pipeline_hardening.rs | 8 |
| rename_conflict_detection.rs | 15 |
| rename_execution.rs | 12 |
| rollback_executor.rs | 11 |
| doc-tests | 0 |
| **Total** | **296** |

---

## 10. Summary

| MVP | Status |
|-----|--------|
| Dry-run Backend MVP | ✅ READY |
| Real Rename MVP | CORE READY / EXPOSURE DEFERRED |
| Rollback MVP | CORE READY / EXPOSURE DEFERRED |

**结论**：Dry-run Backend MVP 已就绪，可以继续前端 UI 开发。Real Rename 和 Rollback 的后端 core 已有，但需要前端 UI 配合才能暴露 command。
