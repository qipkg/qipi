use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct AddCommand {}

#[async_trait]
impl Command for AddCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
