use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct UninstallCommand {}

impl Command for UninstallCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
