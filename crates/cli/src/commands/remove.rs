use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct RemoveCommand {}

impl Command for RemoveCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
