use crate::command::fmt::NewLine;
use crate::io::Stdio;
use crate::state::Storage;
use std::io::{Error, Write};

pub fn run_command(stdio: &mut Stdio, storage: &Storage, newline: &NewLine) -> Result<(), Error> {
    if let Some(commands) = storage.get() {
        let len = commands.len();

        for (mut number, command) in commands.iter().enumerate() {
            number += 1;
            let mut to_print = format!("    {}  {}", number, command);

            if number == 1 {
                to_print = format!("{}{}", newline.stdout_start(), to_print);
            }

            if number == len {
                to_print = format!("{}{}", to_print, newline.stdout_end());
            } else {
                to_print = format!("{}\n", to_print);
            }

            write!(stdio.stdout(), "{}", to_print)?;
            stdio.stdout().flush()?;
        }
    }

    Ok(())
}
