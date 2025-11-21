mod builtin;
mod external;

use crate::command::builtin::{run_builtin, to_builtin};
use crate::command::external::{is_external, run_external};
use crate::fs::write_to_file;
use crate::parser::Parsed;
use std::io::Error;

pub fn run_commands(parseds: Vec<Parsed>, bin_paths: &Vec<&str>) -> Result<CommandResult, Error> {
    let mut result = CommandResult::new(Some(": not found".to_string()), None);
    let mut pipeline_args: Vec<String> = vec![];

    for parsed in parseds {
        let mut args = parsed.args().unwrap_or(vec![]);

        if !pipeline_args.is_empty() {
            let mut r = pipeline_args
                .iter()
                .map(|r| r.as_str())
                .collect::<Vec<&str>>();

            args.append(&mut r);
        }

        let temp = run_command(parsed.command(), &args, bin_paths)?;

        if let Some(redirect) = parsed.redirect() {
            let mut to_write = String::new();

            if !redirect.is_stderr() && temp.output().is_some() {
                to_write.push_str(temp.output().unwrap());
            }

            if redirect.is_stderr() && temp.error().is_some() {
                to_write.push_str(temp.error().unwrap());
            }

            if !to_write.is_empty() {
                to_write.push('\n');
                write_to_file(redirect.path(), to_write.as_str(), redirect.is_append())?;
            }
        }

        if temp.is_exit() {
            if parsed.redirect().is_none() || parsed.redirect().unwrap().is_stderr() {
                return Ok(temp);
            }

            return Ok(CommandResult::new(None, None));
        }

        if let Some(pipeline) = parsed.pipeline() {
            let mut args = vec![];

            if let Some(output) = temp.output() {
                args.push(output.to_string());
            }

            if !pipeline.is_stdout() && temp.error().is_some() {
                args.push(temp.error().unwrap().to_string());
            }

            pipeline_args = args;
        }

        if parsed.redirect().is_some() && parsed.pipeline().is_none() {
            if parsed.redirect().unwrap().is_stderr() {
                result = CommandResult::new(None, temp.output);
            } else {
                result = CommandResult::new(temp.error, None);
            }
        } else {
            result = temp;
        }
    }

    Ok(result)
}

fn run_command(
    command: &str,
    args: &Vec<&str>,
    bin_paths: &Vec<&str>,
) -> Result<CommandResult, Error> {
    if let Some(builtin) = to_builtin(command) {
        run_builtin(&builtin, args, bin_paths)
    } else if is_external(command, bin_paths) {
        run_external(command, args)
    } else {
        let msg = format!("{}: not found", command);
        Ok(CommandResult::new(Some(msg), None))
    }
}

pub struct CommandResult {
    error: Option<String>,
    output: Option<String>,
    is_exit: bool,
}

impl CommandResult {
    fn new(error: Option<String>, output: Option<String>) -> CommandResult {
        CommandResult {
            error,
            output,
            is_exit: false,
        }
    }

    fn new_exit(output: Option<String>) -> CommandResult {
        CommandResult {
            error: None,
            output,
            is_exit: true,
        }
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_ref().map(|r| r.as_str())
    }

    pub fn output(&self) -> Option<&str> {
        self.output.as_ref().map(|r| r.as_str())
    }

    pub fn is_exit(&self) -> bool {
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
