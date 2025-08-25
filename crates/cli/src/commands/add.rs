use crate::Command;
use async_trait::async_trait;

use clap::Args;

use client::versions::RequestPackage;
use resolver::graph::DAGBuilder;
use store::Store;
use utils::logger::*;

#[derive(Debug, Args)]
pub(crate) struct AddCommand {
    packages: Vec<String>,
}

#[async_trait]
impl Command for AddCommand {
    async fn run(&self) -> Result<(), ()> {
        let builder = DAGBuilder::new();
        let store = Store::new();

        for package in &self.packages {
            info(format!("Adding package {package}"), false);

            let req_package = Self::parse_package_str(package.clone());
            
            let graph = builder.build(req_package).await;
            let sorted_nodes = graph.lock().await.topological_sort();

            for node in &sorted_nodes {
                info(format!("Installing {node}"), false);

                if let Some(dag_node) = graph.lock().await.nodes.get(node) {
                    store.add_package(dag_node.info.clone()).await;
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
