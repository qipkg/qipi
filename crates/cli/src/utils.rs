use regex::Regex;

#[derive(Debug)]
pub struct Package {
    author: Option<String>,
    name: String,
    version: Option<String>,
}

pub fn parse_package(package: String) -> Result<Package, String> {
    let re = Regex::new(r"^(?:(?P<author>@?[^@/]+)\/)?(?P<name>[^@]+)(?:@(?P<version>[^\s]+))?$").unwrap();

    if let Some(captures) = re.captures(&package) {
        let author = captures.name("author")
            .map(|m| m.as_str().trim_start_matches('@').to_string());
        let name = captures["name"].to_string();
        let version = captures.name("version").map(|m| m.as_str().to_string());

        let package = Package { author, name, version };
        Ok(package)
    } else {
        Err(format!("failed to parse package: {}", package))
    }
}
