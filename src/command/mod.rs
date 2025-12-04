mod registry;

use crate::command::registry::cd::run_command as run_command_cd;
use crate::command::registry::echo::run_command as run_command_echo;
use crate::command::registry::exit::run_command as run_command_exit;
use crate::command::registry::pwd::run_command as run_command_pwd;
use crate::command::registry::r#type::run_command as run_command_type;
use crate::command::registry::Builtin;
use std::io::Error;

pub fn to_builtin(command: &str) -> Option<Builtin> {
    Builtin::to_builtin(command)
}

pub fn run_command(
    command: &Builtin,
    args: Option<&Vec<&str>>,
    bin_paths: Option<&Vec<&str>>,
) -> Result<CommandResult, Error> {
    match command {
        Builtin::Cd => {
            let args = args.ok_or(&vec![] as &[&str]).unwrap();
            run_command_cd(args.first().copied())
        }
        Builtin::Echo => run_command_echo(args),
        Builtin::Exit => run_command_exit(),
        Builtin::Pwd => run_command_pwd(),
        Builtin::Type => {
            let command = args.ok_or(&vec![""]).unwrap().first().unwrap();
            let bin_paths = bin_paths.ok_or(&vec![] as &[&str]).unwrap();
            run_command_type(command, bin_paths)
        }
    }
}

pub fn get_command_list() -> Vec<String> {
    Builtin::list_as_strings()
}

pub struct CommandResult {
    error: Option<String>,
    output: Option<String>,
    is_exit: bool,
}

impl CommandResult {
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn output(&self) -> Option<&str> {
        self.output.as_deref()
    }

    pub fn is_exit(&self) -> bool {
        self.is_exit
    }

    pub fn new(error: Option<String>, output: Option<String>) -> CommandResult {
        CommandResult {
            error,
            output,
            is_exit: false,
        }
    }

    pub fn new_exit(output: Option<String>) -> CommandResult {
        CommandResult {
            error: None,
            output,
            is_exit: true,
        }
    }
}
