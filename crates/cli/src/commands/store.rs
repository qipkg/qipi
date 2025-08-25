use crate::{Command, utils::parse_package_str};
use async_trait::async_trait;

use clap::{ArgGroup, Args};

use store::Store;

#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args(&["remove", "clear"])
))]
pub(crate) struct StoreCommand {
    #[clap(short, long, num_args = 1.., value_name = "PACKAGE")]
    remove: Vec<String>,

    #[clap(short, long)]
    clear: bool,
}

#[async_trait]
impl Command for StoreCommand {
    async fn run(&self) -> Result<(), ()> {
        let store = Store::new();

        if self.clear {
            // todo: before deleting, confirm with a prompt y/n
            store.clear();
        }

        if !self.remove.is_empty() {
            for package in &self.remove {
                let req_package = parse_package_str(package.to_owned());

                if let Some(version) = req_package.version {
                    // todo: before deleting, confirm with a prompt y/n

                    store.remove(req_package.name, version);
                } else {
                    // if there are multiple packages in $HOME/.qipi/store/name@whatever, i.e., multiple versions of the same package
                    todo!("list them all to select which ones to delete");
                }
            }
        }

        Ok(())
    }
}
