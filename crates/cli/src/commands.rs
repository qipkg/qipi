use client::{create_client, NpmPackage};
use futures::future::join_all;

use crate::parser::Package;

pub fn install_command() {
    todo!("Install dependencies");
}

pub fn update_command() {
    todo!("Update dependencies");
}

pub fn run_command(_script: String) {
    todo!("Run a script");
}

pub fn list_command() {
    todo!("List dependencies");
}

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

    let responses = join_all(requests).await;
    for response in responses {
        match response {
            Ok(res) => {
                let package: NpmPackage = res.json().await.unwrap();

                println!("Package: {}", package.name);
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

pub fn remove_command(package: String) {
    let package = Package::parse(&package);

    match package {
        Ok(package) => {
            println!("{:?}", package);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn init_command(_name: Option<String>) {
    todo!("Create a new project");
}
