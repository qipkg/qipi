use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct ShellCommand {}

impl Command for ShellCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
