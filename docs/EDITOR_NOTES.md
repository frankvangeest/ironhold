# Editor Notes

- **UI framework**: egui/eframe (web). Game runtime UI is separate (retained).
- **Layout**: a top toolbar, a viewport (center), and placeholders for hierarchy/inspector/assets.
- **Mode switch**: a play/stop button toggling `Engine.set_play_mode(...)` and world/schedule selection.
- **Viewport**: show engine output as an egui image/texture.

## Planned panels

- **Hierarchy**: simple tree of entities
- **Inspector**: derived (later with reflection) editing components
- **Assets**: file browser (dev-only) backed by `/assets` static server

## Bootstrap (web page)
The editor page uses an ES module to load the bindgen artifacts and start the engine:
```js
import initWasm, { init as initEngine, EngineOptions } from "/pkg/engine_wasm_api.js";

await initWasm();
const eng = await initEngine(new EngineOptions().canvas_id("editor_canvas"));
await eng.mount_async();
eng.set_play_mode(true);
eng.start();
```
Make sure `/pkg/engine_wasm_api.js` exists (see `docs/BUILD.md` → bindgen).


