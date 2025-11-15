/**
 * path: /crates/xtask/src/main.rs
 * description: An xtask for building and serving the web components of the project.
 * It supports commands for development server, building web assets, and bundling the editor.
 */

use clap::{Parser, Subcommand};
use std::{env, fs, io::Write, path::{Path, PathBuf}, process::Command};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "xtask")]
struct XTask {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Dev server (web) - builds wasm and serves static
    DevWeb {
        #[arg(long, default_value_t = 5173)]
        port: u16,
    },
    /// Build wasm artifacts (release) - (stub bindgen)
    BuildWeb,
    /// Bundle editor (stub)
    BundleEditor,

    /// Export all *.md, *.rs, *.toml into a single txt with code fences
    ExportSources {
        /// Output path (defaults to docs/project_snapshot.txt)
        #[arg(long, default_value = "docs/project_snapshot.txt")]
        out: String,
    },
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

fn run_build(target_pkg: &str, release: bool) {
    let rf = env::var("RUSTFLAGS").unwrap_or_default();
    let rf = ensure_wasm_js_rustflags(rf);
    let mut cmd = Command::new("cargo");
    let mut args = vec!["build", "-p", target_pkg, "--target", "wasm32-unknown-unknown"];
    if release {
        args.insert(1, "--release");
    }
    let status = cmd.env("RUSTFLAGS", rf).args(args).status().expect("spawn cargo");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn respond_static_server(port: u16) {
    use tiny_http::{Server, Response, Method, StatusCode, Header, Request};
    use std::str::FromStr;

    fn respond_not_found(req: Request) {
        let _ = req.respond(
            Response::from_string("Not Found").with_status_code(StatusCode(404))
        );
    }

    fn respond_file(req: Request, path: PathBuf) {
        match fs::read(&path) {
            Ok(bytes) => {
                let mut resp = Response::from_data(bytes);
                match path.extension().and_then(|s| s.to_str()).unwrap_or_default() {
                    "css"  => resp.add_header(Header::from_str("Content-Type: text/css").unwrap()),
                    "js"   => resp.add_header(Header::from_str("Content-Type: application/javascript").unwrap()),
                    "wasm" => resp.add_header(Header::from_str("Content-Type: application/wasm").unwrap()),
                    "html" => resp.add_header(Header::from_str("Content-Type: text/html; charset=utf-8").unwrap()),
                    "json" => resp.add_header(Header::from_str("Content-Type: application/json").unwrap()),
                    "ron"  => resp.add_header(Header::from_str("Content-Type: text/plain; charset=utf-8").unwrap()),
                    _ => {}
                }
                let _ = req.respond(resp);
            }
            Err(_) => respond_not_found(req),
        }
    }

    let server = Server::http(("127.0.0.1", port)).expect("bind");

    println!("Dev server at http://127.0.0.1:{port}");
    println!("  - /            -> apps/editor_web/index.html");
    println!("  - /static/*    -> web/static/*");
    println!("  - /assets/*    -> assets/*");
    println!("  - /pkg/*       -> web/engine-npm/dist/* (after bindgen)");

    for req in server.incoming_requests() {
        let url = req.url().to_string();
        let method = req.method().clone();

        if method == Method::Get && url == "/" {
            let path = PathBuf::from("apps/editor_web/index.html");
            respond_file(req, path);
            continue;
        }

        let (base, strip) = if url.starts_with("/static/") {
            ("web/static", "/static/")
        } else if url.starts_with("/assets/") {
            ("assets", "/assets/")
        } else if url.starts_with("/pkg/") {
            ("web/engine-npm/dist", "/pkg/")
        } else {
            ("", "")
        };

        if !base.is_empty() {
            let rel = &url[strip.len()..];
            let path = PathBuf::from(base).join(rel);
            respond_file(req, path);
            continue;
        }

        respond_not_found(req);
    }
}

fn run_bindgen(debug: bool) {
    // Pick debug or release output wasm path
    let (profile_dir, msg) = if debug { ("debug", "debug") } else { ("release", "release") };
    let wasm_path = format!("target/wasm32-unknown-unknown/{}/engine_wasm_api.wasm", profile_dir);

    // Ensure wasm exists (build if not)
    if !std::path::Path::new(&wasm_path).exists() {
        run_build("engine_wasm_api", !debug);
    }

    // Create output dir: web/engine-npm/dist
    let out_dir = "web/engine-npm/dist";
    let _ = std::fs::create_dir_all(out_dir);

    // Run wasm-bindgen
    let status = Command::new("wasm-bindgen")
        .args([
            "--target","web",
            "--no-typescript",
            "--out-dir", out_dir,
            &wasm_path,
        ])
        .status()
        .expect("spawn wasm-bindgen");
    if !status.success() {
        eprintln!("wasm-bindgen failed for {msg} wasm");
        std::process::exit(status.code().unwrap_or(1));
    }
}



fn ws_thread(port: u16) {
    // Non-Windows: real echo WS (placeholder for hot-reload)
    #[cfg(not(windows))]
    {
        let addr = format!("127.0.0.1:{}", port + 1);
        println!("WS listening on ws://{addr}/ws");
        std::thread::spawn(move || {
            // Echo server
            ws::listen(addr, |out| move |msg| out.send(msg)).expect("ws listen");
        });
    }

    // Windows: stub (avoid ws+miow)
    #[cfg(windows)]
    {
        println!("WS disabled on Windows for now (using stub).");
    }
}

/// Return true if path should be skipped (build outputs, VCS, etc.)
fn is_skipped_dir(p: &Path) -> bool {
    if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
        matches!(
            name,
            "target" | "dist" | "node_modules" | ".git"
        )
    } else {
        false
    }
}

/// Map extension to code fence language
fn lang_for(ext: &str) -> &'static str {
    match ext {
        "rs"    => "rust",
        "toml"  => "toml",
        "md"    => "markdown",
        "ron"   => "text", // not requested, but harmless if included later
        _       => "text",
    }
}

