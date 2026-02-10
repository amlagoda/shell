use crate::command::get_command_list;
use crate::core::run;
use crate::env::split_env_path;
use crate::io::Stdio;
use crate::keyboard::handle_key;
use crate::parser::parse;
use crossterm::cursor::MoveLeft;
use crossterm::event::{read, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use libc::{signal, SIGCHLD, SIG_IGN};
use std::fs::File;
use std::io::{stderr, stdin, stdout, Error, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::process::exit;

mod command;
mod core;
mod env;
mod fs;
mod io;
mod keyboard;
mod parser;

fn main() -> Result<(), Error> {
    // unsafe {
    //     libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    //     signal(SIGCHLD, SIG_IGN);
    // }

    let mut stdio = unsafe {
        Stdio::new(
            File::from_raw_fd(stdin().as_raw_fd()),
            File::from_raw_fd(stdout().as_raw_fd()),
            File::from_raw_fd(stderr().as_raw_fd()),
        )
    };
    let mut input = String::new();
    let mut previous_key: Option<KeyEvent> = None;

    let path = split_env_path()?;
    let bin_paths = path.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

    enable_raw_mode()?;
    write!(stdio.stdout(), "$ ")?;
    stdio.stdout().flush()?;

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
            execute!(stdio.stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine))?;
        }

        if let Some(r) = to_print {
            write!(stdio.stdout(), "{}", r)?;
            stdio.stdout().flush()?;
        }

        if let Some(r) = hint {
            write!(stdio.stdout(), "\r\n{}\r\n$ {}", r, input)?;
            stdio.stdout().flush()?;
        }

        if is_enter {
            if let Some(parseds) = parse(input.as_str())? {
                disable_raw_mode()?;
                // parseds технически могут быть пустыми?
                run(&parseds, &mut stdio, &bin_paths)?; // печатет если команда не найдена
                enable_raw_mode()?;
            }

            write!(stdio.stdout(), "\n\r$ ")?;
            stdio.stdout().flush()?;
            input.clear();
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
