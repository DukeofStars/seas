use std::path::PathBuf;

use clap::Parser;
use wharf::*;

fn main() {
    let cli = Cli::parse();
    install(cli.path);
}

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}
