use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct MountCommand {}

#[async_trait]
impl Command for MountCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
