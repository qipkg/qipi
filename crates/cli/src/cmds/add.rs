use super::Command;

pub struct AddCommand {}

impl Command for AddCommand {
    async fn execute() {
        println!("hello world");
    }
    async fn resolve_dependencies() {}
    async fn update_manifests() {}
    async fn scripts() {}
    async fn validate() {}
}
