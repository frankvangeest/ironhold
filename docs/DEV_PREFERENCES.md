# Development Preferences

- Use Window cmd Shell
- Generate Compact patches
- Keep features and versions local to each crate until things stabilize; avoid a big [workspace.dependencies] early on.
- Guard web-only editor code with #![cfg(target_arch = "wasm32")] to keep IDE builds quiet.
- Commit frequently with short messages tied to docs tasks (e.g., docs: add WEBGPU_SETUP / xtask: dev server & ws / engine_wasm_api: RAF loop).
- Keep documentation and task progress up to date in the markdown files. (e.g. TODO.md)
- When we have a succesfull build. We check if the documentation needs updating. 