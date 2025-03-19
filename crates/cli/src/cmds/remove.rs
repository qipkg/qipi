use super::Command;

pub struct RemoveCommand {}

impl Command for RemoveCommand {
    async fn execute() {}
    async fn resolve_dependencies() {}
    async fn update_manifests() {}
    async fn scripts() {}
    async fn validate() {}
}
