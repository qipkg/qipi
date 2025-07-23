use crate::Command;
use clap::Args;

use std::{
    fs::{create_dir, write},
    path::MAIN_SEPARATOR_STR,
};

#[derive(Debug, Args)]
pub(crate) struct NewCommand {
    path: String,
}

impl Command for NewCommand {
    fn run(&self) -> Result<(), ()> {
        create_dir(&self.path).expect("error creating folder in 'new' command");

        let package_json_path = format!("{0}{MAIN_SEPARATOR_STR}package.json", self.path);

        let name = self.path.split(MAIN_SEPARATOR_STR).last().unwrap_or("unnamed");

        let content = format!(
            r###"{{
  "name": "{name}",
  "version": "1.0.0",
  "dependencies": {{}}
}}"###,
            name = name
        );

        write(package_json_path, content).expect("error writing package.json in 'new' command");

        let package_lock_path = format!("{0}{MAIN_SEPARATOR_STR}package.lock", self.path);
        write(package_lock_path, b"").expect("error creating package.lock in 'new' command");

        Ok(())
    }
}
