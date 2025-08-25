use client::registry::PackageVersion;
use flate2::read::GzDecoder;
use tar::Archive;

use std::{
    fs::{File, create_dir_all, remove_file},
    io::{BufReader, copy},
    path::PathBuf,
};
use utils::logger::*;

pub struct Store {
    pub store_path: PathBuf,
}

impl Store {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        let store_path = home_dir.join(".qipi").join("store");

        if store_path.exists() {
            return Self { store_path };
        }

        create_dir_all(&store_path).unwrap();
        info("Store directory created", false);

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

        remove_file(tarball_path).unwrap();

        success(format!("Package {name}@{version} installed"), false);
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
