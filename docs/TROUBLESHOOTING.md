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
