use crate::command::{NewLine, PrintFact};
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, new_line: &NewLine) -> Result<PrintFact, Error> {
    let prefix = if new_line.is_stdout() { "\r\n" } else { "" };
    let stdout = stdio.stdout();
    let mut is_first = true;

    loop {
        if is_first {
            write!(stdout, "{}{}\r\n", prefix, "y")?; // infinite output, so at the end \r\n
            is_first = false;
        } else {
            write!(stdout, "{}\r\n", "y")?; // infinite output, so at the end \r\n
            stdout.flush()?;
        }
    }
}
