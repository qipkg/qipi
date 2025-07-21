use crate::Command;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct UmountCommand {}

impl Command for UmountCommand {
    fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
