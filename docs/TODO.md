# TODO / Next Sprint

## Must-have (to see a live viewport)

1. **WebGPU init**
   - Implement `platform_web::wgpu_init::init_wgpu` (adapter/device/queue/surface/config).
   - Return context with `device`, `queue`, `surface`, `config`.

2. **Engine mount + clear-color**
   - `Engine::mount_async()` calls `init_wgpu`, stores fields.
   - `tick()` acquires frame, clears to color, submits.

3. **Bindgen & /pkg**
   - `xtask build-web`: `wasm-bindgen --target web` for `engine_wasm_api.wasm`.
   - Output to `web/engine-npm/dist/`; dev server serves `/pkg/*`.
   - Update `apps/editor_web/index.html` to import `/pkg/engine_wasm_api.js` (temporary until npm publish).

4. **Editor boot -> Engine**
   - In the page, call:
     ```js
     await init();
     const opts = new EngineOptions().canvas_id("editor_canvas");
     const eng = await initEngine(opts);
     await eng.mount_async();
     eng.set_play_mode(true);
     eng.start();
     ```

## Nice-to-have (short)

5. **Hot reload WS**
   - Watch `/assets` in `xtask` and broadcast `{type:"asset-changed", url}`.
   - Engine/editor listens and calls `engine.hot_reload_asset(url)`.

6. **Editor viewport texture**
   - Render engine into texture and show in egui CentralPanel (egui_wgpu path).

7. **Scene authoring roundtrip**
   - Minimal `Scene` RON with one sprite/mesh; load & display.

## Later

- NPM packaging variants (`/2d`, `/3d`, `/full`).
- Portable Windows editor & host apps.
- Asset pipeline & importers (GLTF → internal).
- Reflection + inspector & undo/redo.
