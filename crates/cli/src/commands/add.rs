use crate::{Command, utils::parse_package_str};
use async_trait::async_trait;

use clap::Args;

use std::time::Instant;

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
        let start = Instant::now();

        let store = Store::new();

        let requested_packages: Vec<_> =
            self.packages.iter().map(|pkg| parse_package_str(pkg.clone())).collect();

        let (missing_packages, existing_count) =
            store.filter_missing_packages(&requested_packages).await;

        if missing_packages.is_empty() {
            success("All packages already installed", false);
            let duration = start.elapsed();
            success(format!("Finished in: {duration:.2?}"), false);
            return Ok(());
        }

        if existing_count > 0 {
            info(format!("Skipping {existing_count} already installed packages"), false);
        }

        let builder = DAGBuilder::new();
        let resolution_results = builder.build_missing_only(missing_packages).await;

        let installed = store.add_packages(resolution_results).await;

        if !installed.is_empty() {
            success(format!("Installed {} packages", installed.len()), false);
        }

        let duration = start.elapsed();
        success(format!("Finished in: {duration:.2?}"), false);

        Ok(())
    }
}
