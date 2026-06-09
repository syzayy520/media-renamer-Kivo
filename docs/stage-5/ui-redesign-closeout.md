# UI Redesign Closeout Audit

**Date**: 2026-06-09  
**Branch**: ui-redesign-dry-run-mvp  
**HEAD**: 883c6b0 chore(settings): remove unused config field and getAppConfig call

---

## 1. Branch Status

| Item | Status |
|------|--------|
| Branch | `ui-redesign-dry-run-mvp` |
| Working Tree | ✅ Clean |
| HEAD | `883c6b0` |
| Broken UI commits in history | ✅ NO |

---

## 2. UI Redesign Commit Chain

从 `9f23bae` 后开始：

### 2.1 Core UI Redesign (UI-1 ~ UI-8)

| # | Hash | Ticket | Message |
|---|------|--------|---------|
| 1 | `8f1edd4` | UI-1 | feat(frontend): add dry-run MVP app shell |
| 2 | `d77031e` | UI-2 | feat(frontend): add typed backend API wrappers |
| 3 | `4b71c5b` | UI-2.1 | refactor(frontend): split backend API contract genealogy |
| 4 | `6985ef1` | UI-3 | feat(frontend): add scan dry-run flow |
| 5 | `f0bfdb2` | UI-3 fix | fix(frontend): normalize scan error state |
| 6 | `a00122a` | UI-4 | feat(frontend): add dry-run preview page |
| 7 | `3dee4e9` | UI-4 fix | fix(frontend): expose full preview paths via title |
| 8 | `ee0d881` | UI-5 | feat(frontend): add safety report page |
| 9 | `1f41111` | UI-6 | feat(frontend): add audit record page |
| 10 | `3935860` | UI-7 | feat(frontend): add settings page |
| 11 | `dc550a8` | UI-7 fix | fix(frontend): resolve settings loading state |
| 12 | `c3fe467` | closeout v1 | docs(frontend): close out dry-run MVP UI redesign |
| 13 | `10d2cf0` | UI-8 | feat(settings): add secure TMDb API key controls |
| 14 | `3afdfb4` | UI-8 test | test(config): add TMDb API key contract tests |
| 15 | `7f370f0` | UI-8 test | test(config): cover TMDb API key command contracts |
| 16 | `88c6588` | UI-8 test | test(settings): verify TMDb API key command handlers |

### 2.2 Automation Polish (Batch 1-4)

| # | Hash | Batch | Message |
|---|------|-------|---------|
| 17 | `bc70e4c` | 1 | chore(frontend): remove scaffolding artifacts and dead imports |
| 18 | `ae093b2` | 1 | fix(frontend): add aria-labels to all input elements |
| 19 | `a0ce0a2` | 1 | fix(types): constrain RenameConflict.conflict_type to real enum |
| 20 | `af26c9d` | 1 | fix(frontend): show error state in preview and safety pages |
| 21 | `ef5537b` | 1 | fix(audit): add keyboard accessibility to TaskList rows |
| 22 | `4f1f5a4` | 1 | fix(audit): use aria-selected instead of role=button on tr |
| 23 | `4daccc9` | 1 | refactor(scan): move async scan flow into store |
| 24 | `5b9fef9` | 1 | refactor(audit): move async audit flow into store |
| 25 | `5a4db2d` | 1 | fix(audit): clear stale state on selectTask and normalize errors |
| 26 | `7d82681` | 1 | fix(accessibility): label data tables |
| 27 | `1783eba` | 2 | fix(frontend): unify error message normalization |
| 28 | `775466f` | 2 | refactor(audit): extract shared formatTime utilities |
| 29 | `86ff92f` | 2 | fix(frontend): add loading state to preview and safety pages |
| 30 | `417494b` | 2 | refactor(frontend): move time formatting out of ui primitives |
| 31 | `d321012` | 3 | fix(frontend): use consistent py-8 padding for audit log loading state |
| 32 | `78e4763` | 3 | fix(types): tighten typeLabels to Record<MediaType,string> |
| 33 | `b73249f` | 4 | fix(settings): use toErrorMessage for save error handling |
| 34 | `68d2b43` | 4 | fix(settings): add explicit loading state and normalize error handling in API key panel |
| 35 | `883c6b0` | 4 | chore(settings): remove unused config field and getAppConfig call |

