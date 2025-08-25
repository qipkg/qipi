mod commands;
mod macros;

use async_trait::async_trait;
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
    Store(StoreCommand),
}

#[async_trait]
impl Command for Commands {
    async fn run(&self) -> Result<(), ()> {
        match self {
            Commands::Init(cmd) => cmd.run().await?,
            Commands::New(cmd) => cmd.run().await?,
            Commands::Add(cmd) => cmd.run().await?,
            Commands::Remove(cmd) => cmd.run().await?,
            Commands::Install(cmd) => cmd.run().await?,
            Commands::Uninstall(cmd) => cmd.run().await?,
            Commands::Shell(cmd) => cmd.run().await?,
            Commands::Mount(cmd) => cmd.run().await?,
            Commands::Umount(cmd) => cmd.run().await?,
            Commands::Lock(cmd) => cmd.run().await?,
            Commands::List(cmd) => cmd.run().await?,
            Commands::Store(cmd) => cmd.run().await?,
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        if cmd.run().await.is_err() {
            std::process::exit(1);
        }
    }
}
