# Ironhold

A web-first Rust game engine **library** (prebuilt to WebAssembly and usable from JavaScript) plus a **browser-based editor**. Target platforms:

- **Web (WASM)**: _primary_. WebGPU-only (if unsupported, show a friendly message).
- **Windows portable**: _secondary_, added later without changing core architecture.

## High-level Goals

- **Prebuilt WASM engine**: published as `@van-geest/ironhold`, with a stable JS/TS API.
- **WASM editor** (egui/eframe) with **Edit** and **Play** modes. The viewport renders via the same engine render path.
- **2D + 3D** support (MVP: stubs/clear color, then small steps).
- **Data-driven**: scenes, components, and UI serialized (RON for authoring).
- **Game UI**: retained-mode (e.g., `taffy`), _not_ egui. Editor UI uses egui.
- **Hot reload** from day one for assets/scenes via a simple WS dev server.
- **No scripting/physics** for MVP (add later via features).

## Workspace Structure (web-first)
- apps/
  - editor_web/           # Browser editor shell (egui/eframe)
  - host_web/             # Minimal host sample (future)
- crates/
  - editor_core/          # editor model (selection/cmd/undo) stub
  - editor_ui/            # egui panels (inspector, hierarchy) stub
  - engine_assets/        # assets + RON + hot-reload hooks
  - engine_core/          # ECS app, edit/play schedules
  - engine_ecs/           # Re-exports ECS utilities (lean)
  - engine_input/         # input abstraction (stub)
  - engine_render/        # wgpu renderer (to implement)
  - engine_scene/         # scene types + serde/RON
  - engine_ui/            # retained-mode game UI (taffy) stub
  - engine_wasm_api/      # wasm-bindgen JS API facade (prebuilt lib)
  - platform_web/         # WASM bindings; WebGPU init; WS hooks
  - xtask/                # dev server, build/bundle tasks
- web/
  - engine-npm/           # npm packaging skeleton
  - static/               # editor CSS, etc.
- assets/

See more in `docs/ARCHITECTURE.md`.