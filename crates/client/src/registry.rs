use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct RegistryPackage {
    pub name: String,
    pub versions: HashMap<String, PackageVersion>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackageVersion {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<HashMap<String, String>>,
    pub dist: DistInfo,
    #[serde(default)]
    pub engines: Option<HashMap<String, String>>,
    #[serde(default)]
    pub bin: Option<HashMap<String, String>>,
    #[serde(default)]
    pub deprecated: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DistInfo {
    pub tarball: String,
    pub shasum: String,
    pub integrity: Option<String>,
    #[serde(rename = "fileCount", default)]
    pub file_count: Option<u64>,
    #[serde(rename = "unpackedSize", default)]
    pub unpacked_size: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VersionedPackage(pub PackageVersion);
