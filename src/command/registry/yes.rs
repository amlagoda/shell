use crate::command::PrintFact;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, start_newline: bool) -> Result<PrintFact, Error> {
    let prefix = if start_newline { "\r\n" } else { "" };
    let stdout = stdio.stdout();
    let mut is_start = true;

    loop {
        if is_start {
            write!(stdout, "{}{}\r\n", prefix, "y")?; // infinite output, so at the end \r\n
            is_start = false;
        }

        write!(stdout, "{}\r\n", "y")?; // infinite output, so at the end \r\n
        stdout.flush()?;
    }
}
