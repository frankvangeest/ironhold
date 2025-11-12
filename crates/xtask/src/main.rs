use clap::{Parser, Subcommand};
use std::{env, fs, path::PathBuf, thread};

use std::str::FromStr;
use tiny_http::Header;


#[derive(Parser)]
#[command(name = "xtask")]
struct XTask { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    DevWeb {
        #[arg(long, default_value_t = 5173)]
        port: u16,
    },
    BuildWeb,
    BundleEditor,
}

fn ensure_wasm_js_rustflags(mut current: String) -> String {
    let token = r#"--cfg getrandom_backend="wasm_js""#;
    if !current.contains(token) {
        if !current.is_empty() && !current.ends_with(' ') { current.push(' '); }
        current.push_str(token);
    }
    current
}

fn run_build(target_pkg: &str, release: bool) {
    let rf = env::var("RUSTFLAGS").unwrap_or_default();
    let rf = ensure_wasm_js_rustflags(rf);
    let mut cmd = std::process::Command::new("cargo");
    let mut args = vec!["build", "-p", target_pkg, "--target", "wasm32-unknown-unknown"];
    if release { args.insert(1, "--release"); }
    let status = cmd.env("RUSTFLAGS", rf).args(args).status().expect("spawn cargo");
    if !status.success() { std::process::exit(status.code().unwrap_or(1)); }
}

fn serve_static(port: u16) {
    use tiny_http::{Server, Response, Method, StatusCode, Header, Request};
    use std::{fs, path::PathBuf};
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

                // naive content-type
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
        // 🔎 Peek into request without consuming it yet
        let url = req.url().to_string();    // snapshot
        let method = req.method().clone();  // Method (value)

        // Route selection happens here; then we consume `req` once.
        if method == Method::Get && url == "/" {
            let path = PathBuf::from("apps/editor_web/index.html");
            respond_file(req, path);
            continue;
        }

        // Static mappings
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


fn ws_thread(port: u16) {
    // Very small WS server for future hot reload. For now, accept connections.
    let addr = format!("127.0.0.1:{}", port + 1);
    println!("WS listening on ws://{addr}/ws");
    thread::spawn(move || {
        ws::listen(addr, |out| {
            move |msg| {
                // echo for now
                out.send(msg)
            }
        }).expect("ws listen")
    });
}

fn main() {
    let args = XTask::parse();
    match args.cmd {
        Cmd::DevWeb { port } => {
            // 1) build wasm api (debug)
            run_build("engine_wasm_api", false);
            // (optionally) build editor_web as well
            run_build("editor_web", false);

            // 2) start WS
            ws_thread(port);

            // 3) start static server (blocking)
            serve_static(port);
        }
        Cmd::BuildWeb => {
            run_build("engine_wasm_api", true);
            println!("(stub) run wasm-bindgen/wasm-opt and copy to web/engine-npm/dist");
        }
        Cmd::BundleEditor => {
            println!("(stub) build editor_web and copy to dist/editor");
        }
    }
}