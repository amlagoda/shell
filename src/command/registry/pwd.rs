use crate::command::{PrintFact, Stdio};
use std::env::current_dir;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio) -> Result<PrintFact, Error> {
    let err = Error::other("path is invalid");

    let path = current_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| err)?;

    write!(stdio.stdout(), "{}", path)?;
    stdio.stdout().flush()?;

    Ok(PrintFact::new(
        true,  /* stdout */
        false, /* stderr */
    ))
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
