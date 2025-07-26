use crate::Command;
use clap::Args;

use std::{env::current_dir, fs::write, path::Path};
use utils::{error, info, logger::Logger};

#[derive(Debug, Args)]
pub(crate) struct InitCommand {
    #[clap(short, long, default_value_t = false)]
    workspace: bool,
}

impl Command for InitCommand {
    fn run(&self) -> Result<(), ()> {
        let logger = Logger::new();

        let path = Path::new(".");

        let workspace_json_path = path.join("workspace.json");

        if self.workspace && !workspace_json_path.exists() {
            if let Err(e) = write(workspace_json_path, b"{}") {
                error!(logger, "Error writing workspace.json in 'init' command: {e}");
            }

            info!(logger, "The workspace.json file was written");
        }

        let name = current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "unnamed".into());

        let content = format!(
            r###"{{
  "name": "{name}",
  "version": "1.0.0",
  "dependencies": {{}}
}}"###,
            name = name
        );

        let package_json_path = path.join("package.json");

        if !package_json_path.exists() {
            if let Err(e) = write(package_json_path, content) {
                error!(logger, "Error writing package.json in 'init' command: {e}");
            }

            info!(logger, "The package.json file was written");
        }

        let package_lock_path = path.join("package.lock");

        if !package_lock_path.exists() {
            if let Err(e) = write(package_lock_path, b"") {
                error!(logger, "Error creating package.lock in 'init' command: {e}");
            }

            info!(logger, "The package.lock file was written");
        }

        Ok(())
    }
}
