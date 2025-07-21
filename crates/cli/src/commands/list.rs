use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct ListCommand {}

impl Command for ListCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
