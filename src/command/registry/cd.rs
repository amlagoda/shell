use crate::command::Stdio;
use std::env::{home_dir, set_current_dir};
use std::fs::read_dir;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, path: Option<&str>) -> Result<(), Error> {
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
        let msg = format!("cd: {}: No such file or directory", path);
        let stderr = stdio.stderr();

        write!(stderr, "\r\n{}", msg)?;
        stderr.flush()?;

        return Ok(());
    }

    set_current_dir(path.as_str())?;

    Ok(())
}

// I'm not testing the command cd because
// it affects the global state
