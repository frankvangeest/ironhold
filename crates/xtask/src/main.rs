use clap::{Parser, Subcommand};
use std::{env, process::Command};

#[derive(Parser)]
#[command(name = "xtask")]
struct XTask {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Dev server stub + build (WASM)
    DevWeb,
    /// Build WASM artifacts (no server)
    BuildWeb,
    /// Bundle editor (stub)
    BundleEditor,
}

fn ensure_wasm_js_rustflags(mut current: String) -> String {
    let token = r#"--cfg getrandom_backend="wasm_js""#;
    if !current.contains(token) {
        if !current.is_empty() && !current.ends_with(' ') {
            current.push(' ');
        }
        current.push_str(token);
    }
    current
}

fn main() {
    let args = XTask::parse();
    match args.cmd {
        Cmd::DevWeb => {
            // Inject RUSTFLAGS for getrandom's wasm_js backend
            let rf = env::var("RUSTFLAGS").unwrap_or_default();
            let rf = ensure_wasm_js_rustflags(rf);
            let mut cmd = Command::new("cargo");
            cmd.env("RUSTFLAGS", rf)
                .args(["build", "-p", "engine_wasm_api", "--target", "wasm32-unknown-unknown"]);
            let status = cmd.status().expect("failed to spawn cargo build");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
            println!("(stub) start static server + websocket for hot reload…");
            // TODO: serve /web/static + /assets, watch, and live-reload
        }
        Cmd::BuildWeb => {
            let rf = env::var("RUSTFLAGS").unwrap_or_default();
            let rf = ensure_wasm_js_rustflags(rf);
            let status = Command::new("cargo")
                .env("RUSTFLAGS", rf)
                .args(["build", "-p", "engine_wasm_api", "--release", "--target", "wasm32-unknown-unknown"])
                .status()
                .expect("failed to spawn cargo build");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
            println!("(stub) run wasm-bindgen/wasm-opt and copy to web/engine-npm/dist");
        }
        Cmd::BundleEditor => {
            println!("(stub) build editor_web and copy to dist/editor");
        }
    }
}