use crate::command::NewLine;
use crate::io::Stdio;
use std::env::current_dir;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, new_line: &NewLine) -> Result<(), Error> {
    let err = Error::other("path is invalid");

    let path = current_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| err)?;

    let prefix = if new_line.is_stdout() { "\r\n" } else { "" };
    write!(stdio.stdout(), "{}{}", prefix, path)?;
    stdio.stdout().flush()?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_run_command() -> Result<(), Error> {
//         assert_eq!(get_current_dir(), run_command()?.output().unwrap());

//         Ok(())
//     }

//     fn get_current_dir() -> String {
//         // does not end with a slash
//         current_dir().unwrap().to_str().unwrap().to_string()
//     }
// }
