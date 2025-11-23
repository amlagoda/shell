use crate::command::{builtin_list, run_commands};
use crate::env::split_env_path;
use crate::keyboard::handle_key;
use crate::parser::parse;
use crossterm::cursor::MoveLeft;
use crossterm::event::{read, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Error, Write};

mod command;
mod env;
mod fs;
mod keyboard;
mod parser;

fn main() -> Result<(), Error> {
    let mut stdout = stdout();
    let mut input = String::new();
    let mut previous_key: Option<KeyEvent> = None;

    let r = split_env_path();

    if let Err(e) = r {
        return Err(Error::other(e.to_string()));
    }

    let r = r.unwrap();
    let bin_paths = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

    enable_raw_mode()?;
    write!(stdout, "$ ")?;
    stdout.flush()?;

    loop {
        let r = read()?;
        let key = r.as_key_event();

        if key.is_none() {
            continue;
        }

        let r = builtin_list();
        let commands = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

        let (i, to_print, hint, is_enter, mut is_exit, is_backspace) =
            handle_key(input, &key.unwrap(), &previous_key, &commands, &bin_paths);
        previous_key = Some(key.unwrap());
        input = i;

        if is_backspace {
            execute!(stdout, MoveLeft(1), Clear(ClearType::UntilNewLine))?;
        }

        if let Some(r) = to_print {
            write!(stdout, "{}", r)?;
            stdout.flush()?;
        }

        if let Some(r) = hint {
            write!(stdout, "\r\n{}\r\n$ {}", r, input)?;
            stdout.flush()?;
        }

        if is_enter {
            let mut output: Option<String> = None;
            let mut error = Some(String::from(": not found"));

            if let Some(parseds) = parse(input.as_str())? {
                let result = run_commands(parseds, &bin_paths)?;

                error = result.error().map(|r| r.to_string());
                output = result.output().map(|r| r.to_string());

                if result.is_exit() {
                    is_exit = true;
                }
            }

            input.clear();

            let mut to_print = error
                .unwrap_or("".to_string())
                .split("\n")
                .filter(|r| !r.is_empty())
                .map(|r| r.to_string())
                .collect::<Vec<String>>();

            to_print.append(
                &mut output
                    .unwrap_or("".to_string())
                    .split("\n")
                    .filter(|r| !r.is_empty())
                    .map(|r| r.to_string())
                    .collect::<Vec<String>>(),
            );

            for r in to_print.iter() {
                write!(stdout, "\r\n{}", r)?;
                stdout.flush()?;
            }

            if !is_exit {
                write!(stdout, "\r\n$ ")?;
                stdout.flush()?;
            }
        }

        if is_exit {
            break;
        }
    }

    disable_raw_mode()?;
    println!(""); // %

    Ok(())
}
