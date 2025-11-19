# TODO / Next Sprint

- Mark as **done**:
  - WebGPU init ✅
  - Engine mount + clear color ✅
  - Bindgen + `/pkg` ✅
  - Editor boot → Engine ✅
  - **Proactive surface reconfigure** ✅
  - **Basic render pipeline (triangle)** ✅
  - **Hot reload WS client + cross‑platform WS server** ✅
- **Next Steps**:
  - Load a RON scene and render primitives/mesh.
  - Tie `asset-changed` → fetch URL → parse (RON) → replace scene / asset.
  - Introduce proper error overlays in the editor for reload failures.

## Next Steps
- Scene rendering from data:
  - [ ] Minimal `Scene` RON w/ one quad/sprite; render in the pipeline
  - [ ] Wire hot reload: on `asset-changed`, `fetch(url)` → parse RON → apply scene/asset
  - [ ] Minimal `Scene` RON w/ one mesh; render in the pipeline
- Editor:
  - [ ] Viewport texture integration (render to texture, display in egui panel)
  - [ ] Inspector stubs (once reflection lands)
- Improve error logging and recovery for WebGPU edge cases.

## Nice-to-have (short)
**Broadcast asset-changed events on file changes**
   - Wire the notify watcher in xtask to broadcast real asset-changed events on file changes. (So we don’t need a separate WS client for testing).
**Partial project snapshot**
   - Add an optional parameter to only include a list of files to the 'cargo run -p xtask -- export-sources' command. E.g. a comma seperated list [file.md,file2.rs,file3.toml]. Or something similar. Without the additional parameter is should still be have as it has before.

## Longer-Term Goals
- Editor panels for scene graph and component editing.
- Data-driven UI authoring using taffy.
- Asset pipeline & importers (GLTF → internal).
- Reflection + inspector & undo/redo.
- Texture instancing
- Model instancing
- Portable Windows editor & host apps.
- Feature flags for physics, networking and scripting.
- WGSL shader rendered grass
   - good looking interactive grass shader
   - include character movement through the grass
   - add wind_direction and wind_strength parameters and create a controllable wind effect
   - add grass_color parameter to control the color of the grass
- WGSL shader rendered water
   - good looking interactive water shader
   - include character movement through the water
   - include dynamic wind effect
- WGSL shader rendered sand (like in the game Journey)
   - good looking interactive sand shader
   - include character movement through the sand
   - include dynamic wind effect
- WGSL shader rendered snow
   - good looking interactive snow shader
   - include character movement through the snow
   - include dynamic wind effect
- Slime shader
- Lava shader
- Stone shader
- Wood shader
- efficient GPU physics