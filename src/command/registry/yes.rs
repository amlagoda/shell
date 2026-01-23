use crate::command::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio) -> Result<(), Error> {
    let stdout = stdio.stdout();

    loop {
        write!(stdout, "\r\n{}", "y")?;
        stdout.flush()?;
    }
}
