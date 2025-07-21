use crate::register_commands;

pub(crate) trait Command {
    fn run(&self) -> Result<(), ()>;
}

register_commands!();
