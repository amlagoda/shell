use crate::command::CommandResult;
use crate::fs::search_executable_file_in_paths;
use std::io::{Error, Read};
use std::process::{Child, Command, Stdio};

pub fn is_external(command: &str, bin_paths: &Vec<&str>) -> bool {
    search_executable_file_in_paths(command, bin_paths).is_some()
}

pub fn run_external(command: &str, args: &Vec<&str>) -> Result<CommandResult, Error> {
    let mut process = Command::new(command)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    process.wait()?;

    to_result(process)
}

fn to_result(process: Child) -> Result<CommandResult, Error> {
    let mut stderr = None;
    let mut stdout = None;

    if let Some(mut r) = process.stderr {
        let mut output = String::new();
        r.read_to_string(&mut output)?;

        if !output.is_empty() {
            stderr = Some(output.trim().to_string());
        }
    }

    if let Some(mut r) = process.stdout {
        let mut output = String::new();
        r.read_to_string(&mut output)?;

        if !output.is_empty() {
            stdout = Some(output.trim().to_string());
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
        let r = run_external("ls", &vec!["not_exists", path.as_str()])?;

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
