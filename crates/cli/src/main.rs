mod commands;
mod macros;

use clap::{Parser, Subcommand};
use commands::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {}

impl Command for Commands {
    fn run(&self) -> Result<(), ()> {
        match self {}

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        if let Err(err) = cmd.run() {
            eprintln!("error: {err:?}");
            std::process::exit(1);
        }
    }
}
