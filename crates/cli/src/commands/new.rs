use crate::Command;
use clap::Args;

use std::{
    fs::{create_dir, write},
    path::Path,
};

#[derive(Debug, Args)]
pub(crate) struct NewCommand {
    path: String,
}

impl Command for NewCommand {
    fn run(&self) -> Result<(), ()> {
        let path = Path::new(&self.path);

        if path.exists() {
            eprintln!("directory '{}' already exists", self.path);
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

        write(path.join("package.lock"), b"")
            .expect("error creating package.lock in 'new' command");

        Ok(())
    }
}
