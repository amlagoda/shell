use crate::command::command::command;
use crate::env::env::split_env_path;
use crate::fs::fs::write_to_file;
use crate::keyboard::keyboard::handle_key;
use crate::parser::parser::parse;
use crossterm::{
    cursor::MoveLeft,
    event::read,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Error, ErrorKind, Write};
use std::process::ExitCode;

mod command;
mod env;
mod fs;
mod keyboard;
mod parser;

fn main() -> Result<(), Error> {
    let mut stdout = stdout();
    let mut input = String::new();
    let mut output: Option<String> = None;
    let mut error: Option<String> = None;
    let mut print: Option<String> = None;
    let mut is_exit = false;
    let mut is_enter = false;
    let mut is_backspace = false;

    let r = split_env_path();

    if r.is_err() {
        let msg = r.unwrap_err().to_string();
        let err = Error::new(ErrorKind::Other, msg);
        return Err(err);
    }

    let bin_paths = r.unwrap();

    enable_raw_mode()?;
    write!(stdout, "$ ")?;
    stdout.flush()?;

    loop {
        let r = read()?;
        let key = r.as_key_event();

        if key.is_none() {
            continue;
        }

        (input, print, is_enter, is_exit, is_backspace) = handle_key(input, &key.unwrap());

        if is_backspace {
            execute!(stdout, MoveLeft(1), Clear(ClearType::UntilNewLine))?;
        }

        if print.is_some() {
            write!(stdout, "{}", print.unwrap())?;
            stdout.flush()?;
            print = None;
        }

        if is_enter {
            let (name, args, redirect) = parse(input.as_str());

            if name.is_some() {
                let name = name.unwrap();
                let args = args.iter().map(|r| r.as_str()).collect::<Vec<&str>>();
                let paths = bin_paths.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

                (output, error, is_exit) = command(name.as_str(), &args, &paths);
            } else {
                output = None;
                error = Some(String::from(": not found"));
            }

            input.clear();

            if redirect.is_some() {
                let [flow, mode, path] = redirect.unwrap();
                let mut to_write: Option<String> = None;

                if flow == "1" && output.is_some() {
                    to_write = Some(output.unwrap());
                    output = None;
                }

                if flow == "2" && error.is_some() {
                    to_write = Some(error.unwrap());
                    error = None;
                }

                if to_write.is_some() {
                    let r = format!("{}\n", to_write.unwrap());
                    write_to_file(path.as_str(), r.as_str(), mode == ">>")?;
                }
            }

            let mut to_print = error
                .unwrap_or("".to_string())
                .split("\n")
                .into_iter()
                .filter(|r| r.len() > 0)
                .map(|r| r.to_string())
                .collect::<Vec<String>>();
           
            to_print.append(&mut output
                .unwrap_or("".to_string())
                .split("\n")
                .into_iter()
                .filter(|r| r.len() > 0)
                .map(|r| r.to_string())
                .collect::<Vec<String>>());

            for r in to_print.iter() {
                write!(stdout, "\r\n{}", r)?;
                stdout.flush()?;
            }

            output = None;
            error = None;

            if !is_exit {
                write!(stdout, "\r\n$ ")?;
                stdout.flush()?;
            }

            is_enter = false;
        }

        if is_exit {
            break;
        }
    }

    disable_raw_mode()?;
    println!(""); // %

    Ok(())
}
