use crate::command::registry::Builtin;
use crate::command::Stdio;
use crate::fs::search_executable_file_in_paths;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, command: &str, bin_paths: &Vec<&str>) -> Result<(), Error> {
    let mut msg = format!("type: {}: not found", command);
    let mut to_stderr = true;

    if Builtin::to_builtin(command).is_some() {
        msg = format!("{} is a shell builtin", command);
        to_stderr = false;
    } else if let Some(path) = search_executable_file_in_paths(command, bin_paths) {
        msg = format!("{} is {}", command, path);
        to_stderr = false;
    }

    if to_stderr {
        let stderr = stdio.stderr();
        write!(stderr, "\r\n{}", msg)?;
        stderr.flush()?;
    } else {
        let stdout = stdio.stdout();
        write!(stdout, "\r\n{}", msg)?;
        stdout.flush()?;
    }

    return Ok(());
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
