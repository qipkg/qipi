use client::NpmPackage;
use reqwest::Client;
use semver::{Version, VersionReq};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct DependencyGraph {
    nodes: HashMap<String, ResolvedPackage>,
    roots: HashSet<String>,
}

#[derive(Debug)]
pub struct ResolvedPackage {
    pub package: NpmPackage,
    pub id: String,
    pub dependencies: HashMap<String, String>,
    pub dependent_packages: HashSet<String>,
}

#[derive(Debug)]
pub enum ResolverError {
    VersionConflict {
        package: String,
        requested: String,
        existing: String,
    },
    CyclicDependency {
        cycle: Vec<String>,
    },
    MissingPackage {
        name: String,
        version: String,
    },
    InvalidVersion(String),
    NetworkError(String),
}

pub struct DependencyResolver {
    client: Client,
    package_map: HashMap<String, NpmPackage>,
    graph: DependencyGraph,
    resolution_stack: Vec<String>,
}

impl DependencyResolver {
    pub fn new(client: Client, packages: Vec<NpmPackage>) -> Self {
        let mut package_map = HashMap::new();
        for package in packages {
            package_map.insert(format!("{}@{}", package.name, package.version), package);
        }

        Self {
            client,
            package_map,
            graph: DependencyGraph {
                nodes: HashMap::new(),
                roots: HashSet::new(),
            },
            resolution_stack: Vec::new(),
        }
    }

    pub async fn resolve_dependencies(&mut self) -> Result<&DependencyGraph, ResolverError> {
        let root_ids: Vec<String> = self.package_map.keys().cloned().collect();

        for root_id in root_ids {
            self.resolve_package(&root_id, true).await?;
        }

        Ok(&self.graph)
    }

