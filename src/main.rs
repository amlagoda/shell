use crate::command::get_command_list;
use crate::control::run;
use crate::env::{get_args, get_current_dir, get_history_log_path, split_env_path};
use crate::history::{download as download_log, Log};
use crate::io::Stdio;
use crate::session::State;
use crossterm::event::read;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::fs::File;
use std::io::{stderr, stdin, stdout, Error, Write};
use std::os::fd::{AsRawFd, FromRawFd};

mod command;
mod complete;
mod control;
mod core;
mod cursor;
mod env;
mod fs;
mod history;
mod io;
mod keyboard;
mod parser;
mod process;
mod session;
mod structure;

fn main() -> Result<(), Error> {
    let mut stdio = Stdio::new();
    let mut state = State::new();
    let mut log = Log::new();
    let path = split_env_path()?;
    let bin_paths = path.iter().map(|r| r.as_str()).collect();
    let args = get_args();

    if let Some(file_path) = get_history_log_path() {
        download_log(&mut log, file_path.as_str())?;
    }

    if args.is_empty() {
        run_interactive(&mut state, &mut stdio, &mut log, &bin_paths)
    } else {
        run_command(args.join(" "), &mut stdio, &mut log, &bin_paths)
    }
}

fn run_interactive(
    state: &mut State,
    stdio: &mut Stdio,
    log: &mut Log,
    bin_paths: &Vec<&str>,
) -> Result<(), Error> {
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

        let is_exit = run(
            &key.unwrap(),
            state,
            stdio,
            log,
            &commands,
            bin_paths,
            get_current_dir().as_str(),
        )?;

        state.keyboard().set_previous_key(key.unwrap());

        if is_exit {
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
    // if let Some(parseds) = parse(input.as_str())? {
    //     let parseds = parseds.iter().collect();
    //     let output_starts_newline = false;
    //     run(&parseds, stdio, log, bin_paths, output_starts_newline)?;
    // }

    Ok(())
}
