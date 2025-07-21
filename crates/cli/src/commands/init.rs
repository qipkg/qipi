use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct InitCommand {}

impl Command for InitCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
