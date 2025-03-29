use super::Command;

pub struct InstallCommand {}

impl Command for InstallCommand {
    async fn execute(&self) {}
    async fn resolve_dependencies(&self) {}
    async fn update_manifests(&self) {}
    async fn scripts(&self) {}
    async fn validate(&self) {}
}
