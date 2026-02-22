pub mod fmt;
mod registry;

use crate::command::fmt::NewLine;
use crate::command::registry::cd::run_command as run_command_cd;
use crate::command::registry::echo::run_command as run_command_echo;
use crate::command::registry::pwd::run_command as run_command_pwd;
use crate::command::registry::r#type::run_command as run_command_type;
use crate::command::registry::tee::run_command as run_command_tee;
use crate::command::registry::yes::run_command as run_command_yes;
use crate::command::registry::Builtin;
use crate::Stdio;
use std::io::Error;

pub fn to_command(name: &str) -> Option<Builtin> {
    Builtin::to_builtin(name)
}

pub fn run_command(
    command: &Builtin,
    stdio: &mut Stdio,
    newline: &NewLine,
    args: Option<&Vec<&str>>,
    bin_paths: Option<&Vec<&str>>,
) -> Result<(), Error> {
    match command {
        Builtin::Cd => {
            let default: Vec<&str> = vec![];
            run_command_cd(stdio, newline, args.unwrap_or(&default).first().copied())
        }
        Builtin::Echo => run_command_echo(stdio, newline, args),
        Builtin::Exit => Ok(()),
        Builtin::Pwd => run_command_pwd(stdio, newline),
        Builtin::Tee => {
            let default: Vec<&str> = vec![];
            let mut args = args.unwrap_or(&default).into_iter();
            let path = args.next().unwrap_or(&"");
            let append = args.next().unwrap_or(&"");
            run_command_tee(stdio, path, append == &"-a")
        }
        Builtin::Type => {
            let default = vec![""];
            let command = args.unwrap_or(&default).first().unwrap();
            let default: Vec<&str> = vec![];
            run_command_type(stdio, newline, command, bin_paths.unwrap_or(&default))
        }
        Builtin::Yes => run_command_yes(stdio, newline),
    }
}

pub fn get_command_list() -> Vec<String> {
    Builtin::list_as_strings()
}
