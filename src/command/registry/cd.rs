use crate::command::NewLine;
use crate::io::Stdio;
use std::env::{home_dir, set_current_dir};
use std::fs::read_dir;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, path: Option<&str>, new_line: &NewLine) -> Result<(), Error> {
    let mut path = path.unwrap_or("~").to_string();

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
        let prefix = if new_line.is_stderr() { "\r\n" } else { "" };
        let msg = format!("{}cd: {}: No such file or directory", prefix, path);

        write!(stdio.stderr(), "{}", msg)?;
        stdio.stderr().flush()?;

        return Ok(());
    }

    set_current_dir(path.as_str())?;

    Ok(())
}

// I'm not testing the command cd because
// it affects the global state
