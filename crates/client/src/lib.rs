mod response;

use reqwest::{header, Client, ClientBuilder};

pub fn create_client() -> Client {
    let client = ClientBuilder::new()
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(header::USER_AGENT, "Qipi/1.0".parse().unwrap());
            headers
        })
        .http2_prior_knowledge()
        .build();

    client.unwrap()
}