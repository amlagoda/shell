mod cd;
mod echo;
mod exit;
mod pwd;
mod r#type;

use crate::command::builtin::cd::run_command as run_command_cd;
use crate::command::builtin::echo::run_command as run_command_echo;
use crate::command::builtin::exit::run_command as run_command_exit;
use crate::command::builtin::pwd::run_command as run_command_pwd;
use crate::command::builtin::r#type::run_command as run_command_type;
use crate::command::CommandResult;
use std::io::Error;

pub fn run_builtin(
    command: &Builtin,
    args: &Vec<&str>,
    bin_paths: &Vec<&str>,
) -> Result<CommandResult, Error> {
    match command {
        Builtin::Type => run_command_type(args.first().unwrap_or(&""), bin_paths),
        Builtin::Echo => run_command_echo(args),
        Builtin::Pwd => run_command_pwd(),
        Builtin::Cd => run_command_cd(args.first().unwrap_or(&"")),
        Builtin::Exit => run_command_exit(),
    }
}

pub enum Builtin {
    Type,
    Echo,
    Pwd,
    Cd,
    Exit,
}

impl Builtin {
    pub fn to_builtin(command: &str) -> Option<Builtin> {
        Builtin::list()
            .into_iter()
            .find(|r| r.to_string() == command)
    }

    pub fn list_as_strings() -> Vec<String> {
        Builtin::list()
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
    }

    fn list() -> Vec<Builtin> {
        vec![
            Builtin::Type,
            Builtin::Echo,
            Builtin::Pwd,
            Builtin::Cd,
            Builtin::Exit,
        ]
    }
}

impl ToString for Builtin {
    fn to_string(&self) -> String {
        match self {
            Builtin::Type => String::from("type"),
            Builtin::Echo => String::from("echo"),
            Builtin::Pwd => String::from("pwd"),
            Builtin::Cd => String::from("cd"),
            Builtin::Exit => String::from("exit"),
        }
    }
}
