use crate::parser::Package;

pub fn remove_command(package: String) {
    let package = Package::parse(&package);

    match package {
        Ok(package) => {
            println!("{:?}", package);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
