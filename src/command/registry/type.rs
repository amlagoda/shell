use crate::command::fmt::NewLine;
use crate::command::registry::Builtin;
use crate::fs::find_file;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    newline: &NewLine,
    bin_paths: Option<&Vec<&str>>,
    args: Option<&Vec<&str>>,
) -> Result<(), Error> {
    let command = get_command(args);
    let mut msg = format!("{}: not found", command);
    let mut to_stderr = true;

    if Builtin::to_builtin(command.as_str()).is_some() {
        msg = format!("{} is a shell builtin", command);
        to_stderr = false;
    } else if let Some(bin_paths) = bin_paths {
        let only_executable = true;

        if let Some(path) = find_file(command.as_str(), only_executable, bin_paths) {
            msg = format!("{} is {}", command, path);
            to_stderr = false;
        }
    }

    if to_stderr {
        write!(
            stdio.stderr(),
            "{}{}{}",
            newline.stderr_start(),
            msg,
            newline.stderr_end()
        )?;

        stdio.stderr().flush()?;
    } else {
        write!(
            stdio.stdout(),
            "{}{}{}",
            newline.stdout_start(),
            msg,
            newline.stdout_end()
        )?;

        stdio.stdout().flush()?;
    }

    Ok(())
}

fn get_command(args: Option<&Vec<&str>>) -> String {
    if let Some(args) = args {
        args.first().unwrap().to_string()
    } else {
        "".to_string()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env::current_dir;

//     #[test]
//     fn test_run_command() -> Result<(), Error> {
//         let path = get_fixture_dir();
//         let paths = vec![path.as_str()];

//         let r = run_command("not_exe", &paths)?;
//         assert_eq!("not_exe: not found", r.error().unwrap());

//         let r = run_command("type", &paths)?;
//         assert_eq!("type is a shell builtin", r.output().unwrap());

//         let r = run_command("exe", &paths)?;
//         assert_eq!(format!("exe is {}/exe", path), r.output().unwrap());

//         Ok(())
//     }

//     fn get_fixture_dir() -> String {
//         // ends with a slash
//         format!("{}/test/fixture/command/registry/type", get_current_dir())
//     }

//     fn get_current_dir() -> String {
//         // does not end with a slash
//         current_dir().unwrap().to_str().unwrap().to_string()
//     }
// }
