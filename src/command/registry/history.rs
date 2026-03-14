use crate::command::fmt::NewLine;
use crate::io::Stdio;
use crate::state::Storage;
use std::io::{Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    storage: &Storage,
    newline: &NewLine,
    limit: Option<usize>,
) -> Result<(), Error> {
    if let Some(mut commands) = storage.all() {
        let len_start = commands.len();
        let mut len_limit = len_start;
        let mut num = 1;

        if let Some(limit) = limit {
            commands = commands.into_iter().rev().take(limit).rev().collect();
            len_limit = commands.len();

            if limit > 1 && limit < len_start {
                num = len_start - limit;
            }
        }

        for (mut iter, command) in commands.iter().enumerate() {
            iter += 1;
            let mut to_print = format!("    {}  {}", num, command);

            if iter == 1 {
                to_print = format!("{}{}", newline.stdout_start(), to_print);
            }

            if iter == len_limit {
                to_print = format!("{}{}", to_print, newline.stdout_end());
            } else {
                to_print = format!("{}\n", to_print);
            }

            write!(stdio.stdout(), "{}", to_print)?;
            stdio.stdout().flush()?;
            num += 1;
        }
    }

    Ok(())
}
