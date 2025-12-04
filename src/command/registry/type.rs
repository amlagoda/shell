use crate::command::registry::Builtin;
use crate::command::CommandResult;
use crate::fs::search_executable_file_in_paths;
use std::io::Error;

pub fn run_command(command: &str, bin_paths: &Vec<&str>) -> Result<CommandResult, Error> {
    if Builtin::to_builtin(command).is_some() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_run_command() -> Result<(), Error> {
        let path = get_fixture_dir();
        let paths = vec![path.as_str()];

        let r = run_command("not_exe", &paths)?;
        assert_eq!("not_exe: not found", r.error().unwrap());

        let r = run_command("type", &paths)?;
        assert_eq!("type is a shell builtin", r.output().unwrap());

        let r = run_command("exe", &paths)?;
        assert_eq!(format!("exe is {}/exe", path), r.output().unwrap());

        Ok(())
    }

    fn get_fixture_dir() -> String {
        // ends with a slash
        format!("{}/test/fixture/command/registry/type", get_current_dir())
    }

    fn get_current_dir() -> String {
        // does not end with a slash
        current_dir().unwrap().to_str().unwrap().to_string()
    }
}
