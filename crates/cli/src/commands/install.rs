use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct InstallCommand {}

impl Command for InstallCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
