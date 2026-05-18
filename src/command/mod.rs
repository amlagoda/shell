mod registry;

use self::registry::cd as run_command_cd;
use self::registry::echo as run_command_echo;
use self::registry::history as run_command_history;
use self::registry::pwd as run_command_pwd;
use self::registry::r#type as run_command_type;
use self::registry::yes as run_command_yes;
use self::registry::Builtin;
use crate::fmt::NewLine;
use crate::history::History;
use crate::Stdio;
use std::io::Error;

pub fn to_command(name: &str) -> Option<Builtin> {
    Builtin::to_builtin(name)
}

pub fn run_command(
    command: &Builtin,
    args: Option<&Vec<&str>>,
    stdio: &mut Stdio,
    history: &mut History,
    newline: &NewLine,
    bin_paths: Option<&Vec<&str>>,
) -> Result<(), Error> {
    match command {
        Builtin::Cd => run_command_cd(stdio, newline, args),
        Builtin::Echo => run_command_echo(stdio, newline, args),
        Builtin::History => run_command_history(stdio, newline, history, args),
        Builtin::Exit => Ok(()),
        Builtin::Pwd => run_command_pwd(stdio, newline),
        Builtin::Type => run_command_type(stdio, newline, bin_paths, args),
        Builtin::Yes => run_command_yes(stdio, newline),
    }
}

pub fn get_command_list() -> Vec<String> {
    Builtin::list_as_strings()
}
