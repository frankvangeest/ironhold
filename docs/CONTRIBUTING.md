# Contributing to Ironhold

Thanks for your interest in contributing! Ironhold is currently in an early stage and primarily maintained by one developer with AI tooling. This guide explains how to set up your environment, make changes, and submit contributions efficiently.

## Ground Rules
- Be pragmatic: prefer small, incremental changes with clear scope.
- Keep documentation in sync (update `README.md`, `docs/CHANGELOG.md`, `docs/TODO.md`, or others when relevant).
- Follow the project’s coding standards (see **[CODING_STANDARDS.md](./CODING_STANDARDS.md)**).
- For now, we use a lightweight workflow; we can formalize later as the project grows.

## Prerequisites
- **Rust (stable)** with components: `rustfmt`, `clippy`.
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`.
- **wasm-bindgen-cli** installed (in a clean shell):
  ```bat
  set RUSTFLAGS=
  cargo install wasm-bindgen-cli --version 0.2.100
  ```
- A WebGPU-capable browser (recent Chrome/Edge).

## Local Setup & Dev Workflow
1. Clone the repo and enter it.
2. Run the dev server:
   ```bat
   cargo run -p xtask -- dev-web
   ```
   This:
   - Builds the WASM crates with the proper `RUSTFLAGS`.
   - Runs `wasm-bindgen` and serves glue under `/pkg/`.
   - Serves the editor page at `http://127.0.0.1:5173`.
   - Starts a WebSocket server at `ws://127.0.0.1:5174/ws`.
3. Open `http://127.0.0.1:5173` and verify: sky-blue clear + red triangle.

## Branching & Commits
- Use short-lived branches named by area + brief topic, e.g. `engine-render/basic-pipeline`.
- Keep commits **small and focused**. Prefer descriptive messages:
  - Format: `area: concise change summary`
    - Examples:
      - `docs: add WEBGPU_SETUP notes`
      - `engine_wasm_api: fix RAF closure lifetime`
      - `xtask: dev server serves /pkg/ bindgen output`
- If a change affects behavior or build flow, update `docs/CHANGELOG.md` (under the Unreleased section) and `docs/TODO.md` as appropriate.

## Code Style & Quality
- **Rust**: follow **[CODING_STANDARDS.md](./CODING_STANDARDS.md)** and the standard Rust style.
### Rust Import Style (clarity)
- Imports should **not** be written on the same line. Use multi-line grouped imports for clarity, e.g.:
  - Preferred import style for clarity:
    ```rust
    use clap::{ // Don't write everything on the same line
      Parser,
      Subcommand
    };
    ```
- Run formatters and linters:
  ```bash
  cargo fmt --all
  cargo clippy --all -- -D warnings
  ```
  Look at the **CODING_STANDARDS.md** for more details.
- **HTML/CSS/JS**: use simple, common conventions (semantic HTML, 2-space indent, kebab-case CSS class names, ES6+ JS).
### Formatting
- Use `cargo run -p xtask -- fmt` to format via **nightly rustfmt**.
- CI/Check mode: `cargo run -p xtask -- fmt --check`.
- Builds/tests remain on **stable**; nightly is used only for rustfmt.

## Testing & Validation (early-stage)
- Manual validation:
  - Build via `xtask dev-web` and ensure the editor boots, the canvas resizes, and the triangle renders.
  - Check console logs for hot-reload WebSocket connection and asset-change messages.
- As automated tests are added (unit/integration), run them with `cargo test`.

## Adding/Changing API & Data Structures
- When introducing **RON scene/data structures**:
  - Document the schema in `docs/ARCHITECTURE.md` (API section) and add examples in `assets/`.
  - Update `README.md` **API documentation** section once the types stabilize.
- Keep JS bindings (from `engine_wasm_api`) coarse-grained; include simple usage snippets.

## Submitting a Contribution (PR Checklist)
- [ ] Branch created, changes scoped.
- [ ] Code formatted (`cargo fmt`) and linted (`cargo clippy`).
- [ ] Builds locally with `xtask dev-web`.
- [ ] Relevant docs updated (README / ARCHITECTURE / BUILD / TODO / CHANGELOG).
- [ ] Clear commit messages.
- [ ] Screenshots or short notes for UI/editor changes (optional but helpful).

## Issue Reporting
- Include environment info (OS, browser, Rust version).
- Steps to reproduce, expected vs actual behavior.
- Console logs for WebGPU/WS issues if applicable.

## Release & Versioning (temporary policy)
- Use pre-release tags like `0.1.0-pre.X` in `docs/CHANGELOG.md`.
- When a milestone is stable (e.g., RON scene loading and basic rendering), bump pre-release version and tag in git.

## Code of Conduct
- Be respectful and constructive. Early-stage collaboration relies on clear communication.

---
**Note:** This guide is intentionally lightweight and will evolve as Ironhold matures.
