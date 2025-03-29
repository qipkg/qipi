use crate::cmds::Command;
use crate::parsers::{parse_package, Package};

pub struct AddCommand {
    pub global: bool,
    pub packages: Vec<String>,
}

impl AddCommand {
    pub fn new(global: bool, packages: Vec<String>) -> Self {
        Self { global, packages }
    }
}

impl Command for AddCommand {
    async fn execute(&self) {
        let parsed_packages: Result<Vec<Package>, ()> = self
            .packages
            .iter()
            .map(|package| parse_package(package.to_string()).map_err(|_| ()))
            .collect();

        match parsed_packages {
            Ok(packages) => {}
            Err(_) => {}
        }
    }
    async fn resolve_dependencies(&self) {}
    async fn update_manifests(&self) {}
    async fn scripts(&self) {}
    async fn validate(&self) {}
}
