use crate::Command;
use clap::Args;

use std::{
    fs::{create_dir, write},
    path::Path,
};
use utils::logger::*;

#[derive(Debug, Args)]
pub(crate) struct NewCommand {
    path: String,

    #[clap(short = 'a', long, default_value_t = false)]
    auto_shell: bool,
}

impl Command for NewCommand {
    fn run(&self) -> Result<(), ()> {
        let path = Path::new(&self.path);

        if path.exists() {
            error(format!("Directory '{0}' already exists", self.path), false);
            return Err(());
        }

        create_dir(path).expect("error creating folder in 'new' command");

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unnamed");

        let content = format!(
            r###"{{
  "name": "{name}",
  "version": "1.0.0",
  "dependencies": {{}}
}}"###,
        );

        let package_json_path = path.join("package.json");
        write(package_json_path, content).expect("error writing package.json in 'new' command");

        info("package.json created", false);

        let package_lock_path = path.join("package.lock");
        write(package_lock_path, b"").expect("error creating package.lock in 'new' command");

        info("package.lock created", false);

        let envrc_path = path.join(".envrc");
        if self.auto_shell {
            let content = r#"# Qipi shell auto-loader (avoid running 'qp shell' every time)
# Install direnv (https://direnv.net) to use this feature
# Run 'direnv allow' to enable automatic environment loading

if command -v qp &> /dev/null; then
    eval "$(qp shell --export)"
fi
"#;
            write(envrc_path, content).expect("error creating .envrc in 'new' command");

            info(".envrc created", false);
            sub_info("Run 'direnv allow' to enable automatic Qipi shell loading", false);
        }

        success(format!("Project '{0}' created", self.path), false);

        Ok(())
    }
}
