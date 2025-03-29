mod cmds;
mod utils;

use clap::{Parser, Subcommand};
use utils::{parse_package, Package};

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
            let parsed_packages: Result<Vec<Package>, ()> = packages
                .iter()
                .map(|package| parse_package(package.to_string()).map_err(|_| ()))
                .collect();

            match parsed_packages {
                Ok(packages) => {}
                Err(_) => {}
            }
        }

        Commands::Remove { global, packages } => {}
        Commands::Install {} => {}
        Commands::Uninstall {} => {}
    }
}
