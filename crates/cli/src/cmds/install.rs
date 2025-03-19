use super::Command;

pub struct InstallCommand {}

impl Command for InstallCommand {
    async fn execute() {}
    async fn resolve_dependencies() {}
    async fn update_manifests() {}
    async fn scripts() {}
    async fn validate() {}
}
