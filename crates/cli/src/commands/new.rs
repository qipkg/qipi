use crate::Command;
use clap::Args;

use std::{
    fs::{create_dir, write},
    path::Path,
};
use utils::{error, info, logger::Logger};

#[derive(Debug, Args)]
pub(crate) struct NewCommand {
    path: String,
}

impl Command for NewCommand {
    fn run(&self) -> Result<(), ()> {
        let logger = Logger::new();

        let path = Path::new(&self.path);

        if path.exists() {
            error!(logger, "Directory '{}' already exists", self.path);
            return Err(());
        }

        if let Err(e) = create_dir(path) {
            error!(logger, "Error creating folder in 'new' command: {e}");
            return Err(());
        }

        info!(logger, "The project folder was created");

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unnamed");

        let content = format!(
            r###"{{
  "name": "{name}",
  "version": "1.0.0",
  "dependencies": {{}}
}}"###,
            name = name
        );

        if let Err(e) = write(path.join("package.json"), content) {
            error!(logger, "Error writing package.json in 'new' command: {e}");
        } else {
            info!(logger, "The package.json file was written")
        };

        if let Err(e) = write(path.join("package.lock"), b"") {
            error!(logger, "Error creating package.lock in 'new' command: {e}");
        } else {
            info!(logger, "The package.lock file was written");
        }

        Ok(())
    }
}
