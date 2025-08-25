use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct ListCommand {}

#[async_trait]
impl Command for ListCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
