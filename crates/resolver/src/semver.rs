use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
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

    if range == "latest" || range == "*" {
        return true;
    }

    if let Some(exact) = SemVer::parse(range) {
        return version == &exact;
    }

    if let Some(rest) = range.strip_prefix('^') {
        if let Some(min) = SemVer::parse(rest) {
            return version >= &min && version.major == min.major;
        }
    }

    if let Some(rest) = range.strip_prefix('~') {
        if let Some(min) = SemVer::parse(rest) {
            return version >= &min && version.major == min.major && version.minor == min.minor;
        }
    }

    if let Some(stripped) = range.strip_prefix(">=") {
        if let Some(min) = SemVer::parse(stripped) {
            return version >= &min;
        }
    }

    if let Some(stripped) = range.strip_prefix('>') {
        if let Some(min) = SemVer::parse(stripped) {
            return version > &min;
        }
    }

    if let Some(stripped) = range.strip_prefix("<=") {
        if let Some(max) = SemVer::parse(stripped) {
            return version <= &max;
        }
    }

    if let Some(stripped) = range.strip_prefix('<') {
        if let Some(max) = SemVer::parse(stripped) {
            return version < &max;
        }
    }

    false
}

pub fn select_version(version_req: &str, available: Vec<&str>) -> Option<String> {
    let mut parsed: Vec<SemVer> = available.iter().filter_map(|v| SemVer::parse(v)).collect();

    parsed.sort_by(|a, b| b.cmp(a));

    for ver in parsed {
        if semver_satisfies(&ver, version_req) {
            let base = format!("{}.{}.{}", ver.major, ver.minor, ver.patch);
            if let Some(pre) = &ver.pre {
                return Some(format!("{base}-{pre}"));
            } else {
                return Some(base);
            }
        }
    }

    None
}
