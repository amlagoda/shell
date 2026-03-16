use crate::command::fmt::NewLine;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    newline: &NewLine,
    args: Option<&Vec<&str>>,
) -> Result<(), Error> {
    let join = args.map_or_else(|| " ".to_string(), |args| args.join(" ").to_string());

    write!(
        stdio.stdout(),
        "{}{}{}",
        newline.stdout_start(),
        join,
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
//         let r = vec!["foo", "bar"];
//         assert_eq!("foo bar", run_command(Some(&r))?.output().unwrap());

//         Ok(())
//     }
// }
