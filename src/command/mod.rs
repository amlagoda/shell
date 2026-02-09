mod registry;

use crate::command::registry::cd::run_command as run_command_cd;
use crate::command::registry::echo::run_command as run_command_echo;
use crate::command::registry::pwd::run_command as run_command_pwd;
use crate::command::registry::r#type::run_command as run_command_type;
// use crate::command::registry::tee::run_command as run_command_tee;
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
    args: Option<&Vec<&str>>,
    bin_paths: Option<&Vec<&str>>,
) -> Result<PrintFact, Error> {
    match command {
        Builtin::Cd => {
            let default: Vec<&str> = vec![];
            run_command_cd(stdio, args.unwrap_or(&default).first().copied())
        }
        Builtin::Echo => run_command_echo(stdio, args),
        Builtin::Exit => Ok(PrintFact::new(
            false, /* stdout */
            false, /* stderr */
        )),
        Builtin::Pwd => run_command_pwd(stdio),
        // Builtin::Tee => run_command_tee(),
        Builtin::Type => {
            let default = vec![""];
            let command = args.unwrap_or(&default).first().unwrap();
            let default: Vec<&str> = vec![];
            run_command_type(stdio, command, bin_paths.unwrap_or(&default))
        }
        Builtin::Yes => run_command_yes(stdio),
    }
}

pub fn get_command_list() -> Vec<String> {
    Builtin::list_as_strings()
}

pub struct PrintFact {
    stdout: bool,
    stderr: bool,
}

impl PrintFact {
    pub fn new(stdout: bool, stderr: bool) -> PrintFact {
        PrintFact { stdout, stderr }
    }

    pub fn is_stdout(&self) -> bool {
        self.stdout
    }

    pub fn is_stderr(&self) -> bool {
        self.stderr
    }

    pub fn is_any(&self) -> bool {
        self.stdout || self.stderr
    }
}
