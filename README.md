# Media Renamer Kivo

A dry-run media file renamer built with React + TypeScript + Rust/Tauri.

## Current Status

**Version**: Dry-run MVP
**Branch**: `ui-redesign-dry-run-mvp`
**Status**: UI redesign complete, automation polish complete, PR ready

### Features

- **Scan**: Select a directory and scan for media files (movies, series, anime, etc.)
- **Preview**: View rename previews with search/filter, hover for full paths
- **Safety**: View safety report, blocking reasons, conflict/needs-review classification
- **Audit**: View task list, task details, and audit logs (read-only)
- **Settings**: Configure rename templates, confidence threshold, TMDb API key

### Important: Dry-run Only

This is a **dry-run MVP**. It does NOT perform real file renames or rollbacks.

| Feature | Status |
|---------|--------|
| Scan and preview | ✅ Active |
| Safety checks | ✅ Active |
| Audit logs | ✅ Read-only |
| Settings / templates | ✅ Active |
| TMDb API key storage | ✅ Active (secure, no network calls) |
| Real rename | ❌ Deferred |
| Rollback | ❌ Deferred |
| Rename / Execute / Rollback buttons | ❌ Not exposed |

### TMDb API Key

- Stored securely, never displayed in plaintext
- Input uses `type="password"`
- No network calls to TMDb are made
- Configuration only — no API key is ever leaked

## Development

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Tauri CLI

### Commands

```bash
# Install dependencies
npm install

# Start dev server with Tauri window
npm run tauri dev

# Build frontend
npm run build

# Type check
npx tsc --noEmit

# Lint
npm run lint

# Rust tests
cd src-tauri && cargo test

# Rust format check
cd src-tauri && cargo fmt --check

# Rust clippy
cd src-tauri && cargo clippy --all-targets -- -D warnings
```

### Test Results

```
cargo test: 315 tests PASS
npm run build: PASS
npx tsc --noEmit: PASS
npm run lint: PASS
```

## Architecture

### Frontend (React + TypeScript)

```
src/
├── app/              # App shell, router, layout
│   └── shell/        # TopBar, Sidebar
├── api/              # Tauri command wrappers
│   ├── session/      # start_rename_session
│   ├── config/       # templates, threshold, API key
│   └── audit/        # tasks, audit logs
├── features/         # Feature modules
│   ├── scan/         # Scan page
│   ├── preview/      # Preview page
│   ├── safety/       # Safety page
│   ├── audit/        # Audit page
│   └── settings/     # Settings page
└── shared/           # Shared utilities
    ├── ui/           # Visual primitives (Badge)
    └── time/         # Time formatting
```

### Backend (Rust + Tauri)

```
src-tauri/src/
├── commands/         # Tauri command layer
├── config/           # Configuration (templates, threshold, API key)
├── scan/             # File scanning
├── parse/            # Filename parsing
├── pipeline/         # Scan → Parse → Preview pipeline
├── session/          # Session management
├── audit/            # Audit logging
├── rename/           # Rename logic (deferred from UI)
├── rollback/         # Rollback logic (deferred from UI)
└── shared/           # Shared types and utilities
```

## Safety Boundaries

- No real file renames or rollbacks
- No `execute_rename` or `rollback_task` command wrappers
- No Rename / Execute / Rollback buttons in UI
- No network calls to TMDb
- API key never displayed in plaintext
- All destructive actions are DEFERRED

## License

MIT
