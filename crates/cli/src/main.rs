mod commands;
mod macros;

use clap::{Parser, Subcommand};
use commands::*;

#[derive(Parser)]
#[command(name = "qp")]
#[command(about = "Extremely fast, disk-efficient, node_modules-free, instant, and secure package manager for Node.js — written in Rust.", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init(InitCommand),
    New(NewCommand),
    Add(AddCommand),
    Remove(RemoveCommand),
    Install(InstallCommand),
    Uninstall(UninstallCommand),
    Shell(ShellCommand),
    Mount(MountCommand),
    Umount(UmountCommand),
    Lock(LockCommand),
    List(ListCommand),
}

impl Command for Commands {
    fn run(&self) -> Result<(), ()> {
        match self {
            Commands::Init(cmd) => cmd.run()?,
            Commands::New(cmd) => cmd.run()?,
            Commands::Add(cmd) => cmd.run()?,
            Commands::Remove(cmd) => cmd.run()?,
            Commands::Install(cmd) => cmd.run()?,
            Commands::Uninstall(cmd) => cmd.run()?,
            Commands::Shell(cmd) => cmd.run()?,
            Commands::Mount(cmd) => cmd.run()?,
            Commands::Umount(cmd) => cmd.run()?,
            Commands::Lock(cmd) => cmd.run()?,
            Commands::List(cmd) => cmd.run()?,
        }

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        if cmd.run().is_err() {
            std::process::exit(1);
        }
    }
}
