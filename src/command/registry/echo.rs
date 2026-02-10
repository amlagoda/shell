use crate::command::NewLine;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    args: Option<&Vec<&str>>,
    new_line: &NewLine,
) -> Result<(), Error> {
    let join = args.unwrap_or(&vec![" "]).to_vec().join(" ").to_string();
    let prefix = if new_line.is_stdout() { "\r\n" } else { "" };

    write!(stdio.stdout(), "{}{}", prefix, join)?;
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
