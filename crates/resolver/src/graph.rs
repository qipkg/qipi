use once_cell::sync::Lazy;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use futures::future::{FutureExt, join_all, ready};
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

    pub async fn build(&self, package: RequestPackage) -> Arc<RwLock<DependencyGraph>> {
        let nodes = self.internal_build(package).await;

        let mut graph = DependencyGraph::with_capacity(nodes.len());
        for (_, node) in nodes {
            graph.add_node(node);
        }

        Arc::new(RwLock::new(graph))
    }

    async fn internal_build(&self, package: RequestPackage) -> HashMap<String, DAGNode> {
        let mut all_packages = HashSet::new();
        let mut to_resolve = vec![package];

        while !to_resolve.is_empty() {
            let current_batch = std::mem::take(&mut to_resolve);

            let futures = current_batch.into_iter().map(|pkg| {
                let semaphore = self.semaphore.clone();
                async move {
                    let _permit = semaphore.acquire().await.ok()?;
                    self.resolve_single(pkg).await
                }
            });

            let results = join_all(futures).await;

            for result in results.into_iter().flatten() {
                let key = format!("{}@{}", result.info.name, result.info.version);
                if all_packages.insert(key) {
                    for dep in &result.dependencies {
                        if let Some(pos) = dep.rfind('@') {
                            let (dep_name, dep_ver_with_at) = dep.split_at(pos);
                            let dep_version = &dep_ver_with_at[1..];
                            to_resolve.push(RequestPackage {
                                name: dep_name.to_string(),
                                version: Some(dep_version.to_string()),
                            });
                        }
                    }
                }
            }
        }

        let resolve_futures: Vec<_> = all_packages
            .into_iter()
            .map(|pkg_key| {
                if let Some(pos) = pkg_key.rfind('@') {
                    let (name, ver_with_at) = pkg_key.split_at(pos);
                    let version = &ver_with_at[1..];

                    let req_pkg = RequestPackage {
                        name: name.to_string(),
                        version: Some(version.to_string()),
                    };
                    let semaphore = self.semaphore.clone();
                    async move {
                        let _permit = semaphore.acquire().await.ok()?;
                        self.resolve_single(req_pkg).await
                    }
                    .boxed()
                } else {
                    ready(None).boxed()
                }
            })
            .collect();

        let final_results = join_all(resolve_futures).await;

        let mut graph = HashMap::new();
        for node in final_results.into_iter().flatten() {
            let key = format!("{}@{}", node.info.name, node.info.version);
            graph.insert(key, node);
        }

        graph
    }

    async fn resolve_single(&self, package: RequestPackage) -> Option<DAGNode> {
        let name = &package.name;
        let version_req = package.version.as_deref().unwrap_or("latest");

        let cache_key = format!("{name}@{version_req}");
        {
            let _cache = self.resolution_cache.read().await;
            // if let Some(_) = cache.get(&cache_key) {}
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
