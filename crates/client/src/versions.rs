use crate::registry::{PackageVersion, RegistryPackage};

pub struct RequestPackage {
    pub name: String,
    pub version: Option<String>,
}

impl RequestPackage {
    pub async fn get_package_versions(&self) -> Vec<(String, PackageVersion)> {
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

        versions
    }
}
