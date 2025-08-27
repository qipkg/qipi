use crate::{Command, utils::parse_package_str};
use async_trait::async_trait;

use clap::Args;

use std::{collections::HashSet, fs::read_dir, time::Instant};

use resolver::graph::DAGBuilder;
use store::Store;
use utils::logger::*;

use futures::future::join_all;
use tokio::task::spawn_blocking;

#[derive(Debug, Args)]
pub(crate) struct AddCommand {
    packages: Vec<String>,
}

#[async_trait]
impl Command for AddCommand {
    async fn run(&self) -> Result<(), ()> {
        let start = Instant::now();

        let store = Store::new();
        let existing_packages = self.get_existing_packages(&store).await;

        if !existing_packages.is_empty() {
            info(format!("Skipping {} already installed packages", existing_packages.len()), false);
        }

        let resolution_futures = self.packages.iter().map(|package| async {
            let req_package = parse_package_str(package.clone());
            let builder = DAGBuilder::new();

            let graph = builder.build(req_package).await;
            let graph_guard = graph.read().await;

            let nodes: Vec<_> = graph_guard.nodes.values().cloned().collect();
            drop(graph_guard);

            (package.clone(), nodes)
        });

        let resolution_results = join_all(resolution_futures).await;

        let mut all_packages = Vec::with_capacity(200);
        let mut seen = HashSet::with_capacity(200);

        for (_, nodes) in &resolution_results {
            for node in nodes {
                let package_key = format!("{}@{}", node.info.name, node.info.version);

                if existing_packages.contains(&package_key) {
                    continue;
                }

                if seen.insert(package_key) {
                    all_packages.push(node.info.clone());
                }
            }
        }

        if all_packages.is_empty() {
            success("All packages already installed", false);
            return Ok(());
        }

        for (package, nodes) in resolution_results {
            sub_info(format!("📦 {package}: {} dependencies", nodes.len()), false);
        }

        let duration = start.elapsed();
        success(format!("Finished in: {duration:.2?}"), false);

        Ok(())
    }
}

impl AddCommand {
    async fn get_existing_packages(&self, store: &Store) -> HashSet<String> {
        let store_path = store.store_path.clone();

        spawn_blocking(move || {
            let Ok(entries) = read_dir(&store_path) else {
                return HashSet::new();
            };

            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .collect()
        })
        .await
        .unwrap_or_else(|_| HashSet::new())
    }
}
