use client::versions::RequestPackage;

pub fn parse_package_str(package: String) -> RequestPackage {
    let (name, version) = if package.starts_with('@') {
        if let Some(pos) = package.rfind('@') {
            let name = &package[..pos];
            let version = &package[pos + 1..];
            (name.to_string(), Some(version.to_string()))
        } else {
            (package.clone(), None)
        }
    } else {
        let parts: Vec<&str> = package.splitn(2, '@').collect();
        let name = parts[0].to_string();
        let version = if parts.len() > 1 { Some(parts[1].to_string()) } else { None };
        (name, version)
    };

    RequestPackage { name, version }
}
