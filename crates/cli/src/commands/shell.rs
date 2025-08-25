use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct ShellCommand {}

#[async_trait]
impl Command for ShellCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
