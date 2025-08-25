use async_recursion::async_recursion;
use tokio::sync::Mutex;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use crate::semver;
use client::versions::RequestPackage;
use utils::logger::*;

#[derive(Debug, Clone)]
pub struct DAGNode {
    pub package: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Default)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DAGNode>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, node: DAGNode) {
        self.nodes.insert(node.package.clone(), node);
    }

    pub fn topological_sort(&self) -> Vec<String> {
        let mut indegree: HashMap<&String, usize> = HashMap::new();

        for (pkg, node) in &self.nodes {
            indegree.entry(pkg).or_insert(0);
            for dep in &node.dependencies {
                *indegree.entry(dep).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<&String> = indegree
            .iter()
            .filter_map(|(&pkg, &deg)| if deg == 0 { Some(pkg) } else { None })
            .collect();

        let mut result = Vec::new();
        while let Some(pkg) = queue.pop_front() {
            result.push(pkg.clone());
            if let Some(node) = self.nodes.get(pkg) {
                for dep in &node.dependencies {
                    if let Some(deg) = indegree.get_mut(dep) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dep);
                        }
                    }
                }
            }
        }

        result
    }
}

pub struct DAGBuilder {
    visited: Arc<Mutex<HashSet<String>>>,
    graph: Arc<Mutex<DependencyGraph>>,
}

impl DAGBuilder {
    pub fn new() -> Self {
        Self {
            visited: Arc::new(Mutex::new(HashSet::new())),
            graph: Arc::new(Mutex::new(DependencyGraph::new())),
        }
    }

    pub async fn build(&self, package: RequestPackage) -> Arc<Mutex<DependencyGraph>> {
        self.resolve_package(package).await;
        self.graph.clone()
    }

    #[async_recursion]
    async fn resolve_package(&self, package: RequestPackage) -> Option<String> {
        let name = package.name.clone();
        let version_req = package.version.clone().unwrap_or_else(|| "latest".to_string());

        let key = format!("{name}@{version_req}");

        {
            let visited = self.visited.lock().await;
            if visited.contains(&key) {
                return Some(key);
            }
        }

        let versions = package.get_package_versions().await;
        if versions.is_empty() {
            error(format!("No versions found for {name}"), false);
            return None;
        }

        let available: Vec<&str> = versions.iter().map(|(v, _)| v.as_str()).collect();
        let selected_version = semver::select_version(&version_req, available)?;

        let pkg_version = versions.into_iter().find(|(v, _)| v == &selected_version).unwrap().1;

        let mut dep_keys = Vec::new();

        if let Some(deps) = pkg_version.dependencies.clone() {
            for (dep_name, dep_range) in deps {
                let dep_pkg =
                    RequestPackage { name: dep_name.clone(), version: Some(dep_range.clone()) };
                if let Some(dep_key) = self.resolve_package(dep_pkg).await {
                    dep_keys.push(dep_key);
                }
            }
        }

        let node =
            DAGNode { package: format!("{name}@{selected_version}"), dependencies: dep_keys };

        {
            let mut graph = self.graph.lock().await;
            graph.add_node(node);
        }

        {
            let mut visited = self.visited.lock().await;
            visited.insert(key.clone());
        }

        Some(format!("{name}@{selected_version}"))
    }
}

impl Default for DAGBuilder {
    fn default() -> Self {
        Self::new()
    }
}
