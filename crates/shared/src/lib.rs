use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Package {
    pub author: Option<String>,
    pub name: String,
    pub version: Option<Semver>,
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
