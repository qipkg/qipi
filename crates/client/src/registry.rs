use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum DeprecatedField {
    Text(String),
    Bool(bool),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum BinField {
    Map(HashMap<String, String>),
    Str(String),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum EnginesField {
    Map(HashMap<String, String>),
    Str(String),
    Bool(bool),
    Seq(Vec<serde_json::Value>),
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegistryPackage {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub versions: HashMap<String, PackageVersion>,
    #[serde(rename = "dist-tags", default)]
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
    pub engines: Option<EnginesField>,
    #[serde(default)]
    pub bin: Option<BinField>,
    #[serde(default)]
    pub deprecated: Option<DeprecatedField>,
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
