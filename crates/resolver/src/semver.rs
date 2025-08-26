use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct SemVer {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

impl SemVer {
    pub fn parse(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        let (main, pre) = if let Some(dash_pos) = s.find('-') {
            let (main_part, pre_part) = s.split_at(dash_pos);
            (main_part, Some(pre_part[1..].to_string()))
        } else {
            (s, None)
        };

        let mut parts = main.splitn(3, '.');
        let major = parts.next()?.parse().ok()?;
        let minor = parts.next()?.parse().ok()?;
        let patch = parts.next()?.parse().ok()?;

        if parts.next().is_some() {
            return None;
        }

        Some(Self { major, minor, patch, pre })
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
            .then_with(|| match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            })
    }
}

fn semver_satisfies(version: &SemVer, range: &str) -> bool {
    let range = range.trim();

    if range == "latest" {
        return true;
    }

    match range.chars().next() {
        Some('^') => {
            if let Some(min) = SemVer::parse(&range[1..]) {
                return *version >= min && version.major == min.major;
            }
        }
        Some('~') => {
            if let Some(min) = SemVer::parse(&range[1..]) {
                return *version >= min && version.major == min.major && version.minor == min.minor;
            }
        }
        Some('>') => {
            if let Some(stripped) = range.strip_prefix(">=") {
                if let Some(min) = SemVer::parse(stripped) {
                    return *version >= min;
                }
            } else if let Some(min) = SemVer::parse(&range[1..]) {
                return *version > min;
            }
        }
        Some('<') => {
            if let Some(stripped) = range.strip_prefix("<=") {
                if let Some(max) = SemVer::parse(stripped) {
                    return *version <= max;
                }
            } else if let Some(max) = SemVer::parse(&range[1..]) {
                return *version < max;
            }
        }
        _ => {
            if let Some(exact) = SemVer::parse(range) {
                return *version == exact;
            }
        }
    }

    false
}

fn satisfies_composite(version: &SemVer, composite: &str) -> bool {
    let composite = composite.trim();

    let allow_prerelease = composite.contains('-');
    if version.pre.is_some() && !allow_prerelease {
        return false;
    }

    composite.split("||").any(|range| semver_satisfies(version, range.trim()))
}

pub fn select_version(range: &str, available: Vec<&str>) -> Option<String> {
    let mut compatible = Vec::with_capacity(available.len().min(32));

    for version_str in available {
        if let Some(version) = SemVer::parse(version_str) {
            if satisfies_composite(&version, range) {
                compatible.push(version);
            }
        }
    }

    if compatible.is_empty() {
        return None;
    }

    compatible.sort_unstable();
    let best_version = compatible.into_iter().last()?;

    Some(match &best_version.pre {
        Some(pre) => {
            format!("{}.{}.{}-{}", best_version.major, best_version.minor, best_version.patch, pre)
        }
        None => format!("{}.{}.{}", best_version.major, best_version.minor, best_version.patch),
    })
}
