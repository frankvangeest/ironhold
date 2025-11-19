# Build & Run (Windows 11, cmd.exe)

> Web is primary; Windows native hosts can be added later.

## Prereqs

- Rust stable; targets:
  ```bat
  rustup target add wasm32-unknown-unknown
  ```
- Ensure WebGPU-capable browser (Chrome/Edge recent).

## getrandom on WASM (browser)

For `wasm32-unknown-unknown`, we enable the JS backend:

- In `engine_wasm_api/Cargo.toml`:
  ```toml
  [target.'cfg(target_arch = "wasm32")'.dependencies]
  getrandom = { version = "0.3", features = ["js"] }
  ```
- We compile with:
  ```bat
  set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
  ```
  (Handled automatically by `xtask`.)

## Dev server + build

From the repo root:

```bat
cargo run -p xtask -- dev-web
```

This will:
- inject `RUSTFLAGS=--cfg getrandom_backend="wasm_js"`
- build `engine_wasm_api` and `editor_web` for wasm
- start a static server on `http://127.0.0.1:5173`
- (WS on `ws://127.0.0.1:5174/ws` – for hot reload later)

Open: `http://127.0.0.1:5173`

## Manual builds (if needed)

```bat
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
cargo build -p engine_wasm_api --target wasm32-unknown-unknown
cargo build -p editor_web     --target wasm32-unknown-unknown
```

## Create a snapshot of all *.md, *.rs and *.toml files in the project to /docs/project_snapshot.txt

```bat
cargo run -p xtask -- export-sources
```

## Editor (eframe/web) specifics

- Use `eframe = { version = "0.33", default-features = false, features = ["wgpu"] }`
- `WebRunner` is under `eframe::web`, but **guard your file**:
  ```rust
  #![cfg(target_arch = "wasm32")]
  ```
- `WebOptions` is re-exported at `eframe::WebOptions`.

## Next: wasm-bindgen step

We'll add an `xtask build-web` step that:
- runs `wasm-bindgen --target web` on `engine_wasm_api.wasm`
- outputs into `web/engine-npm/dist/`
- dev server serves that under `/pkg/*`

Then your `apps/editor_web/index.html` can `import "/pkg/engine_wasm_api.js"` and drive the engine directly.
