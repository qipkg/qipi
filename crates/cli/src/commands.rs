use crate::parser::Package;

pub fn install_command() {
    todo!("Install dependencies");
}

pub fn update_command() {
    todo!("Update dependencies");
}

pub fn run_command(_script: String) {
    todo!("Run a script");
}

pub fn list_command() {
    todo!("List dependencies");
}

pub fn add_command(package: String, _dev: bool, _peer: bool, _optional: bool) {
    let package = Package::parse(&package);
    let client = client::create_client();

    match package {
        Ok(package) => {
            println!("{:?}", package);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

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

pub fn init_command(_name: Option<String>) {
    todo!("Create a new project");
}
