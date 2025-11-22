# Ironhold
Ironhold is a web-first Rust game engine with a browser-based editor, designed for WebGPU rendering and hot-reload workflows.
The prebuilt wasm game engine library can be used standalone in javascript. Or used through the browser-based editor.
Data will be loaded into the game engine through .ron files. Ron files are similar to json, but supports comments and you can leave the last comma. 
The main in javascript and in rust will look similar in usage.

Target platforms:
- **Web (WASM)**: _primary_. WebGPU-only (if unsupported, show a friendly message).
- **Windows portable**: _secondary_, added later without changing core architecture.
- **Linux portable**: _secondary_, added later without changing core architecture.

## High-level Goals

- **Prebuilt WASM engine**: published as `@van-geest/ironhold`, with a stable JS/TS API.
- **WASM editor** (egui/eframe) with **Edit** and **Play** modes. The viewport renders via the same engine render path.
- **2D + 3D** support (MVP: stubs/clear color, then small steps).
- **Data-driven**: scenes, components, and UI serialized (RON for authoring).
- **Game UI**: retained-mode (e.g., `taffy`), _not_ egui. Editor UI uses egui.
- **Hot reload** from day one for assets/scenes via a simple WS dev server.
- **No scripting/physics/networking** for MVP (add later via features).

## Workspace Structure (web-first)
- apps/
  - editor_web/           # Browser editor shell (egui/eframe)
    - src
      - lib.rs
    - Cargo.toml
    - index.html
    - favicon.ico
  - host_web/             # Minimal host sample (future)
    - src
      - lib.rs
    - Cargo.toml
- assets/
- crates/
  - editor_core/          # editor model (selection/cmd/undo) stub
    - src
      - lib.rs
    - Cargo.toml
  - editor_ui/            # egui panels (inspector, hierarchy) stub
    - src
      - lib.rs
    - Cargo.toml
  - engine_assets/        # assets + RON + hot-reload hooks
    - src
      - lib.rs
    - Cargo.toml
  - engine_core/          # ECS app, edit/play schedules
    - src
      - lib.rs
    - Cargo.toml
  - engine_ecs/           # Re-exports ECS utilities (lean)
    - src
      - lib.rs
    - Cargo.toml
  - engine_input/         # input abstraction (stub)
    - src
      - lib.rs
    - Cargo.toml
  - engine_render/        # wgpu renderer (to implement)
    - src
      - lib.rs
    - Cargo.toml
  - engine_scene/         # scene types + serde/RON
    - src
      - lib.rs
    - Cargo.toml
  - engine_ui/            # retained-mode game UI (taffy) stub
    - src
      - lib.rs
    - Cargo.toml
  - engine_wasm_api/      # wasm-bindgen JS API facade (prebuilt lib)
    - src
      - lib.rs
    - Cargo.toml
  - platform_web/         # WASM bindings; WebGPU init; WS hooks
    - src
      - lib.rs
      - wgpu_init.rs
    - Cargo.toml
  - xtask/                # dev server, build/bundle tasks
    - src
      - main.rs
    - Cargo.toml
- docs/                   # Project Documentation
  - [Architecture](./docs/ARCHITECTURE.md)          # Project architecture and design choices
  - [Build Guide](./docs/BUILD.md)                  # How to build the project and prereqs
  - [Changelog](./docs/CHANGELOG.md)                # All notable changes to this project will be documented in this file.
  - [Coding Standards](./docs/CODING_STANDARDS.md)  # Coding standards the project tries to adhere to
  - [Contribution](./docs/Contribution.md)          # Guide for contributors
  - [Dev Preferences](./docs/DEV_PREFERENCES.md)    # Lead dev project preferences notes
  - [Editor Notes](./docs/EDITOR_NOTES.md)          # Things that need remembering when developing the editor
  - [Roadmap](./ROADMAP.md)                         # 
  - [ToDo](./docs/TODO.md)                          # Task and task progress
  - [Troubleshooting](./docs/TROUBLESHOOTING.md)    # Solutions to issues to things we have already solved
  - [WebGPU Setup](./docs/WEBGPU_SETUP.md)          # The wgpu flow we use
- web/
  - engine-npm/           # npm packaging skeleton
  - static/               # editor CSS, etc.
- README.md
- Cargo.toml              # Root Cargo file
- config.toml             # "getrandom_backend=\"wasm_js\"", target = "wasm32-unknown-unknown"
- rust-toolchain.toml     # stable
- LICENSE-APACHE          # Dual MIT and Apache license
- LICENSE-MIT             # Dual MIT and Apache license


See more in [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md).


## License
Dual licensed under MIT and Apache 2.0.
Any assets in this repo fall under CC0 (public domain) or CC BY (give credit to creator).


## Current Status (2025-11-19)
- ✅ Dev server at `http://127.0.0.1:5173` (WS at `ws://127.0.0.1:5174/ws`)
- ✅ `engine_wasm_api` builds; bindgen outputs `/pkg/engine_wasm_api.js`
- ✅ Editor page imports engine and boots it
- ✅ **Viewport clears to sky blue and renders a red triangle** via a basic `wgpu` pipeline
- ✅ **Hot reload**: WS client + **cross‑platform** WS server (tokio‑tungstenite)
- ⚠️ Next: begin scene rendering from data (RON), reload scenes on change

See more in `docs/TODO.md` and `docs/CHANGELOG.md`.


### Quick Start (dev)
```bat
cargo run -p xtask -- dev-web
```
Open `http://127.0.0.1:5173`.

> If `/pkg/engine_wasm_api.js` is missing, run the `wasm-bindgen` step described in `docs/BUILD.md`, or use the automated `xtask build-web` that runs bindgen for you.
