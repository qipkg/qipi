use regex::Regex;
use shared::{Package, Semver};

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

fn parse_version(version: &str) -> Option<Semver> {
    match version.parse() {
        Ok(semver) => Some(semver),
        Err(_) => None,
    }
}
