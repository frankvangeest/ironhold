## wasm-bindgen CLI fails with getrandom "wasm_js" error (Windows)

### Symptom
```
error: The "wasm_js" backend requires the `wasm_js` feature for `getrandom` ...
```

### Cause
`cargo install wasm-bindgen-cli` (host binary) inherited `RUSTFLAGS=--cfg getrandom_backend="wasm_js"`.
That flag is only valid when compiling **wasm** targets, not host tools. Host builds will trip `compile_error!`.

### Fix
Install the CLI in a clean shell:
```bat
set RUSTFLAGS=
cargo install wasm-bindgen-cli --version 0.2.100
```
Use `RUSTFLAGS=--cfg getrandom_backend="wasm_js"` **only** for the wasm build of `engine_wasm_api`.

---

## Windows dev: process abort in `miow` (WS thread)

### Symptom
Dev server starts, then aborts with a stack buffer/miow null-pointer panic.

### Cause
The `ws` crate (0.9) relies on `miow` on Windows. `miow` is effectively unmaintained and can crash on modern setups.

### Fix
Disable WS on Windows (stub) and keep HTTP server. For cross-platform WS, replace `ws` with `tokio + tokio-tungstenite` later.

---

## Editor Web OperationError: Failed to execute 'requestDevice' on 'GPUAdapter': The limit "maxInterStageShaderComponents" with a non-undefined value is not recognized.

### Symptom
When using the build wasm library. The adapter.request_device call would fail. Changing Limits::default() to Limits::downlevel_defaults() or Limits::downlevel_webgl2_defaults() did not work. Changing limits to adapter.limits() did not work.

### Cause
wgpu version 0.20 was outdated. maxInterStageShaderComponents is no long supported by the latest browsers.

### Fix
Updated to wgpu 27. The newer version of wgpu also has new versions of the various limit defaults.

---

## WebGPU null context error (`getCurrentTexture`)

### Symptom
TypeError: Cannot read properties of null (reading 'getCurrentTexture')

### Cause
The WebGPU surface was created using raw-handle mapping instead of the Canvas path, or the surface was not reconfigured after a resize.

### Fix
- Switch to `SurfaceTarget::Canvas` when creating the surface.
- Add proactive surface reconfigure logic on resize and on any acquisition error.

---

# WebSocket fails to connect on Windows
**Symptom**
WebSocket connection to 'ws://127.0.0.1:5174/ws' failed

**Cause**
Old `ws` crate server wasn’t started on Windows.

**Fix**
Replaced with `tokio + tokio-tungstenite`; server runs on all platforms at `ws://127.0.0.1:(5173+1)/ws`.

---

## WS JSON parse error: missing field `url` for `{"type":"hello"}`

**Symptom**
`WS JSON parse error (text): missing field url => {"type":"hello"}`

**Fix**
Client now parses with a tagged enum (`type` field). `hello` is accepted and ignored.

---



