use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct NewCommand {}

impl Command for NewCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
