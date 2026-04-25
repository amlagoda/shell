use crate::command::get_command_list;
use crate::control::{run_command, run_interactive};
use crate::env::{get_args, get_current_dir, get_history_log_path, split_env_path};
use crate::history::{download as download_history_log, History};
use crate::io::Stdio;
use crate::session::State;
use crossterm::event::read as read_key;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{Error, Write};

mod command;
mod complete;
mod control;
mod core;
mod cursor;
mod env;
mod fmt;
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
    let mut history = History::new();

    let bin_paths = split_env_path()?;
    let bin_paths = bin_paths.iter().map(|r| r.as_str()).collect();
    let args = get_args();

    if let Some(path) = get_history_log_path() {
        download_history_log(&mut history, path.as_str())?;
    }

    if args.is_empty() {
        mode_interactive(&mut state, &mut stdio, &mut history, &bin_paths)
    } else {
        let input = args.join(" ");
        state.terminal().input().push_as_system(input.as_str());

        mode_command(&mut state, &mut stdio, &mut history, &bin_paths)
    }
}

fn mode_interactive(
    state: &mut State,
    stdio: &mut Stdio,
    history: &mut History,
    bin_paths: &Vec<&str>,
) -> Result<(), Error> {
    enable_raw_mode()?;
    write!(stdio.stdout(), "\r$ ")?;
    stdio.stdout().flush()?;

    loop {
        let pressed_key = read_key()?.as_key_event();

        if pressed_key.is_none() {
            continue;
        }

        let available_commands = get_command_list();
        let available_commands = available_commands.iter().map(|r| r.as_str()).collect();
        let output_starts_newline = true;

        let is_exit = run_interactive(
            &pressed_key.unwrap(),
            state,
            stdio,
            history,
            &available_commands,
            bin_paths,
            get_current_dir().as_str(),
            output_starts_newline,
        )?;

        state.keyboard().set_previous_key(pressed_key.unwrap());

        if is_exit {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}

fn mode_command(
    state: &mut State,
    stdio: &mut Stdio,
    history: &mut History,
    bin_paths: &Vec<&str>,
) -> Result<(), Error> {
    let output_starts_newline = false;
    run_command(state, stdio, history, bin_paths, output_starts_newline)?;

    Ok(())
}
