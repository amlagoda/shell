use crate::command::CommandResult;
use std::io::Error;

pub fn run_command() -> Result<CommandResult, Error> {
    Ok(CommandResult::new_exit(None))
}
