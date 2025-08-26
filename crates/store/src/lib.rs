use client::registry::PackageVersion;
use flate2::read::GzDecoder;
use reqwest::Client;
use tar::Archive;

use futures::future::join_all;
use futures_util::StreamExt;
use tokio::{fs::File as TokioFile, io::AsyncWriteExt, sync::Semaphore, task::spawn_blocking};

use std::{
    error::Error,
    fs::{File, create_dir_all, metadata, read_dir, remove_dir_all, remove_file, rename},
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};
use utils::logger::*;

pub struct Store {
    pub store_path: PathBuf,
    pub client: Arc<Client>,
    pub download_semaphore: Arc<Semaphore>,
    pub extract_semaphore: Arc<Semaphore>,
}

impl Store {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let store_path = home_dir.join(".qipi").join("store");

        if !store_path.exists() {
            create_dir_all(&store_path).unwrap();
            info("Store directory created", false);
        }

        Self {
            store_path,
            client: Arc::new(Self::create_client()),
            download_semaphore: Arc::new(Semaphore::new(50)),
            extract_semaphore: Arc::new(Semaphore::new(20)),
        }
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

    #[allow(dead_code)]
    async fn download(
        &self,
        url: &str,
        package_path: &Path,
    ) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        let response = self.client.get(url).send().await?;

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

    #[allow(dead_code)]
    async fn extract(
        &self,
        tarball_path: PathBuf,
        package_path: PathBuf,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
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

    pub async fn install_packages(&self, packages: Vec<PackageVersion>) -> Vec<String> {
        let packages_to_install: Vec<_> = packages
            .into_iter()
            .filter(|pkg| {
                let package_key = format!("{}@{}", pkg.name, pkg.version);
                !self.store_path.join(&package_key).exists()
            })
            .collect();

        if packages_to_install.is_empty() {
            return Vec::new();
        }

        let futures = packages_to_install.into_iter().map(|pkg| self.download_extract(pkg));
        let results = join_all(futures).await;
        results.into_iter().flatten().collect()
    }

    async fn download_extract(&self, package: PackageVersion) -> Option<String> {
        let package_key = format!("{}@{}", package.name, package.version);
        let package_path = self.store_path.join(&package_key);

        if create_dir_all(&package_path).is_err() {
            return None;
        }

        let download_result = self.download(&package.dist.tarball, &package_path).await;

        match download_result {
            Ok(tarball_path) => {
                let extract_result = self.extract(tarball_path, package_path).await;

                match extract_result {
                    Ok(_) => Some(package_key),
                    Err(_) => {
                        let _ = remove_dir_all(self.store_path.join(&package_key));
                        None
                    }
                }
            }
            Err(_) => {
                let _ = remove_dir_all(&package_path);
                None
            }
        }
    }

    pub async fn add_packages(&self, packages: Vec<PackageVersion>) -> Vec<String> {
        self.install_packages(packages).await
    }

    pub async fn add_package(&self, package: PackageVersion) {
        self.install_packages(vec![package]).await;
    }

    pub fn remove(&self, name: String, version: String) {
        let package_key = format!("{name}@{version}");

        let package_path = self.store_path.join(&package_key);
        if !package_path.exists() {
            error(format!("The package {package_key} does not exist in the store"), false);
            return;
        }

        remove_dir_all(package_path).unwrap();

        success(format!("The package {package_key} was removed from store"), false);
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

        success("Store cleared", false);
    }

    pub fn list(&self) -> Vec<(String, String, Option<String>)> {
        let mut packages = vec![];

        for entry in read_dir(&self.store_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let metadata = metadata(&path).unwrap();

            let timestamp = metadata
                .created()
                .or_else(|_| metadata.modified())
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs().to_string());

            let file_name = entry.file_name().to_string_lossy().to_string();
            if let Some(pos) = file_name.rfind('@') {
                let name = &file_name[..pos];
                let version = &file_name[pos + 1..];
                packages.push((name.to_string(), version.to_string(), timestamp));
            }
        }

        packages
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
