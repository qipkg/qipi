mod parser;

use clap::{Parser, Subcommand};
use parser::Package;

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
        package: String,

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

pub fn init() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Install) => todo!("Install dependencies"),
        Some(Commands::Update) => todo!("Update dependencies"),
        Some(Commands::Run { script: _ }) => todo!("Run a script"),
        Some(Commands::List) => todo!("List dependencies"),
        Some(Commands::Add {
            package,
            dev: _,
            peer: _,
            optional: _,
        }) => {
            let package = Package::parse(&package);

            match package {
                Ok(package) => {
                    println!("{:?}", package);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Some(Commands::Remove { package }) => {
            let package = Package::parse(&package);

            match package {
                Ok(package) => {
                    println!("{:?}", package);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Some(Commands::Init { name: _ }) => todo!("Create a new project"),
        None => {}
    }
}