    fn resolve_package<'a>(
        &'a mut self,
        package_id: &'a str,
        is_root: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), ResolverError>> + 'a>> {
        Box::pin(async move {
            if self.graph.nodes.contains_key(package_id) {
                return Ok(());
            }

            if self.resolution_stack.contains(&package_id.to_string()) {
                return Err(ResolverError::CyclicDependency {
                    cycle: self.resolution_stack.clone(),
                });
            }

            let package = self
                .package_map
                .get(package_id)
                .ok_or_else(|| ResolverError::MissingPackage {
                    name: package_id.to_string(),
                    version: "unknown".to_string(),
                })?
                .clone();

            self.resolution_stack.push(package_id.to_string());

            let mut dependencies = HashMap::new();
            if let Some(deps) = package.dependencies.clone() {
                for (dep_name, dep_version) in deps {
                    let dep_id = self.resolve_dependency(&dep_name, &dep_version).await?;
                    dependencies.insert(dep_name, dep_id);
                }
            }

            self.resolution_stack.pop();

            let resolved = ResolvedPackage {
                package: package.clone(),
                id: package_id.to_string(),
                dependencies: dependencies.clone(),
                dependent_packages: HashSet::new(),
            };

            if is_root {
                self.graph.roots.insert(package_id.to_string());
            }
            
            self.graph.nodes.insert(package_id.to_string(), resolved);

            for dep_id in dependencies.values() {
                if let Some(dep) = self.graph.nodes.get_mut(dep_id) {
                    dep.dependent_packages.insert(package_id.to_string());
                }
            }
            Ok(())
        })
    }

    async fn resolve_dependency(
        &mut self,
        name: &str,
        version_req: &str,
    ) -> Result<String, ResolverError> {
        if let Some(existing_id) = self.find_compatible_resolved(name, version_req) {
            return Ok(existing_id);
        }

        let req = VersionReq::parse(version_req)
            .map_err(|_| ResolverError::InvalidVersion(version_req.to_string()))?;

        let matching_packages: Vec<_> = self
            .package_map
            .iter()
            .filter(|(_, p)| p.name == name)
            .filter(|(_, p)| {
                Version::parse(&p.version)
                    .map(|v| req.matches(&v))
                    .unwrap_or(false)
            })
            .map(|(id, _)| id.clone())
            .collect();

        if let Some(id) = matching_packages
            .iter()
            .max_by_key(|id| Version::parse(&self.package_map[id.as_str()].version).unwrap())
        {
            self.resolve_package(id, false).await?;
            Ok(id.to_string())
        } else {
            let package = self.download_package(name, version_req).await?;
            let package_id = format!("{}@{}", package.name, package.version);
            self.package_map.insert(package_id.clone(), package);
            self.resolve_package(&package_id, false).await?;
            Ok(package_id)
        }
    }

    async fn download_package(
        &self,
        name: &str,
        version: &str,
    ) -> Result<NpmPackage, ResolverError> {
        let base_url = if name.starts_with('@') {
            format!("https://registry.npmjs.org/{}", name)
        } else {
            format!("https://registry.npmjs.org/{}", name)
        };

        let url = if version == "latest" {
            format!("{}/latest", base_url)
        } else {
            let metadata: serde_json::Value = self
                .client
                .get(&base_url)
                .send()
                .await
                .map_err(|e| ResolverError::NetworkError(e.to_string()))?
                .json()
                .await
                .map_err(|e| ResolverError::NetworkError(e.to_string()))?;

            let req = VersionReq::parse(version)
                .map_err(|_| ResolverError::InvalidVersion(version.to_string()))?;

            let versions = metadata["versions"].as_object().ok_or_else(|| {
                ResolverError::NetworkError("Invalid package metadata".to_string())
            })?;

            let best_version = versions
                .keys()
                .filter(|v| {
                    Version::parse(v)
                        .map(|ver| req.matches(&ver))
                        .unwrap_or(false)
                })
                .max_by_key(|v| Version::parse(v).unwrap())
                .ok_or_else(|| ResolverError::MissingPackage {
                    name: name.to_string(),
                    version: version.to_string(),
                })?;

            format!("{}/{}", base_url, best_version)
        };

        let package = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ResolverError::NetworkError(e.to_string()))?
            .json::<NpmPackage>()
            .await
            .map_err(|e| ResolverError::NetworkError(e.to_string()))?;

        Ok(package)
    }

    fn find_compatible_resolved(&self, name: &str, version_req: &str) -> Option<String> {
        let version_req = VersionReq::parse(version_req).ok()?;

        self.graph
            .nodes
            .iter()
            .find(|(_, resolved)| {
                resolved.package.name == name
                    && Version::parse(&resolved.package.version)
                        .map(|v| version_req.matches(&v))
                        .unwrap_or(false)
            })
            .map(|(id, _)| id.clone())
    }
}

impl DependencyGraph {
    pub fn iter_installation_order(&self) -> InstallationOrderIterator {
        InstallationOrderIterator::new(self)
    }
}

pub struct InstallationOrderIterator<'a> {
    graph: &'a DependencyGraph,
    queue: VecDeque<String>,
    visited: HashSet<String>,
}

impl<'a> InstallationOrderIterator<'a> {
    fn new(graph: &'a DependencyGraph) -> Self {
        let mut iter = Self {
            graph,
            queue: VecDeque::new(),
            visited: HashSet::new(),
        };

        for (id, package) in &graph.nodes {
            if package.dependencies.is_empty() {
                iter.queue.push_back(id.clone());
            }
        }

        iter
    }
}

impl<'a> Iterator for InstallationOrderIterator<'a> {
    type Item = &'a ResolvedPackage;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(id) = self.queue.pop_front() {
            if self.visited.insert(id.clone()) {
                let package = self.graph.nodes.get(&id)?;

                for dep_id in &package.dependent_packages {
                    if let Some(dep) = self.graph.nodes.get(dep_id) {
                        if dep
                            .dependencies
                            .values()
                            .all(|dep_id| self.visited.contains(dep_id))
                        {
                            self.queue.push_back(dep_id.clone());
                        }
                    }
                }

                return Some(package);
            }
        }
        None
    }
}
