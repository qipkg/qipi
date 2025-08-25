use crate::Command;
use async_trait::async_trait;

use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct LockCommand {}

#[async_trait]
impl Command for LockCommand {
    async fn run(&self) -> Result<(), ()> {
        Ok(())
    }
}
