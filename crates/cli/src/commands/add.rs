use crate::Command;
use async_trait::async_trait;

use clap::Args;

use client::versions::RequestPackage;
use resolver::semver;
use utils::logger::*;

#[derive(Debug, Args)]
pub(crate) struct AddCommand {
    packages: Vec<String>,
}

#[async_trait]
impl Command for AddCommand {
    async fn run(&self) -> Result<(), ()> {
        for package in &self.packages {
            info(format!("Adding package {package}"), false);

            let req_package = Self::parse_package_str(package.clone());
            let package_versions = req_package.get_package_versions().await;

            if package_versions.is_empty() {
                error(format!("No versions were found for {}", req_package.name), false);
                continue;
            }

            let available: Vec<&str> = package_versions.iter().map(|(v, _)| v.as_str()).collect();

            let wanted = req_package.version.clone().unwrap_or("latest".to_string());
            match semver::select_version(&wanted, available) {
                Some(selected_version) => {
                    info(format!("Selected {}@{}", req_package.name, selected_version), false)
                }

                None => {
                    let error_msg = format!(
                        "Could not resolve a version for {} with range {}",
                        req_package.name, wanted
                    );

                    error(error_msg, false);
                }
            }
        }

        Ok(())
    }
}

impl AddCommand {
    fn parse_package_str(package: String) -> RequestPackage {
        let (name, version) = if package.starts_with('@') {
            if let Some(pos) = package.rfind('@') {
                let name = &package[..pos];
                let version = &package[pos + 1..];
                (name.to_string(), version.to_string())
            } else {
                (package.clone(), "latest".to_string())
            }
        } else {
            let parts: Vec<&str> = package.splitn(2, '@').collect();
            let name = parts[0].to_string();
            let version = if parts.len() > 1 { parts[1].to_string() } else { "latest".to_string() };
            (name, version)
        };

        RequestPackage { name, version: Some(version) }
    }
}
