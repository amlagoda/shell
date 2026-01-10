use crate::command_old::CommandResult;
use crate::fs::search_executable_file_in_paths;
use std::io::{Error, Write};
use std::process::{Command as Process, Output, Stdio};
use std::thread::spawn;

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