---

## 3. UI Ticket Closure Matrix

| # | Ticket | Status | Commit | Real Window | Touch Rust | Leak Key | Rename/Rollback |
|---|--------|--------|--------|-------------|------------|----------|----------------|
| 1 | UI-1 App Shell + Routing + Window Size | CLOSED | `8f1edd4` | ✅ | ❌ | N/A | ❌ |
| 2 | UI-2 TypeScript Contracts + API Wrappers | CLOSED | `d77031e` | N/A | ❌ | N/A | ❌ |
| 3 | UI-2.1 API Contract Genealogy Split | CLOSED | `4b71c5b` | N/A | ❌ | N/A | ❌ |
| 4 | UI-3 Scan Page Dry-run Flow | CLOSED | `6985ef1` | ✅ | ❌ | N/A | ❌ |
| 5 | UI-3 Runtime Fix (error state) | CLOSED | `f0bfdb2` | ✅ | ❌ | N/A | ❌ |
| 6 | UI-4 Preview Page | CLOSED | `a00122a` | ✅ | ❌ | N/A | ❌ |
| 7 | UI-4 Path Visibility Fix | CLOSED | `3dee4e9` | ✅ | ❌ | N/A | ❌ |
| 8 | UI-5 Safety Page | CLOSED | `ee0d881` | ✅ | ❌ | N/A | ❌ |
| 9 | UI-6 Audit Page | CLOSED | `1f41111` | ✅ | ❌ | N/A | ❌ |
| 10 | UI-7 Settings Page | CLOSED | `3935860` | ✅ | ❌ | N/A | ❌ |
| 11 | UI-7 Loading Fix | CLOSED | `dc550a8` | ✅ | ❌ | N/A | ❌ |
| 12 | UI-7 Save Smoke Check | CLOSED | — | ✅ | ❌ | N/A | ❌ |
| 13 | UI-8 TMDb API Key Secure Settings | CLOSED | `10d2cf0` | ✅ | ✅ | ❌ | ❌ |
| 14 | UI-8 Command Contract Tests | CLOSED | `3afdfb4` | N/A | ✅ | ❌ | ❌ |
| 15 | UI-8 Command Handler Tests | CLOSED | `88c6588` | N/A | ✅ | ❌ | ❌ |

### 3.1 Automation Polish Closure Matrix

| # | Ticket | Status | Commit |
|---|--------|--------|--------|
| T1 | Scaffolding cleanup | CLOSED | `bc70e4c` |
| T2 | Input aria-labels | CLOSED | `ae093b2` |
| T3 | conflict_type enum tightening | CLOSED | `a0ce0a2` |
| T4 | Preview/Safety error state | CLOSED | `af26c9d` |
| T5 | TaskList keyboard accessibility | CLOSED | `ef5537b`, `4f1f5a4` |
| T6 | Scan async into store | CLOSED | `4daccc9` |
| T7 | Audit async into store | CLOSED | `5b9fef9`, `5a4db2d` |
| T8 | Table accessibility labels | CLOSED | `7d82681` |
| T9 | Batch closeout audits | CLOSED | — |
| T10 | Error normalization unified | CLOSED | `1783eba` |
| T11 | Shared formatTime extraction | CLOSED | `775466f` |
| T12 | Preview/Safety loading state | CLOSED | `86ff92f` |
| T13 | formatTime genealogy fix | CLOSED | `417494b` |
| T14 | Loading padding consistency | CLOSED | `d321012` |
| T15 | typeLabels MediaType tightening | CLOSED | `78e4763` |
| T16 | Settings save error normalization | CLOSED | `b73249f` |
| T17 | API Key explicit loading state | CLOSED | `68d2b43` |
| T18 | Remove unused config/getAppConfig | CLOSED | `883c6b0` |

