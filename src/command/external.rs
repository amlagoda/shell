use crate::command::CommandResult;
use crate::fs::search_executable_file_in_paths;
use std::io::{Error, Write};
use std::process::{Command as Process, Output, Stdio};
use std::thread::spawn;

pub fn is_external(command: &str, bin_paths: &Vec<&str>) -> bool {
    search_executable_file_in_paths(command, bin_paths).is_some()
}

pub fn run_external(
    command: &str,
    args: &Vec<&str>,
    stdin: Option<String>,
) -> Result<CommandResult, Error> {
    let mut process = Process::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = stdin {
        let err = Error::other("stdin opening error");
        let mut std = process.stdin.take().ok_or(err)?;

        spawn(move || -> Result<(), Error> {
            let err = Error::other("stdin write error");
            std.write_all(stdin.as_bytes()).map_err(|_| err)
        })
        .join()
        .unwrap()?;
    }

    to_result(process.wait_with_output()?)
}

fn to_result(output: Output) -> Result<CommandResult, Error> {
    let mut stdout: Option<String> = None;
    let mut stderr: Option<String> = None;

    if !output.stdout.is_empty() {
        let err = Error::other("stdout reading error");
        let r = String::from_utf8(output.stdout).map_err(|_| err)?;

        if !r.trim().is_empty() {
            stdout = Some(r.trim().to_string());
        }
    }

    if !output.stderr.is_empty() {
        let err = Error::other("stderr reading error");
        let r = String::from_utf8(output.stderr).map_err(|_| err)?;

        if !r.trim().is_empty() {
            stderr = Some(r.trim().to_string());
        }
    }

    Ok(CommandResult::new(stderr, stdout))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::split_env_path;
    use std::env::current_dir;

    #[test]
    fn test_is_external() {
        let r = split_env_path().unwrap();
        let paths = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

        assert!(is_external("ls", &paths));
    }

    #[test]
    fn test_run_external() -> Result<(), Error> {
        let path = get_fixture_dir();
        let r = run_external("ls", &vec!["not_exists", path.as_str()], None)?;

        assert_eq!(
            "ls: not_exists: No such file or directory",
            r.error().unwrap()
        );

        assert_eq!(format!("{}:\nfile", path), r.output().unwrap());

        Ok(())
    }

    fn get_fixture_dir() -> String {
        // ends with a slash
        format!("{}/test/fixture/command/external/", get_current_dir())
    }

    fn get_current_dir() -> String {
        // does not end with a slash
        current_dir().unwrap().to_str().unwrap().to_string()
    }
}
