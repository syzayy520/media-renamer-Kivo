# Backend Release Safety Gate Audit

**Date**: 2026-06-09  
**Branch**: backend-continuation  
**HEAD**: f9fdd7a docs(backend): add MVP readiness audit  

---

## 1. Audit Scope

在不暴露 real rename command、不修改 UI 的前提下，对后端 release safety gate 做最终审计，确认 dry-run MVP 可以安全交给前端接入。

---

## 2. 真实文件操作入口清单

| # | 入口 | 文件 | 操作 | 当前状态 |
|---|------|------|------|----------|
| 1 | config write | `config/config_loader.rs` | `std::fs::write` | ✅ 可达，但只写配置文件 |
| 2 | api_key write | `config/secret/api_key_store.rs` | `std::fs::write` | ✅ 可达，但只写 API key |
| 3 | create dir | `commands/config_commands.rs` | `std::fs::create_dir_all` | ✅ 可达，但只创建配置目录 |
| 4 | create dir | `lib.rs` | `std::fs::create_dir_all` | ✅ 可达，但只创建应用数据目录 |
| 5 | rename | `rename/execution/single_rename.rs` | `std::fs::rename` | ❌ 不可达（无 command 暴露） |
| 6 | rollback | `rollback/executor/rollback_single.rs` | `std::fs::rename` | ❌ 不可达（无 command 暴露） |

---

## 3. 当前暴露 Command 清单

| Command | 功能族 | 是否涉及真实文件操作 |
|---------|--------|---------------------|
| `start_rename_session` | session | ❌ 只执行 dry-run pipeline |
| `get_app_config` | config | ❌ 只读 |
| `get_all_templates` | config | ❌ 只读 |
| `set_template` | config | ✅ 写配置文件 |
| `get_confidence_threshold` | config | ❌ 只读 |
| `set_confidence_threshold` | config | ✅ 写配置文件 |
| `get_task` | audit | ❌ 只读 |
| `get_all_tasks` | audit | ❌ 只读 |
| `get_audit_logs` | audit | ❌ 只读 |

**确认**：当前暴露的 commands 中没有真实 rename / rollback。

---

## 4. 哪些入口当前不可达

| 入口 | 原因 |
|------|------|
| `rename/execution/single_rename.rs` | 无 `execute_rename` command 暴露 |
| `rollback/executor/rollback_single.rs` | 无 `rollback_task` command 暴露 |

**确认**：execution / rollback core 虽然存在，但不会被当前 command API 调用。

---

## 5. start_rename_session 安全性确认

**确认**：`start_rename_session` 只调用 `orchestrator::run_dry_run_pipeline`。

代码证据（`session/plan_session.rs:66`）：
```rust
orchestrator::run_dry_run_pipeline(path, &conn).map_err(|e| e.to_string())
```

`run_dry_run_pipeline` 内部流程：
1. scan - 扫描目录
2. parse - 解析文件名
3. preview - 生成预览
4. safety - 安全检查
5. audit - 写审计记录

**不包含**：`execution::execute` 或 `rollback::rollback_task`。

---

## 6. Audit 敏感字段脱敏确认

**模块**：`audit/redaction.rs`

**覆盖范围**：
- ✅ TMDb API Key
- ✅ 通用 API Key
- ✅ Token / Bearer
- ✅ Secret / Password
- ✅ URL Query 中的 key 参数
- ✅ JSON 字段中的敏感值

**测试覆盖**：
- `test_redact_tmdb_key`
- `test_redact_api_key`
- `test_redact_token`
- `test_redact_secret`
- `test_redact_url_query_key`
- `test_redact_json_field`
- `test_normal_text_not_redacted`
- `test_multiple_sensitive_fields`
- `test_contains_sensitive_true`
- `test_contains_sensitive_false`

**确认**：audit 不泄露敏感字段。

---

## 7. Config API Key 泄露确认

**模块**：`config/secret/api_key_store.rs`

**安全措施**：
- ✅ 使用单独的 JSON 文件存储（`.api_keys.json`）
- ✅ 不在 `config.toml` 中存储 API key
- ✅ 有 `mask_api_key` 脱敏功能
- ✅ 测试使用 tempdir

**确认**：config 不泄露 API key。

---

## 8. 测试安全性确认

**确认**：所有测试使用 tempdir / memory DB。

| 测试文件 | 使用 tempdir | 使用 memory DB |
|----------|--------------|----------------|
| `commands/session/session_contract_tests.rs` | ✅ | ✅ |
| `commands/config/config_contract_tests.rs` | ✅ | N/A |
| `commands/audit/audit_contract_tests.rs` | N/A | ✅ |
| `pipeline_hardening.rs` | ✅ | ✅ |
| `rename_conflict_detection.rs` | N/A | N/A |
| `rename_execution.rs` | ✅ | ✅ |
| `rollback_executor.rs` | N/A | ✅ |

---

## 9. Dry-run MVP 安全性

**结论**：✅ **安全**

理由：
1. `start_rename_session` 只执行 dry-run pipeline
2. dry-run pipeline 不调用 `execution::execute`
3. 所有测试使用 tempdir / memory DB
4. 无真实 rename / rollback 入口可达

---

## 10. Real Rename 是否仍 deferred

**结论**：✅ **仍 deferred**

理由：
1. 无 `execute_rename` command 暴露
2. `rename/execution/single_rename.rs` 不可达
3. 需要前端 UI 配合确认操作

---

## 11. Rollback 是否仍 deferred

**结论**：✅ **仍 deferred**

理由：
1. 无 `rollback_task` command 暴露
2. `rollback/executor/rollback_single.rs` 不可达
3. 需要前端 UI 配合确认操作

---

## 12. Risk Register

| # | Risk | Likelihood | Impact | Mitigation |
|---|------|------------|--------|------------|
| 1 | 意外暴露 execute_rename command | Very Low | Critical | 当前无 command 暴露，代码审查 |
| 2 | 意外暴露 rollback_task command | Very Low | Critical | 当前无 command 暴露，代码审查 |
| 3 | 配置文件写入失败 | Low | Low | 有错误处理 |
| 4 | API key 存储文件损坏 | Very Low | Low | 有 JSON 解析错误处理 |

---

## 13. Verification Results

| Check | Result |
|-------|--------|
| `cargo fmt --check` | ✅ PASS |
| `cargo check` | ✅ PASS |
| `cargo test` | ✅ 296 tests PASS |
| `cargo clippy --all-targets -- -D warnings` | ✅ PASS |

---

## 14. Summary

| 检查项 | 结果 |
|--------|------|
| 当前暴露 commands 中没有真实 rename / rollback | ✅ 确认 |
| start_rename_session 只执行 dry-run pipeline | ✅ 确认 |
| execution / rollback core 不会被当前 command API 调用 | ✅ 确认 |
| audit 不泄露敏感字段 | ✅ 确认 |
| config 不泄露 API key | ✅ 确认 |
| 所有测试使用 tempdir / memory DB | ✅ 确认 |
| dry-run MVP 安全 | ✅ 确认 |
| real rename 仍 deferred | ✅ 确认 |
| rollback 仍 deferred | ✅ 确认 |

**结论**：dry-run MVP 可以安全交给前端接入。real rename / rollback 仍被明确阻断。
