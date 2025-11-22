mod builtin;
mod external;

use crate::command::builtin::{builtin_to_string, run_builtin, to_builtin, Builtin};
use crate::command::external::{is_external, run_external};
use crate::fs::write_to_file;
use crate::parser::Parsed;
use std::io::Error;

pub fn builtin_list() -> Vec<String> {
    vec![
        builtin_to_string(&Builtin::Type),
        builtin_to_string(&Builtin::Echo),
        builtin_to_string(&Builtin::Pwd),
        builtin_to_string(&Builtin::Cd),
        builtin_to_string(&Builtin::Exit),
    ]
}

pub fn run_commands(parseds: Vec<Parsed>, bin_paths: &Vec<&str>) -> Result<CommandResult, Error> {
    let mut result = CommandResult::new(Some(": not found".to_string()), None);
    let mut stdin: Option<String> = None;

    for parsed in parseds {
        let args = parsed.args().unwrap_or(vec![]);

        let temp = run_command(parsed.command(), &args, bin_paths, stdin)?;
        stdin = None;

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
            let mut output = vec![];

            if let Some(out) = temp.output() {
                output.push(out.to_string());
            }

            if !pipeline.is_stdout() && temp.error().is_some() {
                output.push(temp.error().unwrap().to_string());
            }

            if !output.is_empty() {
                stdin = Some(output.join(" "));
            }
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
    stdin: Option<String>,
) -> Result<CommandResult, Error> {
    if let Some(builtin) = to_builtin(command) {
        run_builtin(&builtin, args, bin_paths)
    } else if is_external(command, bin_paths) {
        run_external(command, args, stdin)
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
        self.error.as_deref()
    }

    pub fn output(&self) -> Option<&str> {
        self.output.as_deref()
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

        let r = run_command("type", &vec!["type"], &paths, None)?;
        assert_eq!("type is a shell builtin", r.output().unwrap());

        let r = run_command("ls", &vec!["h&6#"], &paths, None)?;
        assert_eq!("ls: h&6#: No such file or directory", r.error().unwrap());

        Ok(())
    }
}
