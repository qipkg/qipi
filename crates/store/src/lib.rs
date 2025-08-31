use client::registry::PackageVersion;
use client::versions::RequestPackage;
use flate2::read::GzDecoder;
use reqwest::Client;
use tar::Archive;

use futures::stream::FuturesUnordered;
use futures_util::StreamExt;
use tokio::{
    fs::File as TokioFile,
    io::AsyncWriteExt,
    runtime::Runtime,
    spawn,
    sync::{RwLock, Semaphore},
    task::spawn_blocking,
};

use std::{
    collections::HashSet,
    error::Error,
    fs::{
        File, create_dir_all, metadata, read_dir, read_to_string, remove_dir_all, remove_file,
        rename, write,
    },
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant, UNIX_EPOCH},
};
use utils::logger::*;

type PackageCache = Arc<RwLock<Option<(HashSet<String>, Instant)>>>;

pub struct Store {
    pub store_path: PathBuf,
    pub client: Arc<Client>,
    pub download_semaphore: Arc<Semaphore>,
    pub extract_semaphore: Arc<Semaphore>,
    package_cache: PackageCache,
}

const STORE_CACHE_TTL: Duration = Duration::from_secs(60);

fn sanitize_package_key(key: &str) -> String {
    key.replace('/', "+")
}

fn unsanitize_name(s: &str) -> String {
    s.replace('+', "/")
}

impl Store {
    fn load_index_sync(store_path: &Path) -> Option<HashSet<String>> {
        let index_path = store_path.join(".index");
        if !index_path.exists() {
            return None;
        }

        match read_to_string(index_path) {
            Ok(content) => {
                let set = content
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                Some(set)
            }
            Err(_) => None,
        }
    }

    fn scan_store_packages_sync(store_path: &Path) -> HashSet<String> {
        let Ok(entries) = read_dir(store_path) else {
            return HashSet::new();
        };
        entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .map(|entry| {
                let fname = entry.file_name().to_string_lossy().to_string();
                if let Some(pos) = fname.rfind('@') {
                    let (name_part, version_part) = fname.split_at(pos);
                    let version = &version_part[1..];
                    let desanitized = name_part.replace('+', "/");
                    format!("{desanitized}@{version}")
                } else {
                    fname
                }
            })
            .collect()
    }

    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let store_path = home_dir.join(".qipi").join("store");

        if !store_path.exists() {
            create_dir_all(&store_path).unwrap();
            info("Store directory created", false);
        }

        let initial_cache = match Self::load_index_sync(&store_path) {
            Some(pkgs) => Some((pkgs, Instant::now())),
            None => {
                let pkgs = Self::scan_store_packages_sync(&store_path);
                if !pkgs.is_empty() {
                    let index_path = store_path.join(".index");
                    let data = pkgs.iter().cloned().collect::<Vec<_>>().join("\n");
                    let tmp = index_path.with_extension("tmp");
                    let _ = write(&tmp, data.as_bytes());
                    let _ = rename(tmp, index_path);
                }
                Some((pkgs, Instant::now()))
            }
        };

