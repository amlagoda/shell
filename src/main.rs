use crate::command::get_command_list;
use crate::core::run;
use crate::env::{get_args, split_env_path};
use crate::io::Stdio;
use crate::keyboard::handle_key;
use crate::parser::parse;
// use crate::process::{kill_group_childs, pid};
use crossterm::cursor::MoveLeft;
use crossterm::event::{read, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::fs::File;
use std::io::{stderr, stdin, stdout, Error, Write};
use std::os::fd::{AsRawFd, FromRawFd};

mod command;
mod core;
mod env;
mod fs;
mod io;
mod keyboard;
mod parser;
mod process;

fn main() -> Result<(), Error> {
    // unsafe {
    //     libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    //     signal(SIGCHLD, SIG_IGN);
    //     libc::signal(libc::SIGINT, libc::SIG_IGN);
    // }

    let mut stdio = unsafe {
        Stdio::new(
            File::from_raw_fd(stdin().as_raw_fd()),
            File::from_raw_fd(stdout().as_raw_fd()),
            File::from_raw_fd(stderr().as_raw_fd()),
        )
    };

    let path = split_env_path()?;
    let bin_paths = path.iter().map(|r| r.as_str()).collect::<Vec<&str>>();
    let args = get_args();

    if args.is_empty() {
        run_interactive(&mut stdio, &bin_paths)
    } else {
        run_command(args.join(" "), &mut stdio, &bin_paths)
    }
}

fn run_interactive(stdio: &mut Stdio, bin_paths: &Vec<&str>) -> Result<(), Error> {
    let mut input = String::new();
    let mut previous_key: Option<KeyEvent> = None;

    enable_raw_mode()?;
    write!(stdio.stdout(), "\r$ ")?;
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

        // if is_exit {
        // disable_raw_mode()?;
        // kill_group_childs(pid())?;
        // enable_raw_mode()?;
        // }

        if is_backspace {
            execute!(stdio.stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine))?;
        }

        if let Some(r) = to_print {
            stdio.stdout().flush()?;
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
                let parseds = parseds.iter().map(|parsed| parsed).collect();
                let output_starts_newline = true;
                is_exit = run(&parseds, stdio, &bin_paths, output_starts_newline)?;
                enable_raw_mode()?;
            }

            input.clear();

            if !is_exit {
                write!(stdio.stdout(), "\r\n$ ")?;
                stdio.stdout().flush()?;
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

fn run_command(input: String, stdio: &mut Stdio, bin_paths: &Vec<&str>) -> Result<(), Error> {
    if let Some(parseds) = parse(input.as_str())? {
        // parseds технически могут быть пустыми?
        let parseds = parseds.iter().map(|parsed| parsed).collect();
        let output_starts_newline = false;
        run(&parseds, stdio, &bin_paths, output_starts_newline)?;
        println!(""); // %
    }

    Ok(())
}
