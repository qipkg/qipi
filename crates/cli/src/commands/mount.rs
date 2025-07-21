use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct MountCommand {}

impl Command for MountCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