/// Normalize path to forward slashes for the code fence header
fn to_forward_slash(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

fn export_sources(out_path: &str) -> std::io::Result<()> {
    // Collect candidate files
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        let path = entry.path();

        // Skip directories we don't want
        if entry.file_type().is_dir() && is_skipped_dir(path) {
            // Skip walking into this directory
            continue;
        }

        if !entry.file_type().is_file() {
            continue;
        }

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or_default().to_ascii_lowercase();
        let include = matches!(ext.as_str(), "md" | "rs" | "toml");
        if include {
            // Also skip files in skipped dirs (defensive)
            if path.components().any(|c| {
                c.as_os_str().to_string_lossy().as_ref() == "target"
                    || c.as_os_str().to_string_lossy().as_ref() == "dist"
                    || c.as_os_str().to_string_lossy().as_ref() == "node_modules"
                    || c.as_os_str().to_string_lossy().as_ref() == ".git"
            }) {
                continue;
            }
            files.push(path.to_path_buf());
        }
    }

    // Deterministic order
    files.sort_by(|a, b| to_forward_slash(a).cmp(&to_forward_slash(b)));

    // Ensure parent dir for output exists
    if let Some(parent) = Path::new(out_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut out = fs::File::create(out_path)?;
    for f in &files {
        let rel = to_forward_slash(&f);
        let ext = f.extension().and_then(|s| s.to_str()).unwrap_or_default().to_ascii_lowercase();
        let lang = lang_for(&ext);

        // Header: path fence
        writeln!(out, "```path")?;
        writeln!(out, "{rel}")?;
        writeln!(out, "```")?;
        writeln!(out, "```{lang}")?;

        // Contents
        let bytes = fs::read(&f)?;
        out.write_all(&bytes)?;
        // Ensure trailing newline before closing fence
        if !bytes.ends_with(b"\n") {
            writeln!(out)?;
        }

        writeln!(out, "```")?;
        writeln!(out)?; // spacer line
    }

    println!("Exported {} files into {}", files.len(), out_path);
    Ok(())
}

fn main() {
    let args = XTask::parse();
    match args.cmd {
        Cmd::DevWeb { port } => {
            // build wasm targets (debug)
            run_build("engine_wasm_api", false);
            run_build("editor_web", false);
            run_bindgen(true); // bindgen the debug wasm

            // start ws + server
            ws_thread(port);
            respond_static_server(port);
        }
        Cmd::BuildWeb => {
            run_build("engine_wasm_api", true);
            run_bindgen(false); // bindgen the release wasm
            println!("(stub) run wasm-bindgen/wasm-opt and copy to web/engine-npm/dist");
        }
        Cmd::BundleEditor => {
            println!("(stub) build editor_web and copy to dist/editor");
        }
        Cmd::ExportSources { out } => {
            if let Err(e) = export_sources(&out) {
                eprintln!("export failed: {e}");
                std::process::exit(1);
            }
        }
    }
}