---

## 4. Page Readiness Matrix

### Scan Page

| Item | Status |
|------|--------|
| 可输入目录 | ✅ YES |
| 可调用 start_rename_session (via store action) | ✅ YES |
| 成功显示结果摘要 | ✅ YES |
| 错误显示 error panel | ✅ YES |
| 不真实修改文件 | ✅ YES |

### Preview Page

| Item | Status |
|------|--------|
| 从 scanStore 读取结果 | ✅ YES |
| 显示预览表格 | ✅ YES |
| 支持搜索/筛选 | ✅ YES |
| 原路径/新路径可通过 title 查看 | ✅ YES |
| 显示 error 和 loading 状态 | ✅ YES |
| 不显示 Rename / Apply / Execute / Rollback | ✅ YES |

### Safety Page

| Item | Status |
|------|--------|
| 显示 safety report | ✅ YES |
| 显示 blocking reasons | ✅ YES |
| 显示 conflict / needs review / safe 分类 | ✅ YES |
| 显示 error 和 loading 状态 | ✅ YES |
| 不显示 destructive action | ✅ YES |

### Audit Page

| Item | Status |
|------|--------|
| 只读调用 get_all_tasks / get_task / get_audit_logs (via store actions) | ✅ YES |
| 显示 task list | ✅ YES |
| 显示 task detail | ✅ YES |
| 显示 audit logs | ✅ YES |
| TaskList 键盘可访问 (tabindex, aria-selected, Enter/Space) | ✅ YES |
| 不显示 rollback 执行按钮 | ✅ YES |

### Settings Page

| Item | Status |
|------|--------|
| 加载 templates / threshold | ✅ YES |
| 模板可保存 | ✅ YES |
| 阈值可保存 | ✅ YES |
| API Key 保存 / 清除 | ✅ YES |
| API Key 不显示明文 (type=password) | ✅ YES |
| API Key 显式 loading 状态 | ✅ YES |
| 不调用 start_rename_session | ✅ YES |
| 不调用 get_app_config (removed in `883c6b0`) | ✅ YES |
| 不调用 execute_rename / rollback_task | ✅ YES |

---

## 5. Command Usage Matrix

### Active Frontend-Invoked Commands

| Page | Commands | Source |
|------|----------|--------|
| Scan | `start_rename_session` (via store) | `src/features/scan/state/scanStore.ts` |
| Preview | none (reads scanStore) | `src/features/scan/state/scanStore.ts` |
| Safety | none (reads scanStore) | `src/features/scan/state/scanStore.ts` |
| Audit | `get_all_tasks`, `get_task`, `get_audit_logs` (via store) | `src/features/audit/state/auditPageStore.ts` |
| Settings | `get_all_templates`, `set_template`, `get_confidence_threshold`, `set_confidence_threshold`, `get_tmdb_api_key_status`, `set_tmdb_api_key`, `clear_tmdb_api_key` | `src/features/settings/state/settingsPageStore.ts` |

### Exposed Backend Commands (not all actively invoked from UI)

| Command | Exposed in lib.rs | Actively Invoked |
|---------|:---:|:---:|
| `start_rename_session` | ✅ | ✅ (Scan) |
| `get_all_templates` | ✅ | ✅ (Settings) |
| `set_template` | ✅ | ✅ (Settings) |
| `get_confidence_threshold` | ✅ | ✅ (Settings) |
| `set_confidence_threshold` | ✅ | ✅ (Settings) |
| `get_tmdb_api_key_status` | ✅ | ✅ (Settings) |
| `set_tmdb_api_key` | ✅ | ✅ (Settings) |
| `clear_tmdb_api_key` | ✅ | ✅ (Settings) |
| `get_app_config` | ✅ | ❌ (removed in `883c6b0`) |
| `get_all_tasks` | ✅ | ✅ (Audit) |
| `get_task` | ✅ | ✅ (Audit) |
| `get_audit_logs` | ✅ | ✅ (Audit) |

### Safety Confirmation

