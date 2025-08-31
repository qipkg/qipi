use crate::registry::{PackageVersion, RegistryPackage};

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use once_cell::sync::Lazy;

use reqwest::Client;
use tokio::sync::RwLock;

use utils::logger::*;

#[derive(Clone)]
struct CacheEntry {
    versions: Vec<(String, PackageVersion)>,
    timestamp: Instant,
}

impl CacheEntry {
    fn is_expired(&self, ttl: Duration) -> bool {
        self.timestamp.elapsed() > ttl
    }
}

static PACKAGE_CACHE: Lazy<Arc<RwLock<HashMap<String, CacheEntry>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Clone, Debug)]
pub struct RequestPackage {
    pub name: String,
    pub version: Option<String>,
}

impl RequestPackage {
    pub async fn get_package_versions(&self) -> Vec<(String, PackageVersion)> {
        const CACHE_TTL: Duration = Duration::from_secs(300);

        {
            let cache = PACKAGE_CACHE.read().await;
            if let Some(entry) = cache.get(&self.name) {
                if !entry.is_expired(CACHE_TTL) {
                    return entry.versions.clone();
                }
            }
        }

        static CLIENT: Lazy<Client> = Lazy::new(|| {
            Client::builder()
                .tcp_keepalive(Duration::from_secs(60))
                .pool_max_idle_per_host(30)
                .pool_idle_timeout(Duration::from_secs(120))
                .timeout(Duration::from_secs(10))
                .connect_timeout(Duration::from_secs(5))
                .build()
                .unwrap_or_else(|_| Client::new())
        });

        let registry_url = format!("https://registry.npmjs.com/{}", self.name);

        let versions = match CLIENT.get(&registry_url).send().await {
            Ok(response) => match response.json::<RegistryPackage>().await {
                Ok(registry_package) => {
                    let mut versions = Vec::with_capacity(registry_package.versions.len());
                    for (version_str, pkg_version) in registry_package.versions {
                        versions.push((version_str, pkg_version));
                    }

                    versions
                }
                Err(e) => {
                    error(format!("json parse error for {}: {e}", self.name), false);
                    Vec::new()
                }
            },
            Err(err) => {
                error(format!("http error for {}: {err}", self.name), false);
                Vec::new()
            }
        };

        if !versions.is_empty() {
            let mut cache = PACKAGE_CACHE.write().await;
            let entry = CacheEntry { versions: versions.clone(), timestamp: Instant::now() };
            cache.insert(self.name.clone(), entry);

            if cache.len() > 50 {
                cache.retain(|_, entry| !entry.is_expired(CACHE_TTL));
            }
        }

        versions
    }
}
