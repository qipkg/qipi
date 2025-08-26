use client::registry::PackageVersion;
use flate2::read::GzDecoder;
use tar::Archive;

use std::{
    fs::{File, create_dir_all, metadata, read_dir, remove_dir_all, remove_file, rename},
    io::{BufReader, copy},
    path::PathBuf,
    time::UNIX_EPOCH,
};
use utils::logger::*;

pub struct Store {
    pub store_path: PathBuf,
}

impl Store {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let store_path = home_dir.join(".qipi").join("store");

        if !store_path.exists() {
            create_dir_all(&store_path).unwrap();
            info("Store directory created", false);
        }

        Self { store_path }
    }

    pub async fn add_package(&self, package: PackageVersion) {
        let name = package.name;
        let version = package.version;

        let package_path = &self.store_path.join(format!("{name}@{version}"));

        if package_path.exists() {
            return;
        }

        create_dir_all(package_path).unwrap();

        self.download_tarball(name, version, package.dist.tarball).await;
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

    async fn download_tarball(&self, name: String, version: String, url: String) {
        let package_path = &self.store_path.join(format!("{name}@{version}"));

        let res = reqwest::get(url).await.unwrap();
        let bytes = res.bytes().await.unwrap();
        let tarball_path = package_path.join("tarball.tgz");

        let mut dest = File::create(&tarball_path).unwrap();
        copy(&mut bytes.as_ref(), &mut dest).unwrap();

        let tar_gz = File::open(&tarball_path).unwrap();
        let tar = GzDecoder::new(BufReader::new(tar_gz));
        let mut archive = Archive::new(tar);
        archive.unpack(package_path).unwrap();

        let inner = package_path.join("package");
        if inner.exists() {
            for entry in read_dir(&inner).unwrap() {
                let entry = entry.unwrap();
                let dest = package_path.join(entry.file_name());
                rename(entry.path(), dest).unwrap();
            }
            remove_dir_all(inner).unwrap();
        }

        remove_file(tarball_path).unwrap();

        success(format!("Package {name}@{version} installed"), false);
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
