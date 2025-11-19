# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.1.0-pre.3] - 2025-11-19 (Dev session)

### Added
- **Basic rendering pipeline (WebGPU/wgpu)**: draw a red triangle on a sky‑blue clear background to verify pipeline setup.
- **Hot reload (foundation)**:
  - **WebSocket client (WASM)** with robust JSON parsing using a tagged enum (`type` field), handling `asset-changed` and `hello` messages.
  - **Cross‑platform dev WebSocket server** in `xtask` using `tokio + tokio-tungstenite` (replaces old `ws` crate / Windows stub). Listens on `ws://<host>:(5173+1)/ws` by default.
- **Surface reconfiguration utilities**: proactive `Engine.reconfigure_surface()` callable from JS on resize and internally on surface acquisition errors.

### Changed
- `engine_wasm_api::Engine::mount_async()` now initializes a basic render pipeline after WebGPU context creation.
- `apps/editor_web/index.html` now calls `engine.start_hot_reload()` after starting the RAF loop.
- Switched WS URL construction to be **dynamic** via `window.location` (protocol, host) and `port+1` convention.

### Fixed
- Eliminated double render‑pass issue; merged clear and draw into a single pass in `tick()`.
- Resolved RAF/closure lifetime issues by storing the callback handle (Rc<RefCell<Option<Closure<..>>>>).
- Addressed **RefCell already borrowed** panic in RAF loop (previous session note) and improved logging.

### Developer Experience
- JSON parsing for WS frames now uses `serde/serde_json` with a tagged enum; unknown message kinds are ignored without warnings.
- Event handler closures now satisfy `FnMut` by wrapping callbacks in `Rc` and cloning per invocation.

### Notes
- Hot reload currently **logs** `asset-changed` events. Next: wire to `engine_assets` to fetch & apply changes (e.g., RON scene reload).
- WS server currently echoes frames; real file‑watch broadcasts to be added via `notify` integration.

### Security/Compatibility
- WebGPU surface is created via `SurfaceTarget::Canvas`; reconfigure on resize and acquisition errors for stability across browsers.
- Requires WebGPU‑capable browser (e.g., recent Chrome/Edge).

---

## [0.1.0-pre.2] - 2025-11-13
- WebGPU initialization path (WASM/web) established.
- Dev server serves `/pkg/*` from bindgen output.
- Editor boot calls into engine; initial clear color path.

## [0.1.0-pre.1] - 2025-11-01
- Project scaffolding, workspace crates, docs skeleton, and `xtask` basics.