        Self {
            store_path,
            client: Arc::new(Self::create_client()),
            download_semaphore: Arc::new(Semaphore::new(50)),
            extract_semaphore: Arc::new(Semaphore::new(20)),
            package_cache: Arc::new(RwLock::new(initial_cache)),
        }
    }

    async fn get_cached_packages(&self) -> HashSet<String> {
        {
            let cache = self.package_cache.read().await;
            if let Some((packages, timestamp)) = cache.as_ref() {
                if timestamp.elapsed() < STORE_CACHE_TTL {
                    return packages.clone();
                }
            }
        }

        let packages = spawn_blocking({
            let store_path = self.store_path.clone();
            move || Self::scan_store_packages_sync(&store_path)
        })
        .await
        .unwrap_or_default();

        {
            let mut cache = self.package_cache.write().await;
            *cache = Some((packages.clone(), Instant::now()));
        }

        let index_path = self.store_path.join(".index");
        let data = packages.iter().cloned().collect::<Vec<_>>().join("\n");
        let index_path_clone = index_path.clone();
        spawn_blocking(move || {
            let tmp = index_path_clone.with_extension("tmp");
            let _ = write(&tmp, data.as_bytes());
            let _ = rename(tmp, index_path_clone);
        })
        .await
        .ok();

        packages
    }

    pub async fn filter_missing_packages(
        &self,
        requested: &[RequestPackage],
    ) -> (Vec<RequestPackage>, usize) {
        let existing_packages = self.get_cached_packages().await;
        let mut missing = Vec::new();
        let mut existing_count = 0;

        for pkg in requested {
            let package_key = if let Some(version) = &pkg.version {
                format!("{}@{version}", pkg.name)
            } else {
                let prefix = format!("{}@", pkg.name);
                let found = existing_packages.iter().any(|k| k.starts_with(&prefix));
                if found {
                    existing_count += 1;
                    continue;
                } else {
                    missing.push((*pkg).clone());
                    continue;
                }
            };
            if existing_packages.contains(&package_key) {
                existing_count += 1;
            } else {
                missing.push((*pkg).clone());
            }
        }

        (missing, existing_count)
    }

    fn create_client() -> Client {
        Client::builder()
            .tcp_keepalive(Duration::from_secs(60))
            .pool_max_idle_per_host(100)
            .pool_idle_timeout(Duration::from_secs(300))
            .timeout(Duration::from_secs(8))
            .connect_timeout(Duration::from_secs(3))
            .tcp_nodelay(true)
            .build()
            .unwrap_or_else(|_| Client::new())
    }

    pub async fn install_packages(&self, packages: Vec<PackageVersion>) -> Vec<String> {
        let existing_packages = self.get_cached_packages().await;
        let packages_to_install: Vec<_> = packages
            .into_iter()
            .filter(|pkg| {
                let package_key = format!("{}@{}", pkg.name, pkg.version);
                !existing_packages.contains(&package_key)
            })
            .collect();

        if packages_to_install.is_empty() {
            return Vec::new();
        }

        info(format!("Installing {} new packages...", packages_to_install.len()), false);

        let mut futs = FuturesUnordered::new();
        for pkg in packages_to_install.into_iter() {
            let download_sem = self.download_semaphore.clone();
            let extract_sem = self.extract_semaphore.clone();
            let s = self;
            futs.push(async move {
                let _download_permit = download_sem.acquire().await.ok()?;
                let tarball_path = s.download_package(&pkg).await.ok()?;

                drop(_download_permit);
                let _extract_permit = extract_sem.acquire().await.ok()?;
                s.extract_package(tarball_path, &pkg).await.ok()?;
                Some(format!("{}@{}", pkg.name, pkg.version))
            });
        }

        let mut installed: Vec<String> = Vec::new();
        while let Some(result) = futs.next().await {
            if let Some(k) = result {
                installed.push(k);
            }
        }

        if !installed.is_empty() {
            let mut cache = self.package_cache.write().await;
            let mut set =
                if let Some((s, _)) = cache.as_ref() { s.clone() } else { HashSet::new() };
            for k in &installed {
                set.insert(k.clone());
            }
            *cache = Some((set.clone(), Instant::now()));

            let index_path = self.store_path.join(".index");
            let data = set.into_iter().collect::<Vec<_>>().join("\n");
            let index_path_clone = index_path.clone();

            spawn_blocking(move || {
                let tmp = index_path_clone.with_extension("tmp");
                let _ = write(&tmp, data.as_bytes());
                let _ = rename(tmp, index_path_clone);
            })
            .await
            .ok();
        }

        installed
    }

    async fn download_package(
        &self,
        package: &PackageVersion,
    ) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        let package_key = format!("{}@{}", package.name, package.version);
        let sanitized_key = sanitize_package_key(&package_key);
        let package_path = self.store_path.join(&sanitized_key);

        create_dir_all(&package_path)?;

        let response = self.client.get(&package.dist.tarball).send().await?;
        if !response.status().is_success() {
            return Err("Failed to download tarball".into());
        }

        let tarball_path = package_path.join("package.tgz");
        let mut file = TokioFile::create(&tarball_path).await?;

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        file.flush().await?;
        Ok(tarball_path)
    }

    async fn extract_package(
        &self,
        tarball_path: PathBuf,
        package: &PackageVersion,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let package_key = format!("{}@{}", package.name, package.version);
        let sanitized_key = sanitize_package_key(&package_key);
        let package_path = self.store_path.join(&sanitized_key);

        spawn_blocking(move || -> Result<(), Box<dyn Error + Send + Sync>> {
            let file = File::open(&tarball_path)?;
            let decoder = GzDecoder::new(BufReader::new(file));
            let mut archive = Archive::new(decoder);

            archive.set_preserve_permissions(false);
            archive.set_preserve_mtime(false);
            archive.unpack(&package_path)?;

            let inner_path = package_path.join("package");
            if inner_path.exists() {
                for entry in read_dir(&inner_path)? {
                    let entry = entry?;
                    let dest = package_path.join(entry.file_name());
                    if !dest.exists() {
                        let _ = rename(entry.path(), dest);
                    }
                }
                let _ = remove_dir_all(inner_path);
            }

            let _ = remove_file(&tarball_path);
            Ok(())
        })
        .await?
    }

    pub async fn add_packages(&self, packages: Vec<PackageVersion>) -> Vec<String> {
        self.install_packages(packages).await
    }

    pub async fn add_package(&self, package: PackageVersion) {
        self.install_packages(vec![package]).await;
    }

    pub fn remove(&self, name: String, version: String) {
        let package_key = format!("{name}@{version}");
        let sanitized = sanitize_package_key(&package_key);
        let package_path = self.store_path.join(&sanitized);
        if package_path.exists() {
            let _ = remove_dir_all(&package_path);
            let cache = self.package_cache.clone();
            spawn(async move {
                let mut cache = cache.write().await;
                if let Some((set, _)) = cache.as_mut() {
                    set.remove(&package_key);
                }
            });
            return;
        }

        match read_dir(&self.store_path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        let _ = remove_dir_all(entry.path());
                    }
                }
                success("Store cleared", false);
            }
            Err(e) => error(format!("Failed to read store directory: {e}"), false),
        }
    }

    pub fn clear(&self) {
        if !self.store_path.exists() {
            error("Store directory does not exist", false);
            return;
        }

        let entries = read_dir(&self.store_path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                remove_dir_all(&path).unwrap();
                info(format!("Removed {}", path.display()), false);
            }
        }

        let index_path = self.store_path.join(".index");
        if index_path.exists() {
            let _ = remove_file(&index_path);
        }

        let cache = self.package_cache.clone();
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let mut cache = cache.write().await;
                *cache = Some((HashSet::new(), Instant::now()));
            });
        })
        .join()
        .unwrap();

        success("Store cleared", false);
    }

    pub fn list(&self) -> Vec<(String, String, Option<String>)> {
        let Ok(entries) = read_dir(&self.store_path) else {
            return Vec::new();
        };

        entries
            .flatten()
            .filter(|entry| entry.path().is_dir())
            .filter_map(|entry| {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if let Some(pos) = file_name.rfind('@') {
                    let (name_part, version_part) = file_name.split_at(pos);
                    let version = &version_part[1..];

                    let name = unsanitize_name(name_part);

                    let timestamp = metadata(entry.path())
                        .ok()
                        .and_then(|meta| meta.created().or_else(|_| meta.modified()).ok())
                        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                        .map(|dur| dur.as_secs().to_string());

                    Some((name.to_string(), version.to_string(), timestamp))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
