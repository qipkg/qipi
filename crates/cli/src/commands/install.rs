use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct InstallCommand {}

#[async_trait]
impl Command for InstallCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
