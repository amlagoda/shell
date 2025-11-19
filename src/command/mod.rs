mod builtin;
mod external;

use crate::command::builtin::{run_builtin, to_builtin};
use crate::command::external::{is_external, run_external};
use std::io::Error;

fn run_command(command: &str, args: &Vec<&str>, bin_paths: &Vec<&str>) -> Result<Command, Error> {
    if let Some(builtin) = to_builtin(command) {
        run_builtin(&builtin, args, bin_paths)
    } else if is_external(command, bin_paths) {
        run_external(command, args)
    } else {
        let msg = format!("{}: not found", command);
        Ok(Command::new(Some(msg), None))
    }
}

struct Command {
    error: Option<String>,
    output: Option<String>,
    is_exit: bool,
}

impl Command {
    fn new(error: Option<String>, output: Option<String>) -> Command {
        Command {
            error,
            output,
            is_exit: false,
        }
    }

    fn new_exit() -> Command {
        Command {
            error: None,
            output: None,
            is_exit: true,
        }
    }

    fn error(&self) -> Option<&str> {
        self.error.as_ref().map(|r| r.as_str())
    }

    fn output(&self) -> Option<&str> {
        self.output.as_ref().map(|r| r.as_str())
    }

    fn is_exit(&self) -> bool {
        self.is_exit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::split_env_path;

    #[test]
    fn test_run_command() -> Result<(), Error> {
        let r = split_env_path().unwrap();
        let paths = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

        let r = run_command("type", &vec!["type"], &paths)?;
        assert_eq!("type is a shell builtin", r.output().unwrap());

        let r = run_command("ls", &vec!["h&6#"], &paths)?;
        assert_eq!("ls: h&6#: No such file or directory", r.error().unwrap());

        Ok(())
    }
}
