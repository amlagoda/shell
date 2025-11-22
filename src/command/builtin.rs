use crate::command::CommandResult;
use crate::fs::search_executable_file_in_paths;
use std::env::{current_dir, home_dir, set_current_dir};
use std::fs::read_dir;
use std::io::Error;

pub fn to_builtin(command: &str) -> Option<Builtin> {
    match command {
        "type" => Some(Builtin::Type),
        "echo" => Some(Builtin::Echo),
        "pwd" => Some(Builtin::Pwd),
        "cd" => Some(Builtin::Cd),
        "exit" => Some(Builtin::Exit),
        _ => None,
    }
}

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
        Builtin::Exit => Ok(CommandResult::new_exit(None)),
    }
}

pub enum Builtin {
    Type,
    Echo,
    Pwd,
    Cd,
    Exit,
}

pub fn builtin_to_string(builtin: &Builtin) -> String {
    match builtin {
        Builtin::Type => String::from("type"),
        Builtin::Echo => String::from("echo"),
        Builtin::Pwd => String::from("pwd"),
        Builtin::Cd => String::from("cd"),
        Builtin::Exit => String::from("exit"),
    }
}

fn run_command_type(command: &str, bin_paths: &Vec<&str>) -> Result<CommandResult, Error> {
    if to_builtin(command).is_some() {
        let msg = format!("{} is a shell builtin", command);
        return Ok(CommandResult::new(None, Some(msg)));
    }

    if let Some(path) = search_executable_file_in_paths(command, bin_paths) {
        let msg = format!("{} is {}", command, path);
        return Ok(CommandResult::new(None, Some(msg)));
    }

    let msg = format!("{}: not found", command);
    Ok(CommandResult::new(Some(msg), None))
}

fn run_command_echo(args: &Vec<&str>) -> Result<CommandResult, Error> {
    let join = args.to_vec().join(" ").to_string();

    Ok(CommandResult::new(None, Some(join)))
}

fn run_command_pwd() -> Result<CommandResult, Error> {
    let err = Error::other("path is invalid");

    let path = current_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| err)?;

    Ok(CommandResult::new(None, Some(path)))
}

fn run_command_cd(path: &str) -> Result<CommandResult, Error> {
    let mut path = path.to_string();

    if path == "~" {
        let err1 = Error::other("HOME is not set");
        let err2 = Error::other("HOME is invalid");

        path = home_dir()
            .ok_or(err1)?
            .into_os_string()
            .into_string()
            .map_err(|_| err2)?;
    }

    if read_dir(path.as_str()).is_err() {
        let msg = format!("cd: {}: No such file or directory", path);
        return Ok(CommandResult::new(Some(msg), None));
    }

    set_current_dir(path.as_str())?;

    Ok(CommandResult::new(None, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command_type() -> Result<(), Error> {
        let path = get_fixture_dir();
        let paths = vec![path.as_str()];

        let r = run_command_type("not_exe", &paths)?;
        assert_eq!("not_exe: not found", r.error().unwrap());

        let r = run_command_type("type", &paths)?;
        assert_eq!("type is a shell builtin", r.output().unwrap());

        let r = run_command_type("exe", &paths)?;
        assert_eq!(format!("exe is {}exe", path), r.output().unwrap());

        Ok(())
    }

    #[test]
    fn test_run_command_echo() -> Result<(), Error> {
        let r = vec!["foo", "bar"];
        assert_eq!("foo bar", run_command_echo(&r)?.output().unwrap());
        Ok(())
    }

    // I'm not testing the command_cd because
    // it affects the global state

    #[test]
    fn test_run_command_pwd() -> Result<(), Error> {
        assert_eq!(get_current_dir(), run_command_pwd()?.output().unwrap());
        Ok(())
    }

    fn get_fixture_dir() -> String {
        // ends with a slash
        format!("{}/test/fixture/command/builtin/", get_current_dir())
    }

    fn get_current_dir() -> String {
        // does not end with a slash
        current_dir().unwrap().to_str().unwrap().to_string()
    }
}
