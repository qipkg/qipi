mod cmds;
mod parsers;

use clap::{Parser, Subcommand};
use cmds::{AddCommand, Command};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    pub cmds: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[clap(short, long)]
        global: bool,

        packages: Vec<String>,
    },
    Remove {
        #[clap(short, long)]
        global: bool,

        packages: Vec<String>,
    },
    Install {},
    Uninstall {},
}

pub async fn init() {
    let args = Args::parse();

    match args.cmds {
        Commands::Add { global, packages } => {
            let cmd = AddCommand::new(global, packages);
            cmd.execute().await;
        }
        Commands::Remove { global, packages } => {}
        Commands::Install {} => {}
        Commands::Uninstall {} => {}
    }
}
