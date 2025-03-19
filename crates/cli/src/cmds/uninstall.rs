use super::Command;

pub struct UninstallCommand {}

impl Command for UninstallCommand {
    async fn execute() {}
    async fn resolve_dependencies() {}
    async fn update_manifests() {}
    async fn scripts() {}
    async fn validate() {}
}
