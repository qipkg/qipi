use crate::cmds::Command;
use crate::parsers::parse_package;
use client::fetch_package;
use shared::Package;

use futures::stream::FuturesUnordered;
use futures::StreamExt;

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
            Ok(packages) => {
                let mut futures = FuturesUnordered::new();

                for package in &packages {
                    futures.push(Box::pin(fetch_package(package)));
                }

                while let Some(result) = futures.next().await {
                    match result {
                        Ok(data) => println!("fetched: {}", data),
                        Err(err) => eprintln!("error fetching package: {}", err),
                    }
                }
            }
            Err(_) => {
                eprintln!("error parsing packages");
            }
        }
    }

    async fn resolve_dependencies(&self) {}
    async fn update_manifests(&self) {}
    async fn scripts(&self) {}
    async fn validate(&self) {}
}
