use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct NpmPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dist: Dist,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub peer_dependencies: Option<HashMap<String, String>>,
    pub optional_dependencies: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize)]
pub struct Dist {
    pub tarball: String,
    pub integrity: String,
    pub shasum: String,
}
