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
  - [ ] Minimal `Scene` RON w/ 3 quad/sprite; render in the pipeline
    - ✅ Ensure WGSL shader uses packed vec4 layout for InstanceData (Transform + Sprite)
    - ✅ Update Rust structs (Transform, Sprite, InstanceData) to match WGSL layout
    - ✅ Implement `scene_to_instances(scene)` to convert RON entities into InstanceData
    - ✅ Load `assets/example_scene.ron` via `fetch()` in JS and call `engine.load_scene_from_ron()`
    - ✅ Create instance buffer and bind group in QuadPipeline
    - ✅ Implement dynamic buffer resizing (`ensure_capacity()`) for variable instance counts
    - ✅ **Camera setup (orthographic projection):**
        - ✅ Add `CameraUniform` struct in Rust (`[f32; 16]` for mat4x4)
        - ✅ Create camera uniform buffer and bind group in QuadPipeline
        - ✅ Update pipeline layout to include camera bind group
        - ✅ Modify WGSL vertex shader to multiply `camera.view_proj * world_pos`
        - ✅ Implement `update_camera(queue, width, height)` in QuadPipeline
        - ✅ Call `update_camera()` inside `Engine::reconfigure_surface()` after canvas resize
    - [ ] Verify rendering of all 3 quads from `example_scene.ron` with correct positions, scales, and colors
          Status: Blue Sky color background visible, no quads visible.
  - [ ] Wire hot reload: on `asset-changed`, `fetch(url)` → parse RON → apply scene/asset
    - [ ] Implement WebSocket client callback in Rust (`start_hot_reload`) to receive `asset-changed` messages
    - [ ] Parse incoming JSON message into a typed enum (already partially done)
    - [ ] Extract `url` from message and validate it points to a `.ron` file
    - [ ] Use `web_sys::window().fetch(url)` to retrieve updated asset asynchronously
    - [ ] Await `Response.text()` and convert to Rust `String`
    - [ ] Call `engine.load_scene_from_ron(&ron_text)` to parse and apply new scene
    - [ ] Rebuild instance data: `scene_to_instances(scene)`
    - [ ] Call `pipeline.ensure_capacity()` to resize GPU buffer if needed
    - [ ] Call `pipeline.update_instances()` to upload new instance data
    - [ ] Trigger a redraw (next RAF tick will render updated scene)
    - [ ] Add error handling and logging for fetch failures or parse errors
  - [ ] Minimal `Scene` RON w/ one mesh; render in the pipeline
- Editor:
  - [ ] Viewport texture integration (render to texture, display in egui panel)
  - [ ] Inspector stubs (once reflection lands)
- [ ] Introduce proper error overlays in the editor for reload failures.
- [ ] Improve error logging and recovery for WebGPU edge cases.


---
For future ideas and longer-term goals, see [ROADMAP.md](./docs/ROADMAP.md).