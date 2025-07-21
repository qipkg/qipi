use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct LockCommand {}

impl Command for LockCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
