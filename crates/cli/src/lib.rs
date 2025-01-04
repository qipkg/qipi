mod commands;
mod parser;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "qipi",
    version = "0.1.0",
    author = "Nehuén <github.com/nehu3n>"
)]
struct Args {
    #[clap(short, long)]
    verbose: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install dependencies
    Install,

    /// Update dependencies
    Update,

    /// Run a script
    Run {
        /// Script to run
        script: String,
    },

    /// List dependencies
    List,

    /// Add a dependency
    Add {
        /// Dependency to add
        packages: Vec<String>,

        /// It's a dev dependency
        #[clap(short, long)]
        dev: bool,

        /// It's a peer dependency
        #[clap(short, long)]
        peer: bool,

        /// It's a optional dependency
        #[clap(short, long)]
        optional: bool,
    },

    /// Remove a dependency
    Remove {
        /// Dependency to remove
        package: String,
    },

    /// Create a new project
    Init {
        /// Project name
        name: Option<String>,
    },
}

pub async fn init() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Install) => commands::install_command(),
        Some(Commands::Update) => commands::update_command(),
        Some(Commands::Run { script }) => commands::run_command(script),
        Some(Commands::List) => commands::list_command(),
        Some(Commands::Add {
            packages,
            dev,
            peer,
            optional,
        }) => commands::add_command(packages, dev, peer, optional).await,
        Some(Commands::Remove { package }) => commands::remove_command(package),
        Some(Commands::Init { name }) => commands::init_command(name),
        None => {}
    }
}
