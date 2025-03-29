use cli::Package;
use once_cell::sync::Lazy;
use reqwest::{header, Client, ClientBuilder};
use std::time::Duration;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .tcp_nodelay(true)
        .http2_prior_knowledge()
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
    // todo: build name and version correctly
    let package_name = &package.name;
    let package_version = "latest";

    let url = format!("https://registry.npmjs.org/{package_name}/{package_version}/");
    let response = HTTP_CLIENT.get(&url).send().await?.text().await?;
    Ok(response)
}
