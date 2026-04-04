use crate::command::get_command_list;
use crate::core::run;
use crate::env::{get_args, get_history_log_path, split_env_path};
use crate::history::{download as download_log, upload as upload_log, Log};
use crate::io::Stdio;
use crate::keyboard::handle_key;
use crate::parser::parse;
use crossterm::cursor::MoveLeft;
use crossterm::event::{read, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::fs::File;
use std::io::{stderr, stdin, stdout, Error, Write};
use std::os::fd::{AsRawFd, FromRawFd};

mod command;
mod complete;
mod core;
mod env;
mod fs;
mod history;
mod io;
mod keyboard;
mod parser;
mod process;
mod structure;

fn main() -> Result<(), Error> {
    let mut stdio = unsafe {
        Stdio::new(
            File::from_raw_fd(stdin().as_raw_fd()),
            File::from_raw_fd(stdout().as_raw_fd()),
            File::from_raw_fd(stderr().as_raw_fd()),
        )
    };

    let mut log = Log::new();
    let path = split_env_path()?;
    let bin_paths = path.iter().map(|r| r.as_str()).collect();
    let args = get_args();

    if let Some(file_path) = get_history_log_path() {
        download_log(&mut log, file_path.as_str())?;
    }

    if args.is_empty() {
        run_interactive(&mut stdio, &mut log, &bin_paths)
    } else {
        run_command(args.join(" "), &mut stdio, &mut log, &bin_paths)
    }
}

fn run_interactive(stdio: &mut Stdio, log: &mut Log, bin_paths: &Vec<&str>) -> Result<(), Error> {
    let mut input = String::new();
    let mut previous_key: Option<KeyEvent> = None;
    let mut has_user_typing = false;

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
        let commands = r.iter().map(|r| r.as_str()).collect();

        let handled_key = handle_key(
            input.as_str(),
            &key.unwrap(),
            &previous_key.as_ref(),
            &commands,
            bin_paths,
            log,
            has_user_typing,
        );

        let mut is_exit = handled_key.is_exit();
        previous_key = Some(key.unwrap());
        input = handled_key.input().to_string();
        has_user_typing = handled_key.has_user_typing();

        if let Some(len) = handled_key.get_backspace_len() {
            execute!(
                stdio.stdout(),
                MoveLeft(len as u16),
                Clear(ClearType::UntilNewLine)
            )?;
        }

        if let Some(r) = handled_key.get_to_print() {
            stdio.stdout().flush()?;
            write!(stdio.stdout(), "{}", r)?;
            stdio.stdout().flush()?;
        }

        if let Some(r) = handled_key.get_hint() {
            write!(stdio.stdout(), "\r\n{}\r\n$ {}", r, input)?;
            stdio.stdout().flush()?;
        }

        if handled_key.is_enter() {
            if let Some(parseds) = parse(input.as_str())? {
                disable_raw_mode()?;
                let parseds = parseds.iter().collect();
                let output_starts_newline = true;
                is_exit = run(&parseds, stdio, log, bin_paths, output_starts_newline)?;
                enable_raw_mode()?;
            }

            input.clear();
            log.reset();

            if !is_exit {
                write!(stdio.stdout(), "\r\n$ ")?;
                stdio.stdout().flush()?;
            }
        }

        if is_exit {
            if let Some(file_path) = get_history_log_path() {
                upload_log(log, file_path.as_str(), false)?;
            }
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}

fn run_command(
    input: String,
    stdio: &mut Stdio,
    log: &mut Log,
    bin_paths: &Vec<&str>,
) -> Result<(), Error> {
    if let Some(parseds) = parse(input.as_str())? {
        let parseds = parseds.iter().collect();
        let output_starts_newline = false;
        run(&parseds, stdio, log, bin_paths, output_starts_newline)?;
    }

    Ok(())
}
