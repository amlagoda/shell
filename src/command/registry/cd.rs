use crate::fmt::NewLine;
use crate::io::Stdio;
use std::env::{home_dir, set_current_dir};
use std::fs::read_dir;
use std::io::{Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    newline: &NewLine,
    args: Option<&Vec<&str>>,
) -> Result<(), Error> {
    let mut path = get_path(args);

    if path == "~" {
        let err1 = Error::other("HOME is not set");
        let err2 = Error::other("HOME is invalid");

        path = home_dir()
            .ok_or(err1)?
            .into_os_string()
            .into_string()
            .map_err(|_| err2)?;
    }

    if read_dir(path.as_str()).is_err() {
        let msg = format!(
            "{}cd: {}: No such file or directory{}",
            newline.stderr_start(),
            path,
            newline.stderr_end()
        );

        write!(stdio.stderr(), "{}", msg)?;
        stdio.stderr().flush()?;

        return Ok(());
    }

    set_current_dir(path.as_str())?;

    Ok(())
}

fn get_path(args: Option<&Vec<&str>>) -> String {
    if let Some(args) = args {
        args.first().unwrap().to_string()
    } else {
        "~".to_string()
    }
}

// I'm not testing the command cd because
// it affects the global state
