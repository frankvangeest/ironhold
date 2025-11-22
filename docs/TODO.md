# DONE
  - WebGPU init ✅
  - Engine mount + clear color ✅
  - Bindgen + `/pkg` ✅
  - Editor boot → Engine ✅
  - **Proactive surface reconfigure** ✅
  - **Basic render pipeline (triangle)** ✅
  - **Hot reload WS client + cross‑platform WS server** ✅


# TODO
- Scene rendering from data:
  - [ ] Minimal `Scene` RON w/ one quad/sprite; render in the pipeline
  - [ ] Wire hot reload: on `asset-changed`, `fetch(url)` → parse RON → apply scene/asset
  - [ ] Minimal `Scene` RON w/ one mesh; render in the pipeline
- Editor:
  - [ ] Viewport texture integration (render to texture, display in egui panel)
  - [ ] Inspector stubs (once reflection lands)
- [ ] Introduce proper error overlays in the editor for reload failures.
- [ ] Improve error logging and recovery for WebGPU edge cases.


---
For future ideas and longer-term goals, see [ROADMAP.md](./docs/ROADMAP.md).