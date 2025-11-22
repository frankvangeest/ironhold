# Ironhold Roadmap

This roadmap outlines the planned development stages for Ironhold. It will evolve as the project grows. For detailed tasks and updates, see:
- [TODO.md](./docs/TODO.md)
- [CHANGELOG.md](./docs/CHANGELOG.md)

---

## ✅ Completed (MVP Foundations)
- WebGPU initialization and surface handling.
- WASM engine builds and JS glue via `wasm-bindgen`.
- Editor boots and integrates engine. 
- Basic rendering pipeline (clear color + red triangle).
- Hot reload foundation (WS client + cross-platform WS server).

---

## 🔜 Short-Term Goals
- **Scene Rendering**:
  - Load RON scene data and render primitives (quad/sprite).
  - Implement reload on asset change (hot reload → fetch → apply).
- **Editor Enhancements**:
  - Viewport texture integration (render to texture, display in egui panel).
  - Inspector stubs (requires reflection).
- **Error Handling**:
  - Add error overlays for reload failures.
- **Dev Experience**:
  - Improve logging and recovery for WebGPU edge cases.
  - Add an optional parameter to only include a list of files and or crates to the 'cargo run -p xtask -- export-sources' command. E.g. a comma seperated list [file.md,file2.rs,file3.toml,<crate_name>]. Or something similar. Without the additional parameter is should still be have as it has before.
---

## ⏳ Medium-Term Goals
- **Editor Panels**:
  - Hierarchy view (entity tree).
  - Inspector with component editing.
  - Asset browser.
- **Data-Driven UI**:
  - Implement retained-mode UI with `taffy`.
- **Asset Pipeline**:
  - Add GLTF importer and basic asset management.
- **Reflection & Undo/Redo**:
  - Introduce reflection for component editing.
  - Implement undo/redo system.

---

## 🚀 Long-Term Goals
- **Advanced Rendering**:
  - WGSL shaders for grass, water, sand, snow, etc.
  - Dynamic wind and interactive effects.
- **Performance Features**:
  - Texture and model instancing.
  - Efficient GPU physics.
- **Platform Expansion**:
  - Portable Windows editor and host apps.
- **Feature Flags**:
  - Physics, networking, scripting support.

---

## 📌 Versioning Milestones
- **0.1.x-pre**: MVP + basic scene rendering.
- **0.2.x**: Editor panels + asset pipeline.
- **0.3.x**: Advanced rendering + Windows support.
- **1.0**: Stable API, full editor, multi-platform.

---
This roadmap is a living document and will be updated as priorities shift.
