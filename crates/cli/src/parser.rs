use regex::Regex;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct PackageVersion {
    pub complete: String,
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Package {
    pub author: Option<String>,
    pub name: String,
    pub version: Option<PackageVersion>,
    pub version_constraint: Option<String>,
}

impl Package {
    pub fn parse(package_str: &str) -> Result<Package, Box<dyn Error>> {
        let package_regex = Regex::new(r"^(?:@([^/]+)/)?([^@]+)(?:@(.+))?$")?;

        if let Some(captures) = package_regex.captures(package_str) {
            let author = captures.get(1).map(|m| m.as_str().to_string());
            let name = captures.get(2).map(|m| m.as_str().to_string()).unwrap();
            let version_str = captures.get(3).map(|m| m.as_str());

            let (version, version_constraint) = if let Some(v_str) = version_str {
                Self::parse_version(v_str)?
            } else {
                (None, None)
            };

            Ok(Package {
                author,
                name,
                version,
                version_constraint,
            })
        } else {
            Err("Invalid package format".into())
        }
    }

    fn parse_version(
        version_str: &str,
    ) -> Result<(Option<PackageVersion>, Option<String>), Box<dyn Error>> {
        let constraint_regex = Regex::new(r"^([~^><]=?)?(.+)$")?;
        let captures = constraint_regex
            .captures(version_str)
            .ok_or("Invalid version format")?;

        let constraint = captures.get(1).map(|m| m.as_str().to_string());
        let version = captures.get(2).map(|m| m.as_str()).unwrap();

        let semver_regex =
            Regex::new(r"^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-.]+))?(?:\+([0-9A-Za-z-.]+))?$")?;

        if let Some(captures) = semver_regex.captures(version) {
            Ok((
                Some(PackageVersion {
                    complete: version.to_string(),
                    major: Some(captures[1].parse()?),
                    minor: Some(captures[2].parse()?),
                    patch: Some(captures[3].parse()?),
                    prerelease: captures.get(4).map(|m| m.as_str().to_string()),
                    build: captures.get(5).map(|m| m.as_str().to_string()),
                }),
                constraint,
            ))
        } else {
            let partial_version_regex = Regex::new(r"^(\d+)(?:\.(\d+))?(?:\.(\d+))?$")?;

            if let Some(captures) = partial_version_regex.captures(version) {
                Ok((
                    Some(PackageVersion {
                        complete: version.to_string(),
                        major: captures.get(1).map(|m| m.as_str().parse().unwrap()),
                        minor: captures.get(2).map(|m| m.as_str().parse().unwrap()),
                        patch: captures.get(3).map(|m| m.as_str().parse().unwrap()),
                        prerelease: None,
                        build: None,
                    }),
                    constraint,
                ))
            } else {
                Ok((None, Some(version.to_string())))
            }
        }
    }
}