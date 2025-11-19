# TODO / Next Sprint

## Must-have (to see a live viewport)
1. **WebGPU init** ✅
   - `platform_web::wgpu_init::init_wgpu` implemented (Instance → Surface → Adapter → Device/Queue → configure).
2. **Engine mount + clear-color** ✅
   - `Engine.mount_async()` calls `init_wgpu` ✅
   - `tick()` clear-color path wired; canvas clears to sky blue.
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
   - ✅ Fixed RefCell already borrowed panic by adopting Rc<RefCell<Option<Closure>>> pattern.
   - ✅ Fixed WebGPU getCurrentTexture null context error by switching to SurfaceTarget::Canvas and adding proactive surface reconfigure logic.

## Next Steps
- ✅ Implement automatic surface reconfigure on window resize and any acquire error.
- Begin scene rendering pipeline inside editor viewport (currently placeholder clear).
- Add hot-reload integration for assets and scenes.
- Improve error logging and recovery for WebGPU edge cases.

## Nice-to-have (short)
5. **Hot reload WS**
   - Windows: WS thread **disabled** (miow/`ws` crate crash). Keep HTTP server only.
   - **Next**: replace `ws` crate with `tokio-tungstenite` (cross-platform), or keep stub until later.
6. **Editor viewport texture**
   - After base clear works, render to a texture and show inside egui panel.

7. **Scene authoring roundtrip**
   - Minimal `Scene` RON with one sprite/mesh; load & display.

## Longer-Term Goals
- Editor panels for scene graph and component editing.
- Data-driven UI authoring using taffy.
- Feature flags for physics and scripting.
- Portable Windows editor & host apps.
- Asset pipeline & importers (GLTF → internal).
- Reflection + inspector & undo/redo.