pub trait Command {
    async fn execute() {}
    async fn resolve_dependencies() {}
    async fn update_manifests() {}
    async fn scripts() {}
    async fn validate() {}
}

mod add;
mod remove;
mod install;
mod uninstall;

pub use add::AddCommand;
pub use remove::RemoveCommand;
pub use install::InstallCommand;
pub use uninstall::UninstallCommand;
