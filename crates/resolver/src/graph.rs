use once_cell::sync::Lazy;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use futures::stream::FuturesUnordered;
use futures_util::StreamExt;
use tokio::sync::{RwLock, Semaphore};

use crate::semver;
use client::{registry::PackageVersion, versions::RequestPackage};

type PackageVersionsMap = HashMap<String, Vec<(String, PackageVersion)>>;
type SharedPackageCache = Arc<RwLock<PackageVersionsMap>>;

static GLOBAL_PACKAGE_CACHE: Lazy<SharedPackageCache> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

static GLOBAL_RESOLUTION_CACHE: Lazy<Arc<RwLock<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug, Clone)]
pub struct DAGNode {
    pub package: String,
    pub dependencies: Vec<String>,
    pub info: PackageVersion,
}

#[derive(Debug, Default)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DAGNode>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { nodes: HashMap::with_capacity(capacity) }
    }

    pub fn add_node(&mut self, node: DAGNode) {
        self.nodes.insert(node.package.clone(), node);
    }

    pub fn topological_sort(&self) -> Vec<String> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(self.nodes.len());
        let mut processed = HashSet::with_capacity(self.nodes.len());

        fn dfs_visit(
            pkg: &str,
            graph: &HashMap<String, DAGNode>,
            result: &mut Vec<String>,
            processed: &mut HashSet<String>,
        ) {
            if processed.contains(pkg) {
                return;
            }

            processed.insert(pkg.to_string());

            if let Some(node) = graph.get(pkg) {
                for dep in &node.dependencies {
                    dfs_visit(dep, graph, result, processed);
                }
            }

            result.push(pkg.to_string());
        }

        for pkg in self.nodes.keys() {
            dfs_visit(pkg, &self.nodes, &mut result, &mut processed);
        }

        result
    }
}

pub struct DAGBuilder {
    resolution_cache: Arc<RwLock<HashMap<String, String>>>,
    semaphore: Arc<Semaphore>,
}

impl DAGBuilder {
    pub fn new() -> Self {
        Self {
            resolution_cache: GLOBAL_RESOLUTION_CACHE.clone(),
            semaphore: Arc::new(Semaphore::new(100)),
        }
    }

    pub async fn build_missing_only(&self, packages: Vec<RequestPackage>) -> Vec<PackageVersion> {
        let mut all_resolved = Vec::new();
        let mut processed = HashSet::new();
        let mut to_process = packages;

        while !to_process.is_empty() {
            let current_batch = std::mem::take(&mut to_process);

            let mut futs = FuturesUnordered::new();
            for pkg in current_batch {
                let semaphore = self.semaphore.clone();
                let b = self;
                futs.push(async move {
                    let _permit = semaphore.acquire().await.ok()?;
                    b.resolve_single(pkg).await
                });
            }

            while let Some(res_opt) = futs.next().await {
                if let Some(result) = res_opt {
                    let key = format!("{}@{}", result.info.name, result.info.version);
                    if processed.insert(key.clone()) {
                        for dep in &result.dependencies {
                            if let Some(pos) = dep.rfind('@') {
                                let dep_name = dep[..pos].to_string();
                                let dep_version = dep[pos + 1..].to_string();
                                let dep_key = format!("{dep_name}@{dep_version}");
                                if !processed.contains(&dep_key) {
                                    to_process.push(RequestPackage {
                                        name: dep_name,
                                        version: Some(dep_version),
                                    });
                                }
                            }
                        }
                        all_resolved.push(result.info);
                    }
                }
            }
        }

        all_resolved
    }

