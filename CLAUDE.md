# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

A Tauri 2.x desktop client for Google Chat. **Windows-only**: Linux/macOS use WebKit, which Google blocks at the TLS-fingerprint level during OAuth. A fresh reimplementation of the Electron-based `google-chat-electron`.

## Commands

```powershell
pnpm install                 # install deps
pnpm exec lefthook install   # install git hooks (run once)
pnpm tauri dev               # run app (runs `pnpm dev:all` first via beforeDevCommand)
pnpm tauri build             # produce MSI + NSIS installers in src-tauri/target/release/bundle/

pnpm lint                    # biome check .
pnpm lint:fix                # biome check --write .
pnpm typecheck               # tsc --noEmit && svelte-check
pnpm test                    # vitest run (TS/Svelte tests)
pnpm test:watch              # vitest watch
pnpm vitest run injection/__tests__/zoom.test.ts   # run a single TS test file

# Rust (run from repo root, --manifest-path points cargo at src-tauri)
cargo test --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml chat_url_uses_account_index   # single test
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml
```

Git hooks (lefthook): pre-commit runs biome + rustfmt on staged files; pre-push runs typecheck, clippy (`-D warnings`), `cargo test`, and vitest; commit-msg enforces Conventional Commits (`feat:`/`fix:`/`perf:` drive release-please version bumps).

## Architecture

Three layers, **two separate frontend bundles**:

1. **Rust backend** (`src-tauri/src/`) — `main.rs` wires plugins, defines the `invoke_handler` command list, and builds windows in `setup()`. Each capability lives in its own module under `features/` (registered in `features/mod.rs`); add a new feature module there.

2. **Auxiliary-window UI** (`src/`) — Svelte 5 app built to `dist/`. Renders the local windows: Settings, About, Offline, Shortcuts (`src/views/`). This is the normal `frontendDist` Tauri app.

3. **Injection script** (`injection/`) — a self-contained IIFE bundle that runs *inside the remote Google Chat webview*. `injection/main.ts` calls each `install*()` module (unread counter, favicon observer, focus mode, notification patch, custom CSS, heartbeat, zoom, etc.). Built to `src-tauri/injection.js`.

### How the pieces connect

- The **main window** loads the *remote* URL `https://mail.google.com/chat/u/{accountIndex}` as a `WebviewUrl::External`, with the injection bundle passed as `initialization_script`. Auxiliary windows load the local Svelte `dist/`.
- `src-tauri/main.rs` embeds the injection bundle at **compile time** via `include_str!("../injection.js")`. **Editing `injection/*.ts` has no effect until you rebuild `src-tauri/injection.js`** (`pnpm build:injection`, or `pnpm dev:injection` to watch). `build.rs` reruns on changes to `injection.js`, so a rebuilt bundle triggers a Rust recompile.
- **Injection → Rust**: the injection script emits Tauri events (`unread-count`, `webview-heartbeat`, badge, injection-log); Rust feature modules register listeners for them in `setup()` (e.g. `badge::setup_listener`, `watchdog::setup_heartbeat_listener`).
- **UI → Rust**: `src/lib/ipc.ts` is the single typed bridge — every `invoke()` here maps to a `#[tauri::command]` in `features/`. Its `AppSettings`/`Theme` TypeScript types must stay in sync with the `serde(rename_all = "camelCase")` structs in `features/config.rs`.
- **Rust → UI/window**: `setup()` also listens for `apply-shortcut`, `apply-theme`, `apply-always-on-top` events emitted from the settings UI to apply changes live.
- **Settings persistence**: `tauri-plugin-store` writes `settings.json`; `features/config.rs` is the source of truth (defaults in `AppSettings::default()`).

### User-agent spoofing (why this is Windows-only)

`build.rs` fetches the latest Firefox version (`FIREFOX_VERSION` env overrides; falls back to a pinned constant offline) and bakes a **Linux Firefox** user-agent string into the binary (`FIREFOX_USER_AGENT`), applied via `.user_agent(USER_AGENT)` on the main window. This is what gets past Google's OAuth fingerprint block. It only works because Windows uses WebView2 (Chromium), not WebKit. `build.rs` also stamps `BUILD_DATE`.

## Conventions

- Biome formats/lints JS/TS/Svelte/JSON: 2-space indent, single quotes, semicolons, 100-col. `src-tauri/injection.js` (generated), `dist`, `target`, `gen` are excluded.
- Rust release profile is size-optimized (`opt-level = "s"`, `lto`, `panic = "abort"`).
- Tauri capabilities/permissions are declared in `src-tauri/capabilities/*.json` — new windows or plugin permissions must be added to the `windows` and `permissions` lists there.

## Releases

release-please opens a release PR from Conventional Commits on `main`; merging it tags and publishes a GitHub Release, which triggers `.github/workflows/release.yml` to build, sign (`TAURI_SIGNING_PRIVATE_KEY` secret), and upload installers plus `latest.json` for `tauri-plugin-updater`. The matching public key is embedded in `src-tauri/tauri.conf.json` under `plugins.updater.pubkey` — rotating the key requires updating both.
