# WebGPU Setup (Web)

## Requirements
- WebGPU-capable browser (Chrome/Edge recent; Safari TP with flags).
- `<canvas id="editor_canvas"></canvas>` present in `index.html`.

## Initialization Flow
In `platform_web::wgpu_init::init_wgpu(canvas)`:
1. `let instance = wgpu::Instance::default();`
2. `let surface = instance.create_surface_from_canvas(&canvas) …`
3. `let adapter = instance.request_adapter(..).await.unwrap();`
4. `let (device, queue) = adapter.request_device(..).await.unwrap();`
5. Pick an sRGB `format` from `surface.get_capabilities(&adapter)`.
6. Configure `SurfaceConfiguration` with `usage: RENDER_ATTACHMENT`, width/height from canvas.

## Render Loop
- `Engine::start()` schedules RAF; calls `tick(dt_ms)`.
- `tick()`:
  - `surface.get_current_texture()` → `view`.
  - Begin a render pass that clears, `queue.submit(..)`, `frame.present()`.

## Resizing & Recovery
- On canvas resize (JS window `resize`): update canvas pixel size, then call `Engine.reconfigure_surface()`.
- In the RAF loop: if `surface.get_current_texture()` errors, log and reconfigure once, then retry acquire.

## Unsupported WebGPU
- Detect via `navigator.gpu` in JS or failure in init.
- Show a message overlay or draw text via 2D canvas explaining WebGPU is required.
