mod response;
pub use response::NpmPackage;

use reqwest::{header, Client, ClientBuilder};

pub fn create_client() -> Client {
    let client = ClientBuilder::new()
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(header::USER_AGENT, "Qipi/1.0".parse().unwrap());
            headers
        })
        .build();

    client.unwrap()
}