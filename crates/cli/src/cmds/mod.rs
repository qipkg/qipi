pub trait Command {
    async fn execute(&self) {}
    async fn resolve_dependencies(&self) {}
    async fn update_manifests(&self) {}
    async fn scripts(&self) {}
    async fn validate(&self) {}
}

mod add;
mod install;
mod remove;
mod uninstall;

pub use add::AddCommand;
pub use install::InstallCommand;
pub use remove::RemoveCommand;
pub use uninstall::UninstallCommand;
