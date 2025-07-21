use crate::register_commands;

pub(crate) trait Command {
    fn run(&self) -> Result<(), ()>;
}

register_commands!(new, init, add, remove, install, uninstall, shell, mount, umount, lock, list);
