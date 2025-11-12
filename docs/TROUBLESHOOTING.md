# Troubleshooting

This file records issues we already hit (and fixes), plus quick diagnostics you can run next time.

> Target environment: **Windows 11 / cmd.exe**, building for **wasm32-unknown-unknown** (WebGPU-only), using `xtask` to inject RUSTFLAGS and run a tiny dev server.

---

## eframe feature errors

### Symptom
```
error: failed to select a version for `eframe`
package `editor_web` depends on `eframe` with feature `web_browser` but `eframe` does not have that feature.
```

### Fix
Use the WebGPU backend feature (no `web_browser` feature exists):
```toml
# apps/editor_web/Cargo.toml
eframe = { version = "0.29", default-features = false, features = ["wgpu"] }
```

---

## eframe web API not found

### Symptoms
- `could not find WebOptions in eframe`
- `could not find WebRunner in eframe`
- `unresolved import eframe::web`

### Causes & Fixes
- The **web types** live under **`eframe::web`** and are only compiled on **`wasm32`**.
- Guard your web entry with:
  ```rust
  #![cfg(target_arch = "wasm32")]
  ```
- Import paths:
  ```rust
  use eframe::web::WebRunner;   // under eframe::web
  use eframe::WebOptions;       // re-exported at crate root
  ```
- `WebRunner::start(..)` expects a **`web_sys::HtmlCanvasElement`**, not a string id.

---

## getrandom on wasm (browser)

### Symptoms
- `The wasm32-unknown-unknown targets are not supported by default; you may need to enable the "wasm_js" configuration flag.`
- Missing `fill_inner` / `inner_u32` functions.

### Fix (two parts)
1) **Dependency feature** for browser:
   ```toml
   # engine_wasm_api/Cargo.toml (or the top-level wasm entry crate)
   [target.'cfg(target_arch = "wasm32")'.dependencies]
   getrandom = { version = "0.3", features = ["js"] }
   ```
2) **Compiler cfg** so the JS backend is selected:
   - Use `xtask dev-web` (injects `RUSTFLAGS` automatically), or set manually:
     ```bat
     set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
     ```

### Mixed versions
If errors persist, check for **multiple versions** of `getrandom`:
```bat
cargo tree -i getrandom
```
If `0.2.x` and `0.3.x` coexist, update offending deps or pin via `cargo update -p getrandom --precise 0.3.x`.

---

## js_sys / web_sys usage

### Symptom
```
use of unresolved module or unlinked crate `js_sys`
```

### Fix
Add the missing deps to the crate that uses them:
```toml
js-sys  = "0.3"
web-sys = { version = "0.3", features = ["Window","Document","HtmlCanvasElement","console"] }
```

---

## RAF loop lifetime / type inference

### Symptoms
- Type annotations needed for `RcCell<_>`
- Closure dropped → no animation

### Fix
Store the RAF `Closure<dyn FnMut(f64)>` **on the Engine struct** so it lives across frames. Avoid ambiguous helpers; if needed, use explicit types:
```rust
raf_closure: Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>
```
and populate it in `start()`.

---

## tiny_http ownership errors

### Symptoms
- `use of moved value: req`
- `cannot move out of *req which is behind &mut`

### Fix
`tiny_http::Request::respond(self, ...)` **consumes** the request.
- Don’t capture `req` in closures.
- Write responders that take `Request` **by value** and call them once per request path:
```rust
fn respond_not_found(req: Request) { let _ = req.respond(...); }
fn respond_file(req: Request, path: PathBuf) { /* ... */ }
```

---

## Cargo workspace feature leakage / cyclic deps

### Symptoms
- Cyclic dependency (crate points to itself in `[dependencies]`).
- Unexpected feature errors from workspace-wide settings.

### Fixes
- **Never** add a path-dep from a crate to itself.
- Keep dependency versions/features **local** to each crate until things stabilize.
- If you want shared versions later, use a small `[workspace.dependencies]` **without** forcing features.

---

## eframe app creator closure type

### Symptom
```
expected Result<Box<dyn App>, Box<dyn Error + Send + Sync>>
```

### Fix
Return a `Result` from the app-creator closure:
```rust
WebRunner::new().start(
    canvas,
    WebOptions::default(),
    Box::new(|_cc| Ok::<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>(Box::new(EditorApp::default()))),
).await?;
```

---

## IDE (host) builds of wasm-only crates

### Symptom
IDE shows unresolved imports for web-only APIs.

### Fix
Guard the file/module with:
```rust
#![cfg(target_arch = "wasm32")]
```
Optionally create a tiny non-wasm stub module so the crate stays buildable in the workspace when targeted by the host.

---

## Quick commands (cmd.exe)

```bat
:: dev server + build (injects RUSTFLAGS for getrandom wasm_js backend)
cargo run -p xtask -- dev-web

:: manual wasm builds
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
cargo build -p engine_wasm_api --target wasm32-unknown-unknown
cargo build -p editor_web     --target wasm32-unknown-unknown

:: diagnose getrandom versions
cargo tree -i getrandom
```

---

## When in doubt
1) `cargo clean` and rebuild the specific crate with `--target wasm32-unknown-unknown`.
2) Verify the file has `#![cfg(target_arch = "wasm32")]` if it imports `eframe::web`.
3) Confirm `RUSTFLAGS` includes `--cfg getrandom_backend="wasm_js"` (or run via `xtask`).
4) If errors persist, paste the **first** error message and the corresponding file lines.
