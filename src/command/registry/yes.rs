use crate::command::registry::PrintFact;
use crate::command::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio) -> Result<PrintFact, Error> {
    let stdout = stdio.stdout();

    loop {
        write!(stdout, "{}\r\n", "y")?; // infinite output, so at the end \r\n
        stdout.flush()?;
    }
}
