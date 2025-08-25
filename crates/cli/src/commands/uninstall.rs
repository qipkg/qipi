use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct UninstallCommand {}

#[async_trait]
impl Command for UninstallCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
