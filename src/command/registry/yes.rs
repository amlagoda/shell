use crate::command::fmt::NewLine;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, newline: &NewLine) -> Result<(), Error> {
    let stdout = stdio.stdout();
    let mut is_first = true;

    loop {
        let msg = if is_first {
            is_first = false;
            format!("{}y\n", newline.stdout_start()) // infinite output, so at the end \n
        } else {
            "y\n".to_string() // infinite output, so at the end \n
        };

        stdout.write_all(msg.as_bytes())?;
        stdout.flush()?;
    }
}
