mod commands;
mod macros;

use clap::Parser;

#[derive(Parser)]
struct Cli {}

fn main() {
    let cli = Cli::parse();
}
