use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct AddCommand {}

impl Command for AddCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
