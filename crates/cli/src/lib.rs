mod cmds;
mod utils;

use clap::{Parser, Subcommand};
use utils::parse_package;

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
            for package in packages {
                let result = parse_package(package).unwrap();
                println!("{:?}", result);
            }
        },
        Commands::Remove { global, packages } => {}
        Commands::Install {  } => {}
        Commands::Uninstall {  } => {}
    }
}
