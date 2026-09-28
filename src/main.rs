use std::process::exit;

use clap::Parser;
use tjcli::{TclConfig, run};

fn main() {
    if let Err(e) = run(TclConfig::parse()) {
        eprintln!("Fatal Error: {}", e.to_string());
        exit(1);
    }
}
