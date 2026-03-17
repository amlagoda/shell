use crate::command::fmt::NewLine;
use crate::io::Stdio;
use crate::storage::History;
use std::io::{BufWriter, Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    newline: &NewLine,
    history: &History,
    args: Option<&Vec<&str>>,
) -> Result<(), Error> {
    if let Some(mut commands) = history.all() {
        let (limit, file_path) = get_args(args);
        let mut buffer = BufWriter::with_capacity(4096, stdio.stdout());
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

            buffer.write_all(to_print.as_bytes())?;
            num += 1;
        }

        buffer.flush()?;
    }

    Ok(())
}

fn get_args(args: Option<&Vec<&str>>) -> (Option<usize>, Option<String>) {
    let mut limit = None;
    let mut file_path = None;

    if let Some(args) = args {
        let mut iter = args.into_iter();

        while let Some(arg) = iter.next() {
            if let Ok(parsed) = arg.parse::<usize>() {
                limit = Some(parsed);
            } else if arg == &"-r" {
                if let Some(path) = iter.next() {
                    file_path = Some(path.to_string());
                }
            }
        }
    }

    (limit, file_path)
}
