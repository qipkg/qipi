use client::{create_client, NpmPackage};
use futures::future::join_all;
use resolver::DependencyResolver;

use crate::parser::Package;

pub async fn add_command(packages: Vec<String>, _dev: bool, _peer: bool, _optional: bool) {
    let client = create_client();

    let packages_urls: Vec<String> = packages
        .iter()
        .filter_map(|pkg| match Package::parse(pkg) {
            Ok(package) => {
                let base_url = if let Some(author) = package.author {
                    format!("https://registry.npmjs.org/@{}/{}", author, package.name)
                } else {
                    format!("https://registry.npmjs.org/{}", package.name)
                };

                let final_url = if let Some(version) = package.version {
                    format!("{}/{}", base_url, version.complete)
                } else {
                    format!("{}/{}", base_url, "latest")
                };

                Some(final_url)
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        })
        .collect();

    let requests = packages_urls.into_iter().map(|url| {
        let client = &client;
        async move { client.get(url).send().await }
    });

    let responses: Vec<NpmPackage> = join_all(
        requests
            .into_iter()
            .map(|res| async { res.await.unwrap().json::<NpmPackage>().await.unwrap() }),
    )
    .await;

    let mut resolver = DependencyResolver::new(client, responses);
    match resolver.resolve_dependencies().await {
        Ok(graph) => {
            for resolved_package in graph.iter_installation_order() {
                println!(
                    "resolved {}@{}",
                    resolved_package.package.name, resolved_package.package.version
                );
            }
        }
        Err(e) => {
            println!("Error resolving dependencies: {:?}", e);
        }
    }
}
