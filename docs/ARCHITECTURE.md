# Ironhold Architecture

## Overview

- **Editor shell (web)**: egui/eframe UI with docking + viewport. Uses the engine to render to a texture displayed inside the editor.
- **Engine**: ECS-based core with distinct **Edit** and **Play** schedules to keep authoring/runtime separate and make undo/redo sane.
- **Render**: `wgpu` for both web and native. Web is WebGPU-only.
    - Basic wgpu pipeline and shader (WGSL) is in place for a triangle draw.
- **Surface handling (Web)**: Use `SurfaceTarget::Canvas` for creating the WebGPU surface instead of raw-handle mapping. This ensures a valid `GPUCanvasContext` and avoids null context errors. SurfaceTarget::Canvas and proactive reconfigure on resize and on acquire errors.
- **Data**: authoring formats are **RON** (human-readable). Packaging can later switch to a compact binary if needed.
- **Game UI**: retained-mode using `taffy` for layout, authored as data, rendered by the engine. (Editor UI = egui only.)
- **Hot reload**: minimal WS server (xtask) emits change events; engine re-fetches via `fetch()` through a web VFS.

## Key Design Decisions

- **WebGPU only** MVP on web. If unsupported, show canvas message; no WebGL fallback.
- **No scripting/physics** in MVP. Hooks exist to add later (`features`).
- **Prebuilt engine library** for web consumption from JS/TS. Editor and user games consume the same library.
- **Feature flags & variants**: publish different npm entry points later (e.g., `/2d`, `/3d`, `/full`).

## Engine API (WASM)

Currently exported from `engine_wasm_api` (subject to iteration):

- `init(opts: EngineOptions) -> Promise<Engine>`
- `Engine.mount_async() -> Promise<void>` — does WebGPU init and surface config for the bound canvas
- `Engine.start()` / `Engine.stop()` — requestAnimationFrame loop
- `Engine.start_hot_reload()` — starts hot-reload WebSocket listener
- `Engine.tick(dt_ms: f32)` — if you want to drive it manually
- `Engine.load_scene_from_ron(ron: &str)`
- `Engine.set_play_mode(play: bool)`

`EngineOptions`:
- `canvas_id(string)`, `assets_base_url(string)`, `enable_2d(bool)`, `enable_3d(bool)`

## Editor Modes

- **Edit**: authoring-only world; systems paused or specialized for editing. Inspector writes authoring components/assets.
- **Play**: build a fresh runtime world from the authoring scene; run full game loop. On stop, discard this world.

## Hot Reload
- Dev server (`xtask dev-web`) serves static content and exposes a WebSocket at `ws://<host>:(HTTP_PORT+1)/ws` (e.g., `5174` when HTTP is `5173`).
- On asset/file change: server would broadcast `{ type: "asset-changed", url }`; engine invalidates handle and re-fetches via `fetch`.
- Messages are JSON-tagged:
  - `{"type":"asset-changed","url":"/assets/foo.ron"}`
  - `{"type":"hello"}` (greeting; ignored by client)
- The client (WASM) parses messages with a tagged enum and triggers reloads by URL.

## JS Interop

- Expose a **coarse-grained** JS API (avoid thousands of tiny calls). Scenes/assets loaded as blobs/strings.
- `wasm-bindgen` will produce `*.js` glue + `*.wasm`; `xtask build-web` will run bindgen, then serve result under `/pkg`.
 