    pub async fn build(&self, package: RequestPackage) -> Arc<RwLock<DependencyGraph>> {
        let packages = self.build_missing_only(vec![package]).await;
        let mut graph = DependencyGraph::with_capacity(packages.len());

        for package_version in packages {
            let package_key = format!("{}@{}", package_version.name, package_version.version);
            let mut dep_keys = Vec::new();
            if let Some(deps) = &package_version.dependencies {
                for (dep_name, dep_version) in deps {
                    dep_keys.push(format!("{dep_name}@{dep_version}"));
                }
            }
            if let Some(opt_deps) = &package_version.optional_dependencies {
                for (dep_name, dep_version) in opt_deps {
                    dep_keys.push(format!("{dep_name}@{dep_version}"));
                }
            }
            if let Some(peer_deps) = &package_version.peer_dependencies {
                for (dep_name, dep_version) in peer_deps {
                    dep_keys.push(format!("{dep_name}@{dep_version}"));
                }
            }
            let node =
                DAGNode { package: package_key, dependencies: dep_keys, info: package_version };
            graph.add_node(node);
        }
        Arc::new(RwLock::new(graph))
    }

    async fn resolve_single(&self, package: RequestPackage) -> Option<DAGNode> {
        let name = &package.name;
        let version_req = package.version.as_deref().unwrap_or("latest");

        let cache_key = format!("{name}@{version_req}");
        {
            let cache = self.resolution_cache.read().await;
            if let Some(final_key) = cache.get(&cache_key).cloned() {
                drop(cache);
                if let Some(pos) = final_key.rfind('@') {
                    let pkg_name = &final_key[..pos];
                    let pkg_version = &final_key[pos + 1..];
                    let versions = self.get_cached_versions(pkg_name).await;
                    if let Some((_, data)) = versions.into_iter().find(|(v, _)| v == pkg_version) {
                        let mut dep_keys: Vec<String> = Vec::new();
                        if let Some(deps) = &data.dependencies {
                            for (dep_name, dep_version) in deps {
                                dep_keys.push(format!("{dep_name}@{dep_version}"));
                            }
                        }
                        if let Some(opt_deps) = &data.optional_dependencies {
                            for (dep_name, dep_version) in opt_deps {
                                dep_keys.push(format!("{dep_name}@{dep_version}"));
                            }
                        }
                        if let Some(peer_deps) = &data.peer_dependencies {
                            for (dep_name, dep_version) in peer_deps {
                                dep_keys.push(format!("{dep_name}@{dep_version}"));
                            }
                        }
                        return Some(DAGNode {
                            package: final_key.clone(),
                            dependencies: dep_keys,
                            info: data,
                        });
                    }
                }
            }
        }

        let versions = self.get_cached_versions(name).await;
        if versions.is_empty() {
            return None;
        }

        let available: Vec<&str> = versions.iter().map(|(v, _)| v.as_str()).collect();
        let selected_version = semver::select_version(version_req, available)?;

        let pkg_version =
            versions.into_iter().find(|(v, _)| v == &selected_version).map(|(_, data)| data)?;

        let final_key = format!("{name}@{selected_version}");

        {
            let mut cache = self.resolution_cache.write().await;
            cache.insert(cache_key, final_key.clone());
        }

        let mut dep_keys: Vec<String> = Vec::new();

        if let Some(deps) = &pkg_version.dependencies {
            for (dep_name, dep_version) in deps {
                dep_keys.push(format!("{dep_name}@{dep_version}"));
            }
        }

        if let Some(opt_deps) = &pkg_version.optional_dependencies {
            for (dep_name, dep_version) in opt_deps {
                dep_keys.push(format!("{dep_name}@{dep_version}"));
            }
        }

        if let Some(peer_deps) = &pkg_version.peer_dependencies {
            for (dep_name, dep_version) in peer_deps {
                dep_keys.push(format!("{dep_name}@{dep_version}"));
            }
        }

        Some(DAGNode { package: final_key, dependencies: dep_keys, info: pkg_version })
    }

    async fn get_cached_versions(&self, name: &str) -> Vec<(String, PackageVersion)> {
        {
            let cache = GLOBAL_PACKAGE_CACHE.read().await;
            if let Some(versions) = cache.get(name) {
                return versions.clone();
            }
        }

        let req_pkg = RequestPackage { name: name.to_string(), version: None };
        let versions = req_pkg.get_package_versions().await;

        if !versions.is_empty() {
            let mut cache = GLOBAL_PACKAGE_CACHE.write().await;
            cache.insert(name.to_string(), versions.clone());
        }

        versions
    }
}

impl Default for DAGBuilder {
    fn default() -> Self {
        Self::new()
    }
}
