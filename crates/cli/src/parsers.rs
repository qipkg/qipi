use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Package {
    pub author: Option<String>,
    pub name: String,
    pub version: Option<Semver>,
}

pub fn parse_package(package: String) -> Result<Package, String> {
    let re = Regex::new(r"^(?:(?P<author>@?[^@/]+)\/)?(?P<name>[^@]+)(?:@(?P<version>[^\s]+))?$")
        .unwrap();

    if let Some(captures) = re.captures(&package) {
        let author = captures
            .name("author")
            .map(|m| m.as_str().trim_start_matches('@').to_string());
        let name = captures["name"].to_string();
        let version = captures
            .name("version")
            .map(|m| m.as_str().to_string())
            .unwrap_or("".to_string());
        let version_parsed = parse_version(&version);

        let package = Package {
            author,
            name,
            version: version_parsed,
        };
        Ok(package)
    } else {
        Err(format!("failed to parse package: {}", package))
    }
}

#[derive(Debug)]
pub struct Semver {
    pub operator: Option<String>,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub complete: String,
}

impl FromStr for Semver {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"^(?P<operator>[<>=^~]*)(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)$")
                .unwrap();

        if let Some(captures) = re.captures(s) {
            let operator = captures.name("operator").map(|m| m.as_str().to_string());
            let major: u32 = captures["major"].parse().unwrap();
            let minor: u32 = captures["minor"].parse().unwrap();
            let patch: u32 = captures["patch"].parse().unwrap();

            Ok(Semver {
                operator,
                major,
                minor,
                patch,
                complete: s.to_string(),
            })
        } else {
            Err(())
        }
    }
}

fn parse_version(version: &str) -> Option<Semver> {
    match version.parse() {
        Ok(semver) => Some(semver),
        Err(_) => None,
    }
}
