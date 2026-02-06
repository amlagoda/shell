use crate::command::PrintFact;
use crate::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, args: Option<&Vec<&str>>) -> Result<PrintFact, Error> {
    let join = args.unwrap_or(&vec![" "]).to_vec().join(" ").to_string();

    write!(stdio.stdout(), "{}", join)?;
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
//         let r = vec!["foo", "bar"];
//         assert_eq!("foo bar", run_command(Some(&r))?.output().unwrap());

//         Ok(())
//     }
// }
