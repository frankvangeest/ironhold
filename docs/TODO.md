# TODO / Next Sprint

## Must-have (to see a live viewport)
1. **WebGPU init** ✅
   - `platform_web::wgpu_init::init_wgpu` implemented (Instance → Surface → Adapter → Device/Queue → configure).
2. **Engine mount + clear-color** (IN PROGRESS)
   - `Engine.mount_async()` calls `init_wgpu` ✅
   - `tick()` clear-color path wired; **pending**: console errors prevent present on our current page.
3. **Bindgen & /pkg** ✅ (manual)
   - Manual `wasm-bindgen` step produces `/pkg/engine_wasm_api.js` and `.wasm`.
   - **Next**: automate via `xtask build-web` and call it from `dev-web`.
4. **Editor boot → Engine** ✅ (with errors)
   - Page calls:
     ```js
     await initWasm();
     const eng = await initEngine(new EngineOptions().canvas_id("editor_canvas"));
     await eng.mount_async();
     eng.set_play_mode(true);
     eng.start();
     ```
   - **Next**: resolve console errors; confirm canvas clear to sky blue.

## Nice-to-have (short)
5. **Hot reload WS**
   - Windows: WS thread **disabled** (miow/`ws` crate crash). Keep HTTP server only.
   - **Next**: replace `ws` crate with `tokio-tungstenite` (cross-platform), or keep stub until later.
6. **Editor viewport texture**
   - After base clear works, render to a texture and show inside egui panel.

## Next Session (bugs to fix)
- Favicon still 404 at `http://127.0.0.1:5173/` → adopt `/static/` mapping as documented.
- Console errors on the editor page → root-cause and fix (likely import path, timing, or missing bindgen artifacts).
- Confirm surface acquire/present path runs; canvas should clear to **sky blue**.
- Optional: implement `xtask run_bindgen(debug: bool)` and call from `dev-web`.

7. **Scene authoring roundtrip**
   - Minimal `Scene` RON with one sprite/mesh; load & display.

## Later

- Portable Windows editor & host apps.
- Asset pipeline & importers (GLTF → internal).
- Reflection + inspector & undo/redo.
