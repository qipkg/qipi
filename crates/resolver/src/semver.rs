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
        let mut parts = s.splitn(2, '-');

        let main = parts.next()?;
        let pre = parts.next().map(|s| s.to_string());

        let nums: Vec<&str> = main.split('.').collect();

        if nums.len() != 3 {
            return None;
        }

        Some(Self {
            major: nums[0].parse().ok()?,
            minor: nums[1].parse().ok()?,
            patch: nums[2].parse().ok()?,
            pre,
        })
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
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
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

    if let Some(stripped) = range.strip_prefix('^') {
        if let Some(min) = SemVer::parse(stripped) {
            return *version >= min && version.major == min.major;
        }
    }

    if let Some(stripped) = range.strip_prefix('~') {
        if let Some(min) = SemVer::parse(stripped) {
            return *version >= min && version.major == min.major && version.minor == min.minor;
        }
    }

    if let Some(stripped) = range.strip_prefix(">=") {
        if let Some(min) = SemVer::parse(stripped) {
            return *version >= min;
        }
    }

    if let Some(stripped) = range.strip_prefix("<=") {
        if let Some(max) = SemVer::parse(stripped) {
            return *version <= max;
        }
    }

    if let Some(stripped) = range.strip_prefix('>') {
        if let Some(min) = SemVer::parse(stripped) {
            return *version > min;
        }
    }

    if let Some(stripped) = range.strip_prefix('<') {
        if let Some(max) = SemVer::parse(stripped) {
            return *version < max;
        }
    }

    if let Some(exact) = SemVer::parse(range) {
        return *version == exact;
    }

    false
}

fn satisfies_composite(version: &SemVer, composite: &str) -> bool {
    let composite = composite.trim();
    let allow_prerelease = composite.split("||").any(|r| r.contains('-'));

    if version.pre.is_some() && !allow_prerelease {
        return false;
    }

    composite.split("||").any(|r| semver_satisfies(version, r))
}

pub fn select_version(range: &str, available: Vec<&str>) -> Option<String> {
    let mut compatible: Vec<SemVer> = vec![];

    for v in available {
        if let Some(ver) = SemVer::parse(v) {
            if satisfies_composite(&ver, range) {
                compatible.push(ver);
            }
        }
    }

    compatible.sort();

    compatible.last().map(|v| {
        if let Some(pre) = &v.pre {
            format!("{}.{}.{}-{}", v.major, v.minor, v.patch, pre)
        } else {
            format!("{}.{}.{}", v.major, v.minor, v.patch)
        }
    })
}
