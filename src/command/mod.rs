mod registry;

use crate::command::registry::cd::run_command as run_command_cd;
use crate::command::registry::echo::run_command as run_command_echo;
use crate::command::registry::pwd::run_command as run_command_pwd;
use crate::command::registry::r#type::run_command as run_command_type;
use crate::command::registry::Builtin;
use std::io::{Error, Stderr, Stdout};

pub fn to_builtin(command: &str) -> Option<Builtin> {
    Builtin::to_builtin(command)
}

pub fn run_command(
    stdio: Stdio,
    command: &Builtin,
    args: Option<&Vec<&str>>,
    bin_paths: Option<&Vec<&str>>,
) -> Result<Exit, Error> {
    let mut exit = Exit::No;

    match command {
        Builtin::Cd => {
            let default: Vec<&str> = vec![];
            run_command_cd(stdio, args.unwrap_or(&default).first().copied())
        }
        Builtin::Echo => run_command_echo(stdio, args),
        Builtin::Exit => {
            exit = Exit::Yes;
            let args = vec!["^C"];
            run_command_echo(stdio, Some(&args))
        }
        Builtin::Pwd => run_command_pwd(stdio),
        Builtin::Type => {
            let default = vec![""];
            let command = args.unwrap_or(&default).first().unwrap();
            let default: Vec<&str> = vec![];
            run_command_type(stdio, command, bin_paths.unwrap_or(&default))
        }
    };

    Ok(exit)
}

pub fn get_command_list() -> Vec<String> {
    Builtin::list_as_strings()
}

pub struct Stdio {
    stdout: Stdout,
    stderr: Stderr,
}

impl Stdio {
    pub fn new(stdout: Stdout, stderr: Stderr) -> Stdio {
        Stdio { stdout, stderr }
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }

    pub fn stderr(&mut self) -> &mut Stderr {
        &mut self.stderr
    }
}

pub enum Exit {
    Yes,
    No,
}

impl Exit {
    pub fn yes(&self) -> bool {
        match self {
            Exit::Yes => true,
            Exit::No => false,
        }
    }
}
