use crate::command::fmt::NewLine;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, newline: &NewLine) -> Result<(), Error> {
    let stdout = stdio.stdout();
    let mut is_first = true;

    loop {
        if is_first {
            is_first = false;

            writeln!(stdout, "{}y", newline.stdout_start())?; // infinite output, so at the end \n
            stdout.flush()?;
        } else {
            writeln!(stdout, "y")?; // infinite output, so at the end \n
            stdout.flush()?;
        }
    }
}
