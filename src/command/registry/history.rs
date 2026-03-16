use crate::command::fmt::NewLine;
use crate::io::Stdio;
use crate::storage::History;
use std::io::{Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    history: &History,
    newline: &NewLine,
    limit: Option<usize>,
    file_path: Option<&str>,
) -> Result<(), Error> {
    if let Some(mut commands) = history.all() {
        let buffer_capacity = 10;
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
        let len_start = commands.len();
        let mut len_limit = len_start;
        let mut num = 1;

        if let Some(limit) = limit {
            commands = commands.into_iter().rev().take(limit).rev().collect();
            len_limit = commands.len();

            if limit > 0 && limit < len_start {
                num = len_start - (limit - 1);
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

            if (buffer.len() + to_print.as_bytes().len()) <= buffer_capacity {
                buffer.write_all(to_print.as_bytes())?;
            } else {
                stdio.stdout().write_all(&buffer)?;
                stdio.stdout().flush()?;

                buffer = Vec::with_capacity(buffer_capacity);
                buffer.write_all(to_print.as_bytes())?;
            }

            num += 1;
        }

        if !buffer.is_empty() {
            stdio.stdout().write_all(&buffer)?;
            stdio.stdout().flush()?;
        }
    }

    Ok(())
}
