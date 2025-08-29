use client::versions::RequestPackage;

pub fn parse_package_str(package: String) -> RequestPackage {
    if package.starts_with('@') {
        if let Some(pos) = package.rfind('@') {
            if pos == 0 {
                return RequestPackage { name: package, version: None };
            } else {
                let name = &package[..pos];
                let version = &package[pos + 1..];
                return RequestPackage {
                    name: name.to_string(),
                    version: Some(version.to_string()),
                };
            }
        } else {
            return RequestPackage { name: package, version: None };
        }
    }

    let parts: Vec<&str> = package.splitn(2, '@').collect();
    let name = parts[0].to_string();
    let version = if parts.len() > 1 { Some(parts[1].to_string()) } else { None };

    RequestPackage { name, version }
}
