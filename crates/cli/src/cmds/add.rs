use super::Command;

pub struct AddCommand {}

impl Command for AddCommand {
    async fn execute(&self) {}
    async fn resolve_dependencies(&self) {}
    async fn update_manifests(&self) {}
    async fn scripts(&self) {}
    async fn validate(&self) {}
}
