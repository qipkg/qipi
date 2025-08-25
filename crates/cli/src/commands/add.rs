use crate::{Command, utils::parse_package_str};
use async_trait::async_trait;

use clap::Args;

use resolver::graph::DAGBuilder;
use store::Store;
use utils::logger::*;

use std::time::Instant;

#[derive(Debug, Args)]
pub(crate) struct AddCommand {
    packages: Vec<String>,
}

#[async_trait]
impl Command for AddCommand {
    async fn run(&self) -> Result<(), ()> {
        let start = Instant::now();

        let builder = DAGBuilder::new();
        let store = Store::new();

        for package in &self.packages {
            info(format!("Adding package {package}"), false);

            let req_package = parse_package_str(package.clone());

            let graph = builder.build(req_package).await;
            let sorted_nodes = graph.lock().await.topological_sort();

            for node in &sorted_nodes {
                info(format!("Installing {node}"), false);

                if let Some(dag_node) = graph.lock().await.nodes.get(node) {
                    store.add_package(dag_node.info.clone()).await;
                }
            }
        }

        let duration = start.elapsed();
        info(format!("Finished in {duration:.2?}"), false);

        Ok(())
    }
}
