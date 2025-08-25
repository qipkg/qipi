use crate::{Command, utils::parse_package_str};
use async_trait::async_trait;

use clap::{ArgGroup, Args};

use store::Store;
use utils::logger::*;

use std::fs::read_dir;

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
        let mut term = Term::default();
        let mut theme = MinimalTheme::default();

        if self.clear {
            let mut p = Promptuity::new(&mut term, &mut theme);
            p.begin().unwrap();
            let confirmed = p
                .prompt(
                    Confirm::new("Are you sure you want to remove all packages?")
                        .with_default(true),
                )
                .unwrap_or(false);
            p.finish().unwrap();

            if confirmed {
                store.clear();
            }
        }

        if !self.remove.is_empty() {
            let packages: Vec<_> =
                self.remove.iter().map(|p| parse_package_str(p.to_owned())).collect();

            let mut with_version = vec![];
            let mut without_version = vec![];

            for pkg in packages {
                if pkg.version.is_some() {
                    with_version.push(pkg);
                } else {
                    without_version.push(pkg);
                }
            }

            if !with_version.is_empty() {
                let mut p = Promptuity::new(&mut term, &mut theme);
                p.begin().unwrap();
                let confirmed = p
                    .prompt(
                        Confirm::new("Are you sure you want to remove the selected packages?")
                            .with_default(true),
                    )
                    .unwrap_or(false);
                p.finish().unwrap();

                if confirmed {
                    for pkg in with_version {
                        store.remove(pkg.name, pkg.version.unwrap());
                    }
                }
            }

            if !without_version.is_empty() {
                let mut options: Vec<MultiSelectOption<String>> = vec![];
                for pkg in &without_version {
                    let prefix = format!("{}@", pkg.name);
                    let versions: Vec<_> = read_dir(&store.store_path)
                        .unwrap()
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let fname = entry.file_name().to_string_lossy().to_string();
                            if fname.starts_with(&prefix) {
                                Some(MultiSelectOption::new(fname.clone(), fname))
                            } else {
                                None
                            }
                        })
                        .collect();
                    options.extend(versions);
                }

                if !options.is_empty() {
                    let mut p = Promptuity::new(&mut term, &mut theme);
                    p.begin().unwrap();
                    let selected: Vec<String> = p
                        .prompt(
                            MultiSelect::new("Select package versions to remove", options).as_mut(),
                        )
                        .unwrap_or_default();
                    p.finish().unwrap();

                    for sel in selected {
                        if let Some(pos) = sel.rfind('@') {
                            let name = &sel[..pos];
                            let version = &sel[pos + 1..];
                            store.remove(name.to_string(), version.to_string());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
