use once_cell::sync::Lazy;
use reqwest::{header, Client, ClientBuilder};
use shared::Package;
use std::time::Duration;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .tcp_nodelay(true)
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(3))
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(header::USER_AGENT, header::HeaderValue::from_static("qipi"));
            headers
        })
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("failed to create http client")
});

pub async fn fetch_package(package: &Package) -> Result<String, reqwest::Error> {
    let url = build_registry_url(package);
    let response = HTTP_CLIENT.get(&url).send().await?.text().await?;
    Ok(response)
}

fn build_registry_url(package: &Package) -> String {
    let scope = package
        .author
        .as_ref()
        .map(|a| format!("@{}/", a))
        .unwrap_or_default();

    let version = package
        .version
        .as_ref()
        .map(|v| v.complete.clone())
        .unwrap_or_else(|| "latest".to_string());

    format!(
        "https://registry.npmjs.org/{}{}/{}",
        scope, package.name, version
    )
}