| Item | Status |
|------|--------|
| start_rename_session 只在 Scan 页面调用 | ✅ YES |
| Preview/Safety 不调用后端 command | ✅ YES |
| execute_rename wrapper | ✅ NO |
| rollback_task wrapper | ✅ NO |
| execute_rename invoke | ✅ NO |
| rollback_task invoke | ✅ NO |

---

## 6. Destructive Action Safety Matrix

| Action | Has Button | Has Wrapper | Has invoke Call | Real File Change |
|--------|-----------|-------------|----------------|-----------------|
| Rename | NO | NO | NO | NO |
| Apply Rename | NO | NO | NO | NO |
| Execute | NO | NO | NO | NO |
| Rollback | NO | NO | NO | NO |
| execute_rename | NO | NO | NO | NO |
| rollback_task | NO | NO | NO | NO |

---

## 7. TMDb API Key Security Matrix

### Frontend
| Item | Status |
|------|--------|
| 输入框 type=password | ✅ YES |
| 保存后清空输入框 | ✅ YES |
| 不回显明文 key | ✅ YES |
| 显式 isLoading 状态 | ✅ YES |
| 不联网 | ✅ YES |

### Backend
| Item | Status |
|------|--------|
| get_tmdb_api_key_status 只返回 `configured: bool` | ✅ YES |
| set_tmdb_api_key 返回值不含明文 | ✅ YES |
| clear_tmdb_api_key 返回值不含明文 | ✅ YES |
| empty/whitespace key 返回错误 | ✅ YES |
| error message 不含明文 key | ✅ YES |

### Tests
| Item | Status |
|------|--------|
| Handler unit tests (8) | ✅ PASS |
| Contract tests | ✅ PASS |
| Serialization no plaintext | ✅ PASS |

---

## 8. UI Genealogy Audit

| Item | Status |
|------|--------|
| App.tsx 只是 thin bootstrap / root composition | ✅ YES |
| router.tsx 只做路由 | ✅ YES |
| AppShell / Sidebar / TopBar 单职责 | ✅ YES |
| 每个 feature 有 page/components/state 分层 | ✅ YES |
| src/api 已按 session/config/audit/shared 拆分 | ✅ YES |
| shared/ui/ = visual primitives only | ✅ YES |
| shared/time/ = time formatting utilities | ✅ YES |
| 没有 helper/utils/manager/service/glue/facade 桶文件 | ✅ YES |
| 没有一个页面文件塞多个大 section | ✅ YES |
| 错误消息标准化 (toErrorMessage) | ✅ YES |

---

## 9. Verification Results

| Check | Result |
|-------|--------|
| `cargo fmt --check` | ✅ PASS |
| `cargo check` | ✅ PASS |
| `cargo test` | ✅ **315 tests PASS** |
| `cargo clippy --all-targets -- -D warnings` | ✅ PASS |
| `npm run build` | ✅ PASS |
| `npx tsc --noEmit` | ✅ PASS |
| `npm run lint` | ✅ PASS |
| `git diff --check` | ✅ PASS |

---

## 10. Real Window Verification Summary

| Ticket | Real Window Check | Verified |
|--------|------------------|----------|
| UI-1 | Shell 视觉通过 | ✅ |
| UI-3 | Scan 成功/错误路径 | ✅ |
| UI-4 | Preview 表格、搜索、path title | ✅ |
| UI-5 | Safety blocking reasons、conflict | ✅ |
| UI-6 | Audit task list/detail/logs、键盘访问 | ✅ |
| UI-7 | Settings 加载、模板、阈值保存 | ✅ |
| UI-8 | TMDb Key 保存/清除/不回显明文 | ✅ |

---

## 11. Deferred Items

| Item | Status | Reason |
|------|--------|--------|
| execute_rename command | DEFERRED | 需要前端确认 UI |
| rollback_task command | DEFERRED | 需要前端确认 UI |
| Real Rename UI | DEFERRED | 当前 UI 冻结 |
| Rollback UI | DEFERRED | 当前 UI 冻结 |
| Tauri dialog Browse | DEFERRED | 未启用 |
