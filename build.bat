cargo clean
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' && cargo build -p engine_wasm_api --target wasm32-unknown-unknown
