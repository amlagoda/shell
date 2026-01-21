use crate::command::{get_command_list, Stdio};
use crate::core::run;
use crate::env::split_env_path;
use crate::keyboard::handle_key;
use crate::parser::parse;
use crossterm::cursor::MoveLeft;
use crossterm::event::{read, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use libc::{signal, SIGCHLD, SIG_IGN};
use std::io::{stderr, stdin, stdout, Error, Write};
use std::process::exit;

mod command;
mod core;
mod env;
mod fs;
mod keyboard;
mod parser;
mod process;

fn main() -> Result<(), Error> {
    // unsafe {
    //     libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    //     signal(SIGCHLD, SIG_IGN);
    // }

    let mut stdout1 = stdout();
    let mut input = String::new();
    let mut previous_key: Option<KeyEvent> = None;

    let r = split_env_path();

    if let Err(e) = r {
        return Err(Error::other(e.to_string()));
    }

    let r = r.unwrap();
    let bin_paths = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

    enable_raw_mode()?;
    write!(stdout1, "$ ")?;
    stdout1.flush()?;

    loop {
        let r = read()?;
        let key = r.as_key_event();

        if key.is_none() {
            continue;
        }

        let r = get_command_list();
        let commands = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

        let (i, to_print, hint, is_enter, mut is_exit, is_backspace) =
            handle_key(input, &key.unwrap(), &previous_key, &commands, &bin_paths);
        previous_key = Some(key.unwrap());
        input = i;

        if is_backspace {
            execute!(stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine))?;
            // execute!(stdout1, MoveLeft(1), Clear(ClearType::UntilNewLine))?;
        }

        if let Some(r) = to_print {
            let mut stdout1 = stdout();
            write!(stdout1, "{}", r)?;
            stdout1.flush()?;
        }

        if let Some(r) = hint {
            let mut stdout1 = stdout();
            write!(stdout1, "\r\n{}\r\n$ {}", r, input)?;
            stdout1.flush()?;
        }

        if is_enter {
            // let mut output: Option<String> = None;
            // let mut error = Some(String::from(": not found"));

            if let Some(parseds) = parse(input.as_str())? {
                let stdout1 = stdout();
                let mut stdio = Stdio::new(stdin(), stdout1, stderr());
                let _ = stdio.stdin();
                let exit = run(&mut stdio, parseds, &bin_paths)?;
                // let result = run(parseds, &bin_paths, stdout);

                // if let Err(_) = result {
                // exit(1);
                // }

                // stdout = result.unwrap();

                // stdout = run(parseds, &bin_paths, stdout)?;

                // error = result.error().map(|r| r.to_string());
                // output = result.output().map(|r| r.to_string());

                if exit.yes() {
                    is_exit = true;
                }
            }

            input.clear();

            // let mut to_print = error
            //     .unwrap_or("".to_string())
            //     .split("\n")
            //     .filter(|r| !r.is_empty())
            //     .map(|r| r.to_string())
            //     .collect::<Vec<String>>();

            // to_print.append(
            //     &mut output
            //         .unwrap_or("".to_string())
            //         .split("\n")
            //         .filter(|r| !r.is_empty())
            //         .map(|r| r.to_string())
            //         .collect::<Vec<String>>(),
            // );

            // for r in to_print.iter() {
            //     write!(stdout, "\r\n{}", r)?;
            //     stdout.flush()?;
            // }

            // if !is_exit {
            //     write!(stdout, "\r\n$ ")?;
            //     stdout.flush()?;
            // }
            let mut stdout1 = stdout();
            write!(stdout1, "\n\r$ ")?;
            stdout1.flush()?;
        }

        if is_exit {
            break;
        }
    }

    disable_raw_mode()?;
    println!(""); // %

    Ok(())
}

pub enum Exit {
    Yes,
    No,
}

impl Exit {
    pub fn yes(&self) -> bool {
        match self {
            Exit::Yes => true,
            Exit::No => false,
        }
    }
}
