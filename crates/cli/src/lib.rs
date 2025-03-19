mod cmds;

use clap::{Parser, Subcommand};

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
    let _args = Args::parse();
}
