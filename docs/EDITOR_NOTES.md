# Editor Notes

- **UI framework**: egui/eframe (web). Game runtime UI is separate (retained).
- **Layout**: a top toolbar, a viewport (center), and placeholders for hierarchy/inspector/assets.
- **Mode switch**: a play/stop button toggling `Engine.set_play_mode(...)` and world/schedule selection.
- **Viewport**: show engine output as an egui image/texture.

## Planned panels

- **Hierarchy**: simple tree of entities
- **Inspector**: derived (later with reflection) editing components
- **Assets**: file browser (dev-only) backed by `/assets` static server