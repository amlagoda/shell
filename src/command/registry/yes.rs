use crate::command::fmt::NewLine;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, newline: &NewLine) -> Result<(), Error> {
    let stdout = stdio.stdout();
    let mut is_first = true;

    loop {
        if is_first {
            is_first = false;

            write!(stdout, "{}{}\r\n", newline.stdout_start(), "y")?; // infinite output, so at the end \r\n
            stdout.flush()?;
        } else {
            write!(stdout, "{}\r\n", "y")?; // infinite output, so at the end \r\n
            stdout.flush()?;
        }
    }
}
