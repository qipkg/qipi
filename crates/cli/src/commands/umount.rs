use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct UmountCommand {}

#[async_trait]
impl Command for UmountCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
