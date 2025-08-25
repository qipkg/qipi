use async_recursion::async_recursion;
use tokio::sync::Mutex;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use crate::semver;
use client::{registry::PackageVersion, versions::RequestPackage};
use utils::logger::*;

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

        let remaining: Vec<String> = indegree
            .iter()
            .filter_map(|(&pkg, &deg)| if deg > 0 { Some(pkg.clone()) } else { None })
            .collect();

        if !remaining.is_empty() {
            warn(
                format!(
                    "Circular dependencies detected: {remaining:?} - will be resolved at runtime"
                ),
                false,
            );
            result.extend(remaining);
        }

        result
    }

    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for package in self.nodes.keys() {
            if !visited.contains(package) {
                let mut path = Vec::new();
                if let Some(cycle) =
                    self.dfs_cycle_detection(package, &mut visited, &mut rec_stack, &mut path)
                {
                    cycles.push(cycle);
                }
            }
        }

        cycles
    }

    fn dfs_cycle_detection(
        &self,
        package: &String,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(package.clone());
        rec_stack.insert(package.clone());
        path.push(package.clone());

        if let Some(node) = self.nodes.get(package) {
            for dep in &node.dependencies {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_cycle_detection(dep, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    if let Some(cycle_start_pos) = path.iter().position(|p| p == dep) {
                        let mut cycle = path[cycle_start_pos..].to_vec();
                        cycle.push(dep.clone());
                        return Some(cycle);
                    } else {
                        warn(
                            format!(
                                "Detected cycle edge case: {package} -> {dep} (dep in rec_stack but not in path)"
                            ),
                            false,
                        );
                        return Some(vec![package.clone(), dep.clone()]);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(package);
        None
    }
}

pub struct DAGBuilder {
    visited: Arc<Mutex<HashSet<String>>>,
    graph: Arc<Mutex<DependencyGraph>>,
    resolution_path: Arc<Mutex<Vec<String>>>,
}

impl DAGBuilder {
    pub fn new() -> Self {
        Self {
            visited: Arc::new(Mutex::new(HashSet::new())),
            graph: Arc::new(Mutex::new(DependencyGraph::new())),
            resolution_path: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn build(&self, package: RequestPackage) -> Arc<Mutex<DependencyGraph>> {
        self.resolve_package(package).await;

        let graph = self.graph.lock().await;
        let cycles = graph.detect_cycles();
        if !cycles.is_empty() {
            warn(
                format!("Circular dependencies detected (will be handled at runtime): {cycles:?}"),
                false,
            );
        }
        drop(graph);

        self.graph.clone()
    }

    #[async_recursion]
    async fn resolve_package(&self, package: RequestPackage) -> Option<String> {
        let name = package.name.clone();
        let version_req = package.version.clone().unwrap_or_else(|| "latest".to_string());

        let key = format!("{name}@{version_req}");

        {
            let path = self.resolution_path.lock().await;
            if path.contains(&key) {
                warn(format!("Circular dependency detected during resolution: {key}"), false);
                return Some(key);
            }
        }

        {
            let visited = self.visited.lock().await;
            if visited.contains(&key) {
                return Some(key);
            }
        }

        {
            let mut path = self.resolution_path.lock().await;
            path.push(key.clone());
        }

        let result = async {
            let versions = package.get_package_versions().await;
            if versions.is_empty() {
                error(format!("No versions found for {name}"), false);
                return None;
            }

            let available: Vec<&str> = versions.iter().map(|(v, _)| v.as_str()).collect();
            let selected_version = semver::select_version(&version_req, available)?;

            let pkg_version = match versions.into_iter().find(|(v, _)| v == &selected_version) {
                Some((_, version_data)) => version_data,
                None => {
                    let err_msg = format!("Selected version {selected_version} for {name} not found in available versions");
                    error(err_msg, false);

                    return None;
                }
            };

            let mut dep_keys = Vec::new();

            if let Some(deps) = pkg_version.dependencies.clone() {
                for (dep_name, dep_range) in deps {
                    let dep_pkg =
                        RequestPackage { name: dep_name.clone(), version: Some(dep_range.clone()) };
                    if let Some(dep_key) = self.resolve_package(dep_pkg).await {
                        dep_keys.push(dep_key);
                    } else {
                        warn(format!("Could not resolve dependency: {dep_name}"), false);
                    }
                }
            }

            let final_key = format!("{name}@{selected_version}");
            let node = DAGNode { package: final_key.clone(), dependencies: dep_keys, info: pkg_version, };

            {
                let mut graph = self.graph.lock().await;
                graph.add_node(node);
            }

            {
                let mut visited = self.visited.lock().await;
                visited.insert(key.clone());
                visited.insert(final_key.clone());
            }

            Some(final_key)
        }
        .await;

        {
            let mut path = self.resolution_path.lock().await;
            if let Some(pos) = path.iter().rposition(|p| p == &key) {
                path.remove(pos);
            }
        }

        result
    }
}

impl Default for DAGBuilder {
    fn default() -> Self {
        Self::new()
    }
}
