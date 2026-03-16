use crate::command::fmt::NewLine;
use crate::io::Stdio;
use std::env::current_dir;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, newline: &NewLine) -> Result<(), Error> {
    let err = Error::other("path is invalid");

    let path = current_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| err)?;

    write!(
        stdio.stdout(),
        "{}{}{}",
        newline.stdout_start(),
        path,
        newline.stdout_end()
    )?;

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
