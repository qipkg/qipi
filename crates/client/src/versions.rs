use crate::registry::{PackageVersion, RegistryPackage};

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use once_cell::sync::Lazy;

use tokio::sync::RwLock;

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

        let registry_url = format!("https://registry.npmjs.com/{}", self.name);
        let req_get = reqwest::get(registry_url).await;

        let mut versions: Vec<(String, PackageVersion)> = vec![];
        match req_get {
            Ok(res) => {
                let res_json: RegistryPackage = res.json().await.unwrap();
                for (version_str, pkg_version) in res_json.versions.iter() {
                    versions.push((version_str.clone(), pkg_version.clone()));
                }
            }
            Err(_err) => {}
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
