use std::io::{Error, Stdout, Write};

pub fn run_command(mut stdout: Stdout, args: Option<&Vec<&str>>) -> Result<(), Error> {
    let join = args.unwrap_or(&vec![" "]).to_vec().join(" ").to_string();

    write!(stdout, "{}", join)?;
    stdout.flush()?;

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
