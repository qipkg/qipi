use crate::Command;
use clap::Args;

use std::{
    fs::{create_dir, write},
    path::Path,
};
use utils::logger::*;

#[derive(Debug, Args)]
pub(crate) struct NewCommand {
    path: String,
}

impl Command for NewCommand {
    fn run(&self) -> Result<(), ()> {
        let path = Path::new(&self.path);

        if path.exists() {
            error(format!("Directory '{0}' already exists", self.path), false);
            return Err(());
        }

        create_dir(path).expect("error creating folder in 'new' command");

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unnamed");

        let content = format!(
            r###"{{
  "name": "{name}",
  "version": "1.0.0",
  "dependencies": {{}}
}}"###,
        );

        write(path.join("package.json"), content)
            .expect("error writing package.json in 'new' command");

        info("package.json created", false);

        write(path.join("package.lock"), b"")
            .expect("error creating package.lock in 'new' command");

        info("package.lock created", false);

        success(format!("Project '{0}' created", self.path), false);

        Ok(())
    }
}
