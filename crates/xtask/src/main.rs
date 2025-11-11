
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask")] 
struct XTask { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    DevWeb,
    BuildWeb,
    BundleEditor,
}

fn main() {
    let args = XTask::parse();
    match args.cmd {
        Cmd::DevWeb => { println!("(stub) start dev server, build wasm, watch"); }
        Cmd::BuildWeb => { println!("(stub) build wasm + wasm-bindgen + wasm-opt"); }
        Cmd::BundleEditor => { println!("(stub) build editor_web and copy to dist/editor"); }
    }
}
