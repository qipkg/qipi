use crate::Command;
use async_trait::async_trait;

use clap::{ArgGroup, Args};

use client::versions::RequestPackage;
use store::Store;

#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args(&["remove", "clear"])
))]
pub(crate) struct StoreCommand {
    #[clap(short, long, num_args = 1.., value_name = "PACKAGE")]
    remove: Vec<String>,

    #[clap(short, long)]
    clear: bool,
}

#[async_trait]
impl Command for StoreCommand {
    async fn run(&self) -> Result<(), ()> {
        let store = Store::new();

        if self.clear {
            // todo: before deleting, confirm with a prompt y/n
            store.clear();
        }

        if !self.remove.is_empty() {
            for package in &self.remove {
                let req_package = Self::parse_package_str(package.to_owned());

                if let Some(version) = req_package.version {
                    // todo: before deleting, confirm with a prompt y/n

                    store.remove(req_package.name, version);
                } else {
                    // if there are multiple packages in $HOME/.qipi/store/name@whatever, i.e., multiple versions of the same package
                    todo!("list them all to select which ones to delete");
                }
            }
        }

        Ok(())
    }
}

impl StoreCommand {
    fn parse_package_str(package: String) -> RequestPackage {
        let (name, version) = if package.starts_with('@') {
            if let Some(pos) = package.rfind('@') {
                let name = &package[..pos];
                let version = &package[pos + 1..];
                (name.to_string(), Some(version.to_string()))
            } else {
                (package.clone(), None)
            }
        } else {
            let parts: Vec<&str> = package.splitn(2, '@').collect();
            let name = parts[0].to_string();
            let version = if parts.len() > 1 { Some(parts[1].to_string()) } else { None };
            (name, version)
        };

        RequestPackage { name, version }
    }
}
