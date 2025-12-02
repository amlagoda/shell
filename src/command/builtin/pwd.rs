use crate::command::CommandResult;
use std::env::current_dir;
use std::io::Error;

pub fn run_command() -> Result<CommandResult, Error> {
    let err = Error::other("path is invalid");

    let path = current_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| err)?;

    Ok(CommandResult::new(None, Some(path)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() -> Result<(), Error> {
        assert_eq!(get_current_dir(), run_command()?.output().unwrap());

        Ok(())
    }

    fn get_current_dir() -> String {
        // does not end with a slash
        current_dir().unwrap().to_str().unwrap().to_string()
    }
}